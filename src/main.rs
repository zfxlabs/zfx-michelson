use std::collections::HashMap;

use serde_json::value::Value;
use zfx_michelson::michelson::Parser;

//"kind": "Decode",

//}

#[tokio::main]
async fn main() {
    let mut p = Parser::new();

    let michelson_str = "{ \"prim\": \"Pair\",
  \"args\":
    [ { \"prim\": \"Pair\",
        \"args\":
          [ { \"prim\": \"Pair\", \"args\": [ { \"int\": \"4\" }, [] ] },
            { \"string\": \"tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb\" },
            { \"prim\": \"Right\", \"args\": [ { \"prim\": \"Unit\" } ] } ] },
      { \"int\": \"5\" },
         [ { \"string\": \"tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb\" } ] ] }
";

    let schema_str = "{
  \"prim\": \"pair\",
  \"args\": [
    {
      \"prim\": \"pair\",
      \"args\": [
        {
          \"prim\": \"pair\",
          \"args\": [
            {
              \"prim\": \"big_map\",
              \"args\": [
                {
                  \"prim\": \"key_hash\"
                },
                {
                  \"prim\": \"list\",
                  \"args\": [
                    {
                      \"prim\": \"pair\",
                      \"args\": [
                        {
                          \"prim\": \"pair\",
                          \"args\": [
                            {
                              \"prim\": \"pair\",
                              \"args\": [
                                {
                                  \"prim\": \"key_hash\",
                                  \"annots\": [
                                    \"%baking_account\"
                                  ]
                                },
                                {
                                  \"prim\": \"key\",
                                  \"annots\": [
                                    \"%public_key\"
                                  ]
                                }
                              ]
                            },
                            {
                              \"prim\": \"bytes\",
                              \"annots\": [
                                \"%tls_cert\"
                              ]
                            }
                          ]
                        },
                        {
                          \"prim\": \"timestamp\"
                        }
                      ]
                    }
                  ]
                }
              ],
              \"annots\": [
                \"%old_validator_map\"
              ]
            },
            {
              \"prim\": \"set\",
              \"args\": [
                {
                  \"prim\": \"key_hash\"
                }
              ],
              \"annots\": [
                \"%old_validators\"
              ]
            }
          ]
        },
        {
          \"prim\": \"address\",
          \"annots\": [
            \"%owner\"
          ]
        },
        {
          \"prim\": \"or\",
          \"args\": [
            {
              \"prim\": \"or\",
              \"args\": [
                {
                  \"prim\": \"unit\",
                  \"annots\": [
                    \"%genesis\"
                  ]
                },
                {
                  \"prim\": \"unit\",
                  \"annots\": [
                    \"%open\"
                  ]
                }
              ]
            },
            {
              \"prim\": \"unit\",
              \"annots\": [
                \"%sealed\"
              ]
            }
          ],
          \"annots\": [
            \"%state\"
          ]
        }
      ]
    },
    {
      \"prim\": \"big_map\",
      \"args\": [
        {
          \"prim\": \"key_hash\"
        },
        {
          \"prim\": \"pair\",
          \"args\": [
            {
              \"prim\": \"pair\",
              \"args\": [
                {
                  \"prim\": \"key_hash\",
                  \"annots\": [
                    \"%baking_account\"
                  ]
                },
                {
                  \"prim\": \"key\",
                  \"annots\": [
                    \"%public_key\"
                  ]
                }
              ]
            },
            {
              \"prim\": \"bytes\",
              \"annots\": [
                \"%tls_cert\"
              ]
            }
          ]
        }
      ],
      \"annots\": [
        \"%validator_map\"
      ]
    },
    {
      \"prim\": \"set\",
      \"args\": [
        {
          \"prim\": \"key_hash\"
        }
      ],
      \"annots\": [
        \"%validators\"
      ]
    }
  ]
}";
    let data_str = "{\"old_validator_map\":\"4\",\"old_validators\":[],\"owner\":\"tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb\",\"state\":{},\"validator_map\":\"5\",\"validators\":[\"tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb\"]}";

    let michelson: Value = serde_json::from_str(&michelson_str).expect("michelson json fail");
    let schema: Value = serde_json::from_str(&schema_str).expect("schema json fail");
    let data: Value = serde_json::from_str(&data_str).expect("schema json fail");
    let r2 = p.decode(michelson, schema.clone()).await;
    println!("decoded: {:?}", r2);

    let r1 = p.encode(data, schema).await;
    println!("encoded: {:?}", r1);

    let mut a = HashMap::new();
    let key: usize = 12;
    a.insert(key, "value");

    let r3 = serde_json::to_string(&a).expect("Failed to encode hashmap to JSON");

    println!("hashmap: {:?}", r3);

    println!("end");
}
