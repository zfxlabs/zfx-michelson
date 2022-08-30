use crate::prelude::*;
use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::value::Value;
//use serde_json::Result;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};

use std::process::Stdio;

#[derive(Clone, Debug, Serialize)]
struct Request {
    id: usize,
    content: RequestContent,
}

#[derive(Clone, Debug, Deserialize)]
struct Response {
    id: usize,
    content: ResponseContent,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind")]
pub enum RequestContent {
    Encode {},
    Decode {},
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "status")]
pub enum ResponseContent {
    success { value: Value },
    error {},
}

pub struct Parser {
    stdin: ChildStdin,
    stdout: ChildStdout,
    current_id: usize,
}

impl Parser {
    pub fn new() -> Parser {
        let mut child = tokio::process::Command::new("node")
            .current_dir("./src")
            .args(&["michelson_parser.js"]) //FIXME: config
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("parser::command failed");
        let stdin = child.stdin.take().expect("couldn't get stdin");
        let stdout = child.stdout.take().expect("couldn't get stdout");
        Parser {
            stdin,
            stdout,
            current_id: 0,
        }
    }

    pub async fn encode(&mut self) -> Result<Value> {
        let id = self.current_id;
        self.current_id += 1;
        let content = RequestContent::Encode {};
        submit(&mut self.stdin, id, content).await;
        let encoded_response = receive(&mut self.stdout).await.unwrap();
        if encoded_response.id != id {
            return Err(Error::IdMismatch);
        };
        match encoded_response.content {
            ResponseContent::success { value } => Ok(value),
            ResponseContent::error {} => Err(Error::EncodeError),
        }
    }

    pub async fn decode(&mut self) -> Result<Value> {
        let id = self.current_id;
        self.current_id += 1;
        let content = RequestContent::Decode {};
        submit(&mut self.stdin, id, content).await;
        let decoded_response = receive(&mut self.stdout).await.unwrap();
        if decoded_response.id != id {
            return Err(Error::IdMismatch);
        };
        match decoded_response.content {
            ResponseContent::success { value } => Ok(value),
            ResponseContent::error {} => Err(Error::DecodeError),
        }
    }
}

async fn submit(stdin: &mut ChildStdin, id: usize, content: RequestContent) {
    let request = Request { id, content };
    let encoded = serde_json::to_string(&request).expect("Failed to encode request to JSON");
    let payload = format!("{}\n", encoded);
    stdin
        .write_all(&payload.as_bytes())
        .await
        .expect("stdin - Write failed");
    stdin.flush().await.expect("stdin - flush failed");
}

async fn receive(stdout: &mut ChildStdout) -> Result<Response> {
    let mut reader = BufReader::new(stdout).lines();

    if let Ok(Some(line)) = reader.next_line().await {
        let response: Response = serde_json::from_str(&line).expect("unable to decode json");
        println!("response: {:?}", response);
        Ok(response)
    } else {
        Err(Error::ReadNone)
    }
}
