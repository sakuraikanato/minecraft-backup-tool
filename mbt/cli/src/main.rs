use core::panic;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    try_join,
};

use interprocess::local_socket::{
	tokio::{ prelude::*, Stream },
	GenericFilePath, GenericNamespaced
};
use protocol::types::{ DataOrError, Options, Request, Response };

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = if GenericNamespaced::is_supported() {
        "mms.sock".to_ns_name::<GenericNamespaced>()?
    } else {
        "/tmp/mms.sock".to_fs_name::<GenericFilePath>()?
    };

	let mut buff = String::new();

  let conn = Stream::connect(name.clone()).await?;
  let mut recver = BufReader::new(&conn);
  let mut sender = &conn;

  let request = Request::List {  };
  let json = serde_json::to_string(&request)?;
  let message = format!("{json}\n");
  let send = sender.write_all(message.as_bytes()).await;
  let n = recver.read_line(&mut buff).await?;
  println!("data: {buff}");
  let res = serde_json::from_str::<Response>(&buff)?;
  let servers = match res {
    Response::Servers { success, data } => {
        if success == true {
            match data {
                DataOrError::Data { data } => {
                    data
                }
                _ => {
                    println!("error");
                    panic!();
                }
            }
        } else {
            println!("失敗");
            panic!()
        }
    }
    _ => {
        println!("error2");
        panic!();
    }
  };

  println!("success: {:?}", servers);

  Ok(())
}