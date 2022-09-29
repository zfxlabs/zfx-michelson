//! The main decoder/encoder ([`Parser`])
use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::value::Value;

use include_dir::{include_dir, Dir};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};

use std::process::Stdio;

use tokio::fs::File;

static SCRIPTS_DIR: Dir<'_> = include_dir!("./scripts");
static BUNDLE_NAME: &str = "michelson_parser.bundle.js";

pub type MichelsonV1Expression = Value;

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
    Encode { data: Value, schema: Value },
    Decode { michelson: Value, schema: Value },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "status")]
pub enum ResponseContent {
    Success { value: Value },
    Error { error: Value },
}

/// Install the necessary Javascript code in the current directory
///
/// This step is needed before creating a [`Parser`]
pub async fn install_parser() {
    let parser_js = SCRIPTS_DIR.get_file(BUNDLE_NAME).unwrap();
    let mut file_to_deploy = File::create(BUNDLE_NAME).await.unwrap();
    file_to_deploy
        .write_all(parser_js.contents())
        .await
        .unwrap();
}

/// The main decoder/encoder backed by a NodeJS process using Taquito
///
/// NodeJS must be installed on the system,
/// in particular the `node` binary must be present in the `PATH`
///
/// ## Examples
/// See the [crate-level documentation](crate).
///
/// ## Panics
///
/// - [`new`](Parser::new) panics if the child process cannot be started
/// - [`encode`](Parser::encode) and [`decode`](Parser::decode) may panic
///   on I/O error (most likely, when the child process exited)
// TODO: unusual race conditions with concurrent calls to the same parser may cause an [`Error::IdMismatch`] result.
// If this is observed in practice, we can match respective request and response IDs to avoid this

pub struct Parser {
    stdin: ChildStdin,
    stdout: ChildStdout,
    current_id: usize,
}

impl Parser {
    /// Create a new `Parser` instance, **this starts a new NodeJS process**
    pub fn new() -> Parser {
        let mut child = Command::new("node")
            //.current_dir("./scripts")
            .args(&[BUNDLE_NAME]) //FIXME: config
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

    /// Encode a [value][`serde_json::Value] to the JSON-based Michelson format
    /// according to the data schema provided
    pub async fn encode(
        &mut self,
        data: Value,
        schema: MichelsonV1Expression,
    ) -> Result<MichelsonV1Expression> {
        let id = self.current_id;
        self.current_id += 1;
        let content = RequestContent::Encode { data, schema };
        submit(&mut self.stdin, id, content).await;
        let encoded_response = receive(&mut self.stdout).await.unwrap();
        if encoded_response.id != id {
            return Err(Error::IdMismatch);
        };
        match encoded_response.content {
            ResponseContent::Success { value } => Ok(value),
            ResponseContent::Error { error } => Err(Error::EncodeError { error }),
        }
    }

    /// Decode a [value][`serde_json::Value] from the JSON-based Michelson format
    /// according to the data schema provided
    pub async fn decode(
        &mut self,
        michelson: MichelsonV1Expression,
        schema: MichelsonV1Expression,
    ) -> Result<Value> {
        let id = self.current_id;
        self.current_id += 1;
        let content = RequestContent::Decode { michelson, schema };
        submit(&mut self.stdin, id, content).await;
        let decoded_response = receive(&mut self.stdout).await.unwrap();
        if decoded_response.id != id {
            return Err(Error::IdMismatch);
        };
        match decoded_response.content {
            ResponseContent::Success { value } => Ok(value),
            ResponseContent::Error { error } => Err(Error::DecodeError { error }),
        }
    }

    // TODO `drop`: technically we'd need to kill the child process explicitly,
    // or verify whether the NodeJS process exits cleanly on closing `stdin`
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
        Ok(response)
    } else {
        Err(Error::ReadNone)
    }
}
