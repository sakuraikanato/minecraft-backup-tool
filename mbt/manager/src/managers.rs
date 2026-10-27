use serde::de;
use tokio::process::{ ChildStdin, ChildStdout, ChildStderr };
use std::{collections::HashMap, fmt::Error, io, path::{ Path, PathBuf }, process::exit, sync::atomic::{AtomicI32, Ordering}, thread::current};
use core::fmt;
use std::fs;

// --- テーブル出力 ---
use colored::*;
use tabled::Table;
// -------------------

use protocol::{
	types::{ self, Server, ServerConfig, State, StdIo, MmsError }, views::OutServer
};

const DATA_PATH: &str = "./manager/data/server_id.txt";

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

// サーバーの情報を格納するための構造体

#[derive(Debug)]
pub struct Manager {
	pub servers: HashMap<i32, Server>
}

impl Manager {
	pub fn list(&self) -> &HashMap<i32, Server> {
		&self.servers
	}

	pub fn info(&self, server_id: i32) -> Option<&Server> {
		match self.servers.get(&server_id) {
			Some(v) => Some(v),
			None => None
		}
	}

	pub fn create(&mut self, name: &String, description: &Option<String>, path: &Path) -> types::Result<()> {
		let des = match description {
			Some(v) => Some(String::from(v)), 
			None => None
		};
		let server = Server {
				name: String::from(name),
				description: des,
				state: State::Stopped,
				io: None,
				path: path.to_path_buf(),
				config: ServerConfig::new()
			};
		match self.servers.insert(generate_next_id(), server) {
			Some(_) => Err(MmsError::Except),
			None => Ok(())
		}
	}

	pub fn update<P: AsRef<Path>>(&mut self, server_id: i32, name: &Option<String>, description: &Option<String>, path: &Option<P>) -> types::Result<()> {
		let server = match self.servers.get_mut(&server_id) {
			Some(v) => v,
			None => return Err(MmsError::NotFound)
		};
		
		if let Some(name) = name {
			server.name = name.clone();
		}
		if let Some(description) = description {
			server.description = Some(description.clone());
		}
		if let Some(path) = path {
			server.path = PathBuf::from(path.as_ref())
		}
		Ok(())
	}

	pub fn delete(&mut self, server_id: i32) -> types::Result<()> {
		match self.servers.remove(&server_id) {
			Some(_) => Ok(()),
			None => Err(MmsError::NotFound)
		}
	}
}

impl fmt::Display for Manager {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let data:Vec<OutServer> = self.servers.iter().map(|(_, s)| {
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