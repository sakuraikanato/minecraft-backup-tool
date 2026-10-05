// use std::process::{ 
// 	ChildStdin, 
// 	ChildStdout, 
// 	ChildStderr, 
// 	Command, 
// 	Stdio 
// };

use tokio::{
	io::{self, AsyncBufReadExt, BufReader}, 
	process::{
		ChildStderr, ChildStdin, ChildStdout, Command
	}
};
use core::fmt;
use std::{io::{Write, stdin}, path::Path, process::Stdio};
use tokio_util::codec::{FramedRead, LinesCodec};
use futures_util::stream::StreamExt;
use std::sync::atomic::{AtomicI32, Ordering};
use std::path::PathBuf;
use colored::*;
use tabled::{Table, Tabled};

#[derive(Debug)]
struct StdIo {
	stdin: ChildStdin,
	stdout: ChildStdout,
	stderr: ChildStderr
}

#[derive(Debug)]
enum State {
	Stopped,
	Starting,
	Running,
	Stopping
}

#[derive(Debug)]
struct Server {
	name: String,
	description: Option<String>,
	state: State,
	io: Option<StdIo>,
	path: PathBuf,
}

#[derive(Tabled)]
struct OutServer {
	#[tabled(rename = "名前")]
	name: String,

	#[tabled(rename = "情報")]
	description: String,

	#[tabled(rename = "状態")]
	state: String,

	#[tabled(rename = "パス")]
	path: String,
}

impl fmt::Display for Server {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let color_state = match self.state {
			State::Stopped => "Stopped".bright_black().to_string(),
			State::Stopping => "Stopping".yellow().to_string(),
			State::Starting => "Starting".cyan().to_string(),
			State::Running => "Running".green().bold().to_string()
		};

		let data = vec![OutServer {
			name: self.name.clone(),
			description: match &self.description {
				Some(v) => v.clone(),
				None => "説明なし".to_string()
			},
			state: color_state,
			path: self.path.to_str().unwrap().to_string()
		}];

		let table = Table::new(data).to_string();
		let _ = write!(f, "{}", table);
		Ok(())
	}
}

struct Manager {
	servers: Vec<Server>
}

impl Manager {
	fn get_all(&self) -> &Manager {
		&self
	}

	fn get(&self, index: usize) -> Option<&Server> {
		match self.servers.get(index) {
			Some(v) => Some(v),
			None => None
		}
	}

	fn create(&mut self, name: &String, description: &Option<String>, path: &Path) {
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

impl fmt::Display for Manager {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let data:Vec<OutServer> = self.servers.iter().map(|s| {
			let color_state = match s.state {
					State::Stopped => "Stopped".bright_black().to_string(),
					State::Stopping => "Stopping".yellow().to_string(),
					State::Starting => "Starting".cyan().to_string(),
					State::Running => "Running".green().bold().to_string()
				};
			OutServer {
				name: s.name.clone(),
				description: match &s.description {
					Some(v) => v.clone(),
					None => "説明なし".to_string()
				},
				state: color_state,
				path: s.path.to_str().unwrap().to_string()
			}
		}).collect();

		let table = Table::new(data).to_string();
		let _ = write!(f, "{}", table);
		Ok(())
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
	let mut server_list = Manager {servers: Vec::<Server>::new()};
	let mut input = BufReader::new(io::stdin()).lines();
	
	printfl("mbt $ ");
	while let Some(line) = input.next_line().await? {
		let commands: Vec<&str> = line.split(" ").collect();
		if CURRENT_NUM.load(Ordering::Relaxed) == -1 {
			match commands[0] {
				"exit" => { return Ok(()) }
				"info" => { println!("{}", server_list.get(commands[1].parse::<usize>().unwrap()).unwrap()); } // メモ：将来的にここはDBのidから検索させたい
				"list" => { println!("{}", server_list.get_all()) }, // テスト出力
				"create" => {create_chat(&mut server_list);},
				_ => {println!("errer: unknown command {}", commands[0])}
			}
		} else {
			if commands[0].starts_with(":") {

			} else {

			}
		}
		printfl("mbt $ ");
	}
	Ok(())
}
