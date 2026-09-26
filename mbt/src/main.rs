// use std::process::{ 
// 	ChildStdin, 
// 	ChildStdout, 
// 	ChildStderr, 
// 	Command, 
// 	Stdio 
// };

use tokio::{
	io::{self, AsyncBufReadExt, BufReader}, process::{
		ChildStderr, ChildStdin, ChildStdout, Command
	}
};
use std::{path::Path, println, process::Stdio};
use tokio_util::codec::{FramedRead, LinesCodec};
use futures_util::stream::StreamExt;
use std::sync::atomic::{AtomicI32, Ordering};
use std::path::PathBuf;

struct StdIo {
	stdin: ChildStdin,
	stdout: ChildStdout,
	stderr: ChildStderr
}

enum State {
	Stopped,
	Starting,
	Running,
	Stopping
}

struct Server {
	name: String,
	description: Option<String>,
	state: State,
	io: Option<StdIo>,
	path: PathBuf,
}

struct Manager {
	servers: Vec<Server>
}

impl Manager {
	fn create(mut self, name: &str, description: Option<&str>, path: &Path) {
		let des = match description {
			Some(v) => Some(String::from(v)), 
			None => None
		};
		let server = Server {
				name: String::from(name),
				description: des,
				state: State::Stopped,
				io: None,
				path: path.to_path_buf()
			};
		self.servers.push(server);
	}
}


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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), tokio::io::Error> {
	Ok(())
}
