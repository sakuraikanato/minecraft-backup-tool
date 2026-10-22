use core::fmt;
use std::path::PathBuf;
use colored::{self, Colorize};
use serde::{Deserialize, Serialize};
use tabled::{Table, derive, settings::TableOption};

use crate::views::OutServer;

use super::{ State, StdIo };

#[derive(Debug)]
pub struct Server {
	pub name: String,
	pub description: Option<String>,
	pub state: State,
	pub io: Option<StdIo>,
	pub path: PathBuf,
	pub config: ServerConfig
}

#[derive(Debug)]
pub struct ServerConfig {
	pub java_path: Option<PathBuf>,
	pub min_memory_mb: u32,
	pub max_memory_mb: u32,
	pub jvm_args: Vec<String>,
	pub program_args: Vec<String>
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

impl ServerConfig {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}
}

impl Default for ServerConfig {
	fn default() -> Self {
			Self {
				java_path: None,
				min_memory_mb: 1024,
				max_memory_mb: 4096,
				jvm_args: Vec::<String>::new(),
				program_args: Vec::<String>::new()
			}
	}
}