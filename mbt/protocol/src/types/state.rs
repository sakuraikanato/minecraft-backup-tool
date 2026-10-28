use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum State {
	Stopped,
	Starting,
	Running,
	Stopping
}