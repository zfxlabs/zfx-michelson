use serde_json::Value;
use std::fs::File;
use zfx_michelson::michelson::*;

//#[test]
#[tokio::test]
async fn happy_decode() {
    let mut p = Parser::new().await;
    let schema_file = File::open("tests/schema.json").expect("schema file should open read only");
    let schema: Value = serde_json::from_reader(schema_file).expect("file should be proper JSON");
    let michelson_file =
        File::open("tests/michelson-to-decode.json").expect("michelson file should open read only");
    let michelson: Value =
        serde_json::from_reader(michelson_file).expect("file should be proper JSON");

    let result = p.decode(michelson, schema.clone()).await;
    println!("decoded: {:?}", result);
    match result {
        Ok(_decoded) => (),
        _ => panic!("unexpected decode result"),
    }
}
