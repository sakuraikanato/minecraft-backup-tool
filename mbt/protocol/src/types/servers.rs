use core::fmt;
use std::path::{ PathBuf, Path };
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
pub struct ServerUpdate<P: AsRef<Path>> {
	pub name: Option<String>,
	pub description: Option<String>,
	pub state: Option<State>,
	pub io: Option<StdIo>,
	pub path: Option<P>,
	pub config: Option<ServerConfigUpdate>
}

#[derive(Debug)]
pub struct ServerConfig {
	pub java_path: Option<PathBuf>,
	pub min_memory_mb: u32,
	pub max_memory_mb: u32,
	pub jvm_args: Vec<String>,
	pub program_args: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfigUpdate {
	pub java_path: Option<PathBuf>,
	pub min_memory_mb: Option<u32>,
	pub max_memory_mb: Option<u32>,
	pub jvm_args: Option<Vec<String>>,
	pub program_args: Option<Vec<String>>
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

	pub fn update(&mut self, update: ServerConfigUpdate) {
		if let Some(java_path) = update.java_path {
			self.java_path = Some(java_path);
		};
		if let Some(min_memory) = update.min_memory_mb {
			self.min_memory_mb = min_memory;
		};
		if let Some(max_memory) = update.max_memory_mb {
			self.max_memory_mb = max_memory;
		};
		if let Some(jvm_args) = update.jvm_args {
			self.jvm_args = jvm_args;
		};
		if let Some(program_args) = update.program_args {
			self.program_args = program_args;
		};
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

impl Default for ServerUpdate<String> {
	fn default() -> Self {
		Self { name: None, description: None, state: None, io: None, path: None, config: None }
	}
}

impl Default for ServerConfigUpdate {
	fn default() -> Self {
			Self { java_path: None, min_memory_mb: None, max_memory_mb: None, jvm_args: None, program_args: None }
	}
}