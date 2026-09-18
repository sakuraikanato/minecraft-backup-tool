// use std::process::{ 
// 	ChildStdin, 
// 	ChildStdout, 
// 	ChildStderr, 
// 	Command, 
// 	Stdio 
// };

use tokio::process::{
	ChildStdin,
	ChildStdout,
	ChildStderr,
	Command
};
use std::{println, process::Stdio};
use tokio_util::codec::{FramedRead, LinesCodec};
use futures_util::stream::StreamExt;
struct StdIo {
	stdin: ChildStdin,
	stdout: ChildStdout,
	stderr: ChildStderr
}


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
	let stdio1 = start_subprocess("8.8.8.8").unwrap();
	let stdio2 = start_subprocess("1.1.1.1").unwrap();
	let task1 = tokio::spawn(read_line(stdio1.stdout));
	let task2 = tokio::spawn(read_line(stdio2.stdout));

	let _ = tokio::try_join!(task1, task2);
	Ok(())
}
