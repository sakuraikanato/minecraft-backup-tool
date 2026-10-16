use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Options {

}


#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    #[serde(rename = "list")]
    List {},
    #[serde(rename = "info")]
    Info {
        server_id: i32
    },
    #[serde(rename = "create")]
    Create {
        name: String,
        description: Option<String>,
        path: PathBuf
    },
    #[serde(rename = "update")]
    Update {
        server_id: i32,
        name: Option<String>,
        description: Option<String>,
        path: Option<PathBuf>
    },
    #[serde(rename = "delete")]
    Delete {
        server_id: i32
    },
    #[serde(rename = "start")]
    Start {
        server_id: i32
    },
    #[serde(rename = "stop")]
    Stop {
        server_id: i32
    },
    #[serde(rename = "option")]
    Option {
        option: Options
    }
}