use serde::{Deserialize, Serialize};
use serde_json::*;
use std::collections::{HashMap, HashSet};
use zfx_michelson::*;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
enum State {
    Genesis,
    Sealed,
    Open,
}
impl EncodeableEnum for State {}

wrapped_struct! { Registration {
    baking_account: String,
    public_key: String,
    tls_cert: String, // Taquito expects `bytes` as a hex string
} as WrappedRegistration }

// Target type
wrapped_struct! { Storage {
    owner: String,
    state: State,
    validators: HashSet<String>,
    validator_map: HashMap<String, Registration>,
    old_validators: HashSet<String>,
    // this is kept empty for now, the value in the contract has a rather complex type
    old_validator_map: HashMap<String, ()>,
} as WrappedStorage with_schema include_str!("./schema.json") }

#[tokio::test]
async fn complex_storage_roundtrip_with_macro() {
    install_parser().await;
    let mut p = Parser::new();

    let schema: Value = Storage::get_schema().unwrap();
    println!("SCHEMA: {}", schema);

    let storage = Storage {
        state: State::Genesis,
        owner: "tz1burnburnburnburnburnburnburjAYjjX".to_owned(),
        validators: HashSet::from(["tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c".to_owned()]),
        old_validators: HashSet::default(),
        old_validator_map: HashMap::new(),
        validator_map: [(
            "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c".to_owned(),
            Registration {
                baking_account: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c".to_owned(),
                public_key: "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg".to_owned(),
                tls_cert: "DEADBEEF".to_owned(),
            },
        )]
        .into(),
    };
    let value = to_wrapped_value(storage.clone()).unwrap();
    println!("DATA: {}", value);

    let mich = p.encode(value.clone(), schema.clone()).await.unwrap();
    println!("MICH: {}", mich);

    let decoded_json = p.decode(mich, schema.clone()).await.unwrap();
    println!("DECODED JSON: {:#}", decoded_json);

    let decoded_storage: Storage = from_wrapped_value(decoded_json).unwrap();
    println!("DECODED: {:#?}", decoded_storage);

    assert_eq!(decoded_storage, storage);
}
