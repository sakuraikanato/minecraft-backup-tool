// --- テーブル出力 ---
use tabled::Tabled;
// -------------------

#[derive(Tabled)]
pub struct OutServer {
	#[tabled(rename = "名前")]
	pub name: String,

	#[tabled(rename = "情報")]
	pub description: String,

	#[tabled(rename = "状態")]
	pub state: String,

	#[tabled(rename = "パス")]
	pub path: String,
}