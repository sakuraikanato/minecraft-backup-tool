use std::process::{ 
	ChildStdin, 
	ChildStdout, 
	ChildStderr, 
	Command, 
	Stdio 
};
use std::io::prelude::*;
use std::io::BufReader;

struct StdIo {
	stdin: ChildStdin,
	stdout: ChildStdout,
	stderr: ChildStderr
}

fn start_subprocess(ip: &str) -> Result<StdIo, std::io::Error> {
	let child = Command::new("ping")
		.arg(ip)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.expect("failed to execute process");

	let stdin = child.stdin.unwrap();
	let stdout = child.stdout.unwrap();
	let stderr = child.stderr.unwrap();
	let stdio: StdIo = StdIo {stdin, stdout, stderr};
	Ok(stdio)
}

fn main() -> Result<(), std::io::Error> {
	
	Ok(())
}
