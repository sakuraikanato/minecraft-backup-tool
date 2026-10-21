use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
	Stopped,
	Starting,
	Running,
	Stopping
}