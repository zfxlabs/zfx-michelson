use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};

use zfx_michelson::*;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
enum State {
    Open,
    Closed,
}
impl EncodeableEnum for State {}

// Rust type
wrapped_struct! {
    Storage {
        state: State,
        bettors: HashSet<String>,
        counter: u64,
        bets: HashMap<String, u8>,
    }
    as StorageForTaquito
    with_schema include_str!("bet-schema.json")
}

const INPUT_STORAGE_STR: &str = include_str!("bet-storage.json");

#[tokio::main]
async fn main() -> Result<()> {
    println!("STORAGE DECODING EXAMPLE");

    install_parser().await;
    let mut parser = Parser::new();

    let schema = Storage::get_schema()?;

    let input = serde_json::from_str(INPUT_STORAGE_STR)?;
    println!("\nINPUT: {:?}", input);

    let decoded_json = parser.decode(input, schema).await?;

    let storage: Storage = from_wrapped_value(decoded_json)?;
    println!("\nDECODED RESULT: {:#?}", storage);
    Ok(())
}
