//! This module contains the [`Micheline`] type and associated type synonyms
use serde::{Deserialize, Serialize};
use serde_json;

/// Annotations
pub type Annotations = Vec<String>;

/// Michelson primitive
pub type Primitive = String;

/// An unbounded `int` or `nat`
pub type Number = String;

/// A hex-encoded sequence of bytes
///
/// TODO: Taquito seems to omit the `"0x"` prefix, yet it is prescribed in the description
pub type Bytes = String;

/// Representation of [_Micheline_](http://tezos.gitlab.io/kathmandu/michelson.html?highlight=view#concrete-syntax) in Rust
///
/// Note that this is a simplistic and future-proof representation,
/// numbers as well as primitives are encoded as `String`s, like in the JSON format.

// TODO: simple values doesn't seem to support annotations in Micheline?
// TODO: we could use Taquito to convert between JSON syntax and the 'normal' Micheline version
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Micheline {
    Number {
        int: Number,
    },
    String {
        string: String,
    },
    Bytes {
        bytes: String,
    },
    Prim {
        prim: Primitive,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        args: Vec<Micheline>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        annots: Annotations,
    },
    Seq(Vec<Micheline>),
}

impl Micheline {
    // TODO: add constructors for each variant
    // TODO: we could add functions checking for the variants and extracting values, also functions for walking the structure

    /// Convert a [`serde_json::Value`] to `Micheline`
    ///
    /// Fails if the JSON value doesn't represent a valid `Micheline` value
    pub fn from_json_value(json: serde_json::Value) -> Result<Micheline, serde_json::Error> {
        serde_json::from_value(json)
    }

    /// Convert a `Micheline` value to a [`serde_json::Value`]
    pub fn to_json_value(self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    /// Desirialise from a JSON string
    pub fn from_str(s: &str) -> Result<Micheline, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Serialise into a JSON string
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod test {
    use super::Micheline::*;
    use super::*;

    #[test]
    fn test_micheline_basic_type() {
        let s = r#"{ "prim": "map", "args": [{ "prim": "string" }, { "prim": "int" }]}"#;
        let m = Micheline::from_str(s).unwrap();
        println!("{:?}", m);

        let m = Prim {
            prim: "map".to_string(),
            args: vec![Prim {
                prim: "string".to_string(),
                args: vec![],
                annots: vec![],
            }],
            annots: vec![],
        };
        println!("{:?}", m);
        let s = m.to_string().unwrap();
        println!("{}", s);
    }
    #[test]
    fn test_micheline_empty_seq() {
        let s = "[]";
        let m = Micheline::from_str(s).unwrap();
        assert_eq!(Seq(vec![]), m);
    }

    #[test]
    fn test_micheline_increment_contract() {
        let type_j = r#"
        { "prim": "or",
          "args":
            [ { "prim": "or",
                "args":
                  [ { "prim": "int", "annots": [ "%decrement" ] },
                    { "prim": "int", "annots": [ "%increment" ] } ] },
              { "prim": "unit", "annots": [ "%reset" ] } ] } "#;
        let value_j = r#" {"prim":"Left","args":[{"prim":"Left","args":[{"int":"1"}]}]} "#;

        let type_m = Micheline::from_str(type_j).unwrap();
        println!("{:?}", type_m);
        println!("{}", type_m.to_string().unwrap());

        let value_m = Micheline::from_str(value_j).unwrap();
        println!("{:?}", value_m);
        println!("{}", value_m.to_string().unwrap());
    }
}
