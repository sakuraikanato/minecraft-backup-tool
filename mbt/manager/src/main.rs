// use std::process::{ 
// 	ChildStdin, 
// 	ChildStdout, 
// 	ChildStderr, 
// 	Command, 
// 	Stdio 
// };

// 非同期用
use tokio::{
	io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader}, 
	process::{
		ChildStderr, ChildStdin, ChildStdout, Command
	},
	try_join
};

use protocol::{types::Request};
mod servers;
use servers::*;

use std::{collections::HashMap, fmt::Debug, io::{ErrorKind::ConnectionAborted, Write, stdin}, path::Path, process::Stdio};
use tokio_util::codec::{FramedRead, LinesCodec};
use futures_util::stream::StreamExt;
use std::sync::atomic::{AtomicI32, Ordering};
use std::path::PathBuf;

use interprocess::local_socket::{
    tokio::{prelude::*, Stream},
    GenericNamespaced, GenericFilePath, ListenerOptions,
};



static CURRENT_NUM: AtomicI32 = AtomicI32::new(-1);


fn start_subprocess(ip: &str) -> Result<StdIo, std::io::Error> {
	let mut child = Command::new("ping")
		.arg(ip)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("failed to execute process");

	let stdin = child.stdin.take().unwrap();
	let stdout = child.stdout.take().unwrap();
	let stderr = child.stderr.take().unwrap();
	let stdio: StdIo = StdIo {stdin, stdout, stderr};
	Ok(stdio)
}

async fn read_line(stdout: ChildStdout) {
	let mut reader = FramedRead::new(stdout, LinesCodec::new());
	while let Some(line) = reader.next().await {
		println!("{:?}", line);
	}
}

async fn send<T: Debug>(sender: &Stream, object: T) {
	println!("{:?}", object); //テスト
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), tokio::io::Error> {
	let mut server_list = Manager {servers: HashMap::<i32, Server>::new()};

	let name = if GenericNamespaced::is_supported() {
        "mms.sock".to_ns_name::<GenericNamespaced>()?
    } else {
        "/tmp/mms.sock".to_fs_name::<GenericFilePath>()?
    };

	let listener = match ListenerOptions::new().name(name).create_tokio() {
		Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
			eprintln!("他プロセスが使用中です");
			return Err(e.into());
		}
		x => x?
	};

	loop {
		let conn = match listener.accept().await {
			Ok(c) => c,
			Err(e) => {
				eprintln!("接続の確立に失敗しました: {e}");
				return Err(e.into());
			}
		};

		let mut buff = String::new();

		let mut recver = BufReader::new(&conn);
		let mut sender = &conn;

		let _ = recver.read_line(&mut buff).await;
		let request = match serde_json::from_str::<Request>(&buff) {
			Ok(j) => j,
			Err(e) => {
				let message = format!("リクエスト形式が不正です: {e}\n");
				let _ = sender.write_all(&message.as_bytes()).await;
				continue
			}
		};
		let _ = match request {
			Request::List {} => {
				let servers = server_list.list();
				send(sender, servers)
			}
			Request::Info { server_id } => { 
				let server: &Server = match server_list.info(server_id) {
					Some(v) => v,
					None => {
						send(sender, "IDが存在しません");
						continue;
					}
				};
				send(sender, server);
				continue;
			},
			Request::Create { name, description, path } => {
				match server_list.create(&name, &description, &path) {
					Ok(v) => v,
					Err(_) => {
						send(sender, "IDが存在しません");
						continue;
					}
				};
				continue;
			},
			Request::Update { server_id, name, description, path } => {
				match server_list.update(server_id, &name, &description, &path){
					Ok(v) => v,
					Err(_) => {
						send(sender, "IDが存在しません");
						continue;
					}
				};
				continue;
			},
			Request::Delete { server_id } => {
				match server_list.delete(server_id) {
					Ok(v) => v,
					Err(_) => {
						send(sender, "IDが存在しません");
						continue;
					}
				};
				continue;
			},
			_ => {
				println!("不正、または未実装のコマンドです");
				continue;
			}
		};


		println!("get message: {buff}");
	}
	Ok(())
}
