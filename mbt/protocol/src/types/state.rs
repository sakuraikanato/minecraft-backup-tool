#[derive(Debug)]
pub enum State {
	Stopped,
	Starting,
	Running,
	Stopping
}