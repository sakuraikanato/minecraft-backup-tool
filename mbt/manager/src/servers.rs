use tokio::process::{ ChildStdin, ChildStdout, ChildStderr };
use std::{collections::HashMap, io, path::{ Path, PathBuf }, process::exit, sync::atomic::{AtomicI32, Ordering}, thread::current};
use core::fmt;
use std::fs;

// --- テーブル出力 ---
use colored::*;
use tabled::Table;
// -------------------

use protocol::views::OutServer;

const DATA_PATH: &str = "../data/server_id.txt";

fn generate_next_id() -> i32 {
	static CURRENT_ID: AtomicI32 = AtomicI32::new(-1);
	if CURRENT_ID.load(Ordering::Relaxed) == -1 {
		let mut next_id = match fs::read_to_string(DATA_PATH) {
			Ok(v) => match v.trim().parse() {
				Ok(v) => v,
				Err(e) => {
					eprintln!("server_idファイルが不正です: {e}");
					panic!();
				}
			}
			Err(e) => {
				eprintln!("server_idファイルが見つかりません: {e}");
				panic!();
			}
		};
		next_id += 1;
		CURRENT_ID.store(next_id, Ordering::Relaxed);
	} else {
		CURRENT_ID.fetch_add(1, Ordering::Relaxed);
	}
	fs::write(DATA_PATH, CURRENT_ID.load(Ordering::Relaxed).to_string());
	CURRENT_ID.load(Ordering::Relaxed)
}

#[derive(Debug)]
pub struct StdIo {
	pub stdin: ChildStdin,
	pub stdout: ChildStdout,
	pub stderr: ChildStderr
}

#[derive(Debug)]
pub enum State {
	Stopped,
	Starting,
	Running,
	Stopping
}

#[derive(Debug)]
pub struct Server {
	pub name: String,
	pub description: Option<String>,
	pub state: State,
	pub io: Option<StdIo>,
	pub path: PathBuf,
}

// サーバーの情報を格納するための構造体
pub struct Manager {
	pub servers: HashMap<i32, Server>
}

impl Manager {
	fn list(&self) -> &Manager {
		&self
	}

	fn info(&self, server_id: i32) -> Option<&Server> {
		match self.servers.get(&server_id) {
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
		self.servers.insert(generate_next_id(), server);
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