# zfx-michelson

Michelson decoder/encoder library for Rust. Allows parsing michelson expression into `serde::Value` and so to Rust `struct`.

### Requirements

This library needs Node.js 16.

### Summary

The library builds an Rust API layer around the `taquito` library. (See: <https://tezostaquito.io/>)

Taquito interfaces Tezos RPC nodes, a JS module implements a node.js process to handle communication via the standard input and standard output. The node.js process is started with `tokio::process::Command` and the Rust library encodes/decodes requests into JSON maps for each request-response pair.

The JS code is included in the Rust build, so before starting the node.js process it has to be installed. Installation practically means writing the JS module into a file.

### Example

```text
use serde_json::Value;
use zfx_michelson::michelson::*;

install_parser().await;
let mut p = Parser::new();

let storage_str = "{\"int\":\"-26\"}".to_string();
let storage: Value = serde_json::from_str(&storage_str).unwrap();
let schema_str = "{ \"prim\": \"int\" }".to_string();
let schema: Value = serde_json::from_str(&schema_str).unwrap();
let decoded = p.decode(storage, schema.clone()).await;

```

### Build

`cargo build`

### Test

`cargo test`

### Documentation

`cargo doc --open`
