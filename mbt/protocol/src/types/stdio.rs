use tokio::process::{ ChildStdin, ChildStdout, ChildStderr };

#[derive(Debug)]
pub struct StdIo {
	pub stdin: ChildStdin,
	pub stdout: ChildStdout,
	pub stderr: ChildStderr
}