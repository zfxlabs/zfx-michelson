use crate::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};

use std::process::Stdio;

#[derive(Clone, Debug, Serialize)]
pub struct Request {
    id: usize,
    content: RequestContent,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
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
pub enum ResponseContent {}

pub struct Parser {
    child: tokio::process::Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
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
        let mut stdin = child.stdin.take().expect("couldn't get stdin");
        let mut stdout = child.stdout.take().expect("couldn't get stdout");
        Parser {
            child,
            stdin,
            stdout,
        }
    }

    pub async fn encode(&mut self) {
        let content = RequestContent::Encode {};
        submit(&mut self.stdin, 1, content).await;
        receive(&mut self.stdout).await;
    }

    pub async fn decode(&mut self) {
        let content = RequestContent::Decode {};
        submit(&mut self.stdin, 1, content).await;
        receive(&mut self.stdout).await;
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

async fn receive(stdout: &mut ChildStdout) {
    let mut reader = BufReader::new(stdout).lines();

    if let Ok(Some(line)) = reader.next_line().await {
        println!("Line: {:?}", line);
    } else {
        // FIXME: proper handle the error
        println!("this shouldn't happen, the JS should have responded.");
    }
}
