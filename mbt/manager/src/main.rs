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

use protocol::servers::*;

use std::{io::{Write, stdin}, path::Path, process::Stdio};
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

// 改行せずにコンソールに出力
fn printfl(str: &str) {
	print!("{}", str);
	std::io::stdout().flush().unwrap();
}

fn create_chat(list: &mut Manager) {
	let mut name = String::new();
	let mut desc = String::new();
	#[allow(unused_assignments)]
	let mut opt_desc = Option::Some(String::new());
	let mut path = String::new();
	while name.trim().is_empty() {
		printfl("サーバー名を入力してください： ");
		match stdin().read_line(&mut name) {
			Ok(_n) => {},
			Err(err) => {println!("{}", err); return}
		};
	}
	printfl("サーバー説明を入力してください： ");
	match stdin().read_line(&mut desc) {
		Ok(_n) => { opt_desc = Some(desc) },
		Err(_err) => { opt_desc = None } 
	};
	while path.trim().is_empty() {
		printfl("サーバーpathを入力してください： ");
		match stdin().read_line(&mut path) {
			Ok(_n) => {},
			Err(err) => {println!("{}", err); return}
		};
	}

	list.create(&name, &opt_desc, Path::new(&path));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), tokio::io::Error> {
	// let mut server_list = Manager {servers: Vec::<Server>::new()};
	// let mut input = BufReader::new(io::stdin()).lines();
	
	// printfl("mbt $ ");
	// while let Some(line) = input.next_line().await? {
	// 	let commands: Vec<&str> = line.split(" ").collect();
	// 	if CURRENT_NUM.load(Ordering::Relaxed) == -1 {
	// 		match commands[0] {
	// 			"exit" => { return Ok(()) }
	// 			"info" => { println!("{}", server_list.get(commands[1].parse::<usize>().unwrap()).unwrap()); } // メモ：将来的にここはDBのidから検索させたい
	// 			"list" => { println!("{}", server_list.get_all()) }, // テスト出力
	// 			"create" => {create_chat(&mut server_list);},
	// 			_ => {println!("errer: unknown command {}", commands[0])}
	// 		}
	// 	} else {
	// 		if commands[0].starts_with(":") {

	// 		} else {

	// 		}
	// 	}
	// 	printfl("mbt $ ");
	// }
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

		let rec = recver.read_line(&mut buff);
		let send = sender.write_all(b"test\n");

		try_join!(send, rec);

		println!("success: {buff}");
	}
	Ok(())
}
