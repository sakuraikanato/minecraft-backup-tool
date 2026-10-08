use tokio::process::{ ChildStdin, ChildStdout, ChildStderr };
use std::path::{ PathBuf, Path };
use core::fmt;

// --- テーブル出力 ---
use colored::*;
use tabled::Table;
// -------------------

use super::views::OutServer;

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
	pub servers: Vec<Server>
}

impl Manager {
	pub fn list(&self) -> &Manager {
		&self
	}

	pub fn info(&self, index: usize) -> Option<&Server> {
		match self.servers.get(index) {
			Some(v) => Some(v),
			None => None
		}
	}

	pub fn create(&mut self, name: &String, description: &Option<String>, path: &Path) {
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