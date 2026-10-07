use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    try_join,
};

use interprocess::local_socket::{
	tokio::{ prelude::*, Stream },
	GenericFilePath, GenericNamespaced
};
use std::io;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = if GenericNamespaced::is_supported() {
        "mms.sock".to_ns_name::<GenericNamespaced>()?
    } else {
        "/tmp/mms.sock".to_fs_name::<GenericFilePath>()?
    };

	let mut buffer = String::new();

  let conn = Stream::connect(name.clone()).await?;
  let mut recver = BufReader::new(&conn);
  let mut sender = &conn;

  let message = format!("get message: {buffer}\n");
  let send = sender.write_all(message.as_bytes());
  let rec = recver.read_line(&mut buffer);
  try_join!(rec, send);

  Ok(())
}