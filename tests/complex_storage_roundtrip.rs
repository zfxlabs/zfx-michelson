use serde::{Serialize, Deserialize};
use serde_json::*;
use zfx_michelson::*;
use std::collections::HashSet;

const SCHEMA: &str = r#"{
  "prim": "pair",
  "args": [
    {
      "prim": "pair",
      "args": [
        {
          "prim": "pair",
          "args": [
            {
              "prim": "big_map",
              "args": [
                {
                  "prim": "key_hash"
                },
                {
                  "prim": "list",
                  "args": [
                    {
                      "prim": "pair",
                      "args": [
                        {
                          "prim": "pair",
                          "args": [
                            {
                              "prim": "pair",
                              "args": [
                                {
                                  "prim": "key_hash",
                                  "annots": [
                                    "%baking_account"
                                  ]
                                },
                                {
                                  "prim": "key",
                                  "annots": [
                                    "%public_key"
                                  ]
                                }
                              ]
                            },
                            {
                              "prim": "bytes",
                              "annots": [
                                "%tls_cert"
                              ]
                            }
                          ]
                        },
                        {
                          "prim": "timestamp"
                        }
                      ]
                    }
                  ]
                }
              ],
              "annots": [
                "%old_validator_map"
              ]
            },
            {
              "prim": "set",
              "args": [
                {
                  "prim": "key_hash"
                }
              ],
              "annots": [
                "%old_validators"
              ]
            }
          ]
        },
        {
          "prim": "address",
          "annots": [
            "%owner"
          ]
        },
        {
          "prim": "or",
          "args": [
            {
              "prim": "or",
              "args": [
                {
                  "prim": "unit",
                  "annots": [
                    "%genesis"
                  ]
                },
                {
                  "prim": "unit",
                  "annots": [
                    "%open"
                  ]
                }
              ]
            },
            {
              "prim": "unit",
              "annots": [
                "%sealed"
              ]
            }
          ],
          "annots": [
            "%state"
          ]
        }
      ]
    },
    {
      "prim": "big_map",
      "args": [
        {
          "prim": "key_hash"
        },
        {
          "prim": "pair",
          "args": [
            {
              "prim": "pair",
              "args": [
                {
                  "prim": "key_hash",
                  "annots": [
                    "%baking_account"
                  ]
                },
                {
                  "prim": "key",
                  "annots": [
                    "%public_key"
                  ]
                }
              ]
            },
            {
              "prim": "bytes",
              "annots": [
                "%tls_cert"
              ]
            }
          ]
        }
      ],
      "annots": [
        "%validator_map"
      ]
    },
    {
      "prim": "set",
      "args": [
        {
          "prim": "key_hash"
        }
      ],
      "annots": [
        "%validators"
      ]
    }
  ]
}"#;

// TODO we don't need this here, but for other tests it might come in handy
#[allow(dead_code)]
fn remove_nl(input: &str) -> String {
    let mut s = input.to_owned();
    s.retain(|c| c != '\n');
    s
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
enum State {
    Genesis,
    Sealed,
    Open
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
struct Registration {
    baking_account: String,
    public_key: String,
    tls_cert: String // Taquito expects `bytes` as a hex string
}

// Target type
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
struct Storage {
   owner: String,
   state: JsonEnum<State>,
   validators: HashSet<String>,
   validator_map: MichelsonMap<String, Registration>,
   old_validators: HashSet<String>,
   // this is kept empty for now, the value in the contract has a rather complex type
   old_validator_map: MichelsonMap<String, JsonUnit>
}

#[tokio::test]
async fn complex_storage() {
    install_parser().await;
    let mut p = Parser::new();

    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    println!("SCHEMA: {}", schema);

    let storage = Storage {
        state: JsonEnum::wrap(State::Genesis),
        owner: "tz1burnburnburnburnburnburnburjAYjjX".to_owned(),
        validators: HashSet::from(["tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c".to_owned()]),
        old_validators: HashSet::default(),
        old_validator_map: MichelsonMap::new(),
        validator_map: [
            ("tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c".to_owned(),
              Registration {
                  baking_account: "tz1d8LSBpEsLtLkCmaj2yBdv2xF4wSYNAa8c".to_owned(),
                  public_key: "edpku2tvek7QFRYm12819P8RwSY8m7zSzKV9RMnWHy3xVbrBwN5zAg".to_owned(),
                  tls_cert: "DEADBEEF".to_owned()})].into()
    };
    let value = serde_json::to_value(&storage).unwrap();
    println!("DATA: {}", value);

    let mich = p.encode(value.clone(), schema.clone()).await.unwrap();
    println!("MICH: {}", mich);

    let decoded_json = p.decode(mich.clone(), schema.clone()).await.unwrap();
    println!("DECODED JSON: {}", decoded_json);

    let decoded_storage: Storage = serde_json::from_value(decoded_json).unwrap();
    println!("DECODED: {:?}", decoded_storage);

    assert_eq!(decoded_storage, storage);
    println!("{}", remove_nl(SCHEMA));
}