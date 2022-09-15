//! Various JSON-serialisable types that need special encoding logic with Taquito
use serde::{Deserialize, Serialize};
use std::convert::From;

/// Numbers are generally represented as `String`s (the can are unbounded)
pub type JsonBigNumber = String;

/// Trait to wrap and unwrap a type from the Taquito/Tezos-specific format
pub trait JsonWrapper: Clone {
    type JsonType;

    fn to_wrapped_json(&self) -> Self::JsonType;

    fn from_wrapped_json(value: &Self::JsonType) -> Self;
}

/// Unambiguous representation of `()` in JSON
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct JsonUnit {
    __unit__: (),
}

impl JsonUnit {
    /// Return the JSON version of `()`
    pub fn unit() -> Self {
        JsonUnit { __unit__: () }
    }
}

impl From<()> for JsonUnit {
    fn from(_: ()) -> Self {
        JsonUnit { __unit__: () }
    }
}

impl From<JsonUnit> for () {
    fn from(_: JsonUnit) -> Self {
        ()
    }
}

impl JsonWrapper for () {
    type JsonType = JsonUnit;

    fn to_wrapped_json(&self) -> Self::JsonType {
        JsonUnit::unit()
    }

    fn from_wrapped_json(_value: &Self::JsonType) -> Self {
        ()
    }
}

/// Unambiguous representation of simple enums in JSON
///
/// Note that neither `From<JsonEnum<E>> for E`, nor [`JsonWrapper`]`for E` can be implemented in Rust.
/// Use [`JsonEnum::wrap`] instead.
///

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct JsonEnum<E> {
    __enum__: E,
}

impl<E: Clone> JsonEnum<E> {
    pub fn wrap(e: E) -> Self {
        JsonEnum { __enum__: e }
    }

    pub fn value(&self) -> E {
        self.__enum__.clone()
    }

    pub fn into(self) -> E {
        self.__enum__
    }
}

impl<E> From<E> for JsonEnum<E> {
    fn from(e: E) -> Self {
        JsonEnum { __enum__: e }
    }
}

/// Macro to generate the trivial implementations for the [`JsonWrapper`] trait.
///
/// ## Examples
///
/// ```text
/// use zfx_michelson::json_wrapper;
///
/// json_wrapper!(String as Self);    // No wrapping necessary
/// json_wrapper!(Vec<T> as Self; T); // Ditto, but with generics
/// json_wrapper!(u64 as String);      // Wrapped version is a `String`; may panic on unwrapping
/// ```
#[macro_export]
macro_rules! json_wrapper {
    ($typ:ty as Self) => {
        impl $crate::JsonWrapper for $typ {
            type JsonType = $typ;

            fn to_wrapped_json(&self) -> Self::JsonType {
                self.clone()
            }

            fn from_wrapped_json(value: &Self::JsonType) -> Self {
                value.clone()
            }
        }
    };
    ($typ:ty as String) => {
        impl $crate::JsonWrapper for $typ {
            type JsonType = String;

            fn to_wrapped_json(&self) -> String {
                self.to_string()
            }

            fn from_wrapped_json(value: &String) -> Self {
                value.as_str().parse().expect(format!("JsonWrapper: Unparseable string {:?}", value).as_str())
            }
        }
    };
    ($typ:ty as Self; $($g:tt),+) => {
        impl<$($g:std::clone::Clone,)+> $crate::JsonWrapper for $typ {
            type JsonType = $typ;

            fn to_wrapped_json(&self) -> Self::JsonType {
                self.clone()
            }

            fn from_wrapped_json(value: &Self::JsonType) -> Self {
                value.clone()
            }
        }
    }
}

json_wrapper!(String as Self);
json_wrapper!(Vec<T> as Self; T);
json_wrapper!(u64 as String);
json_wrapper!(i64 as String);

#[cfg(test)]
mod test {
    use super::*;
    use crate::MichelsonMap;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_basic_serialise_unit() {
        let unit = JsonUnit::unit();
        let json = serde_json::to_string(&unit).unwrap();
        let expected = "{\"__unit__\":null}";
        println!("{:?}", json);
        assert!(json == expected);
        let back: JsonUnit = serde_json::from_str(&json).unwrap();
        assert_eq!(unit, back);
    }

    #[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
    enum E {
        A,
        B,
    }

    #[test]
    fn test_basic_serialise_enum() {
        let e = JsonEnum::wrap(E::A);
        let json = serde_json::to_string(&e).unwrap();
        let expected = "{\"__enum__\":\"A\"}";
        println!("{:?}", json);
        assert!(json == expected);
        let back: JsonEnum<E> = serde_json::from_str(&json).unwrap();
        assert_eq!(e, back);
    }

    #[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
    struct S {
        u: JsonUnit,
        e: JsonEnum<E>,
        m: MichelsonMap<String, usize>,
        b: JsonBigNumber,
        i: isize,
        s: String,
    }

    #[test]
    fn test_serialise_struct() {
        let s = S {
            u: JsonUnit::unit(),
            e: JsonEnum::wrap(E::A),
            m: MichelsonMap::new(),
            b: "42".to_owned(),
            i: 1,
            s: "foo".to_owned(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let expected = "{\"u\":{\"__unit__\":null},\"e\":{\"__enum__\":\"A\"},\
            \"m\":{\"MichelsonMap\":{}},\"b\":\"42\",\"i\":1,\"s\":\"foo\"}";
        println!("{:?}", json);
        assert!(json == expected);
        let back: S = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct RustStruct {
        u: (),
        e: E,
        m: HashMap<String, usize>,
        b: i64,
        i: isize,
        s: String,
    }

    fn convert_struct(s: S) -> RustStruct {
        RustStruct {
            u: s.u.into(),
            e: s.e.value(),
            m: s.m.into(),
            b: s.b.parse().unwrap(),
            i: s.i,
            s: s.s,
        }
    }

    fn convert_struct_back(r: RustStruct) -> S {
        S {
            u: r.u.into(),
            e: JsonEnum::wrap(r.e),
            m: r.m.into(),
            b: r.b.to_string(),
            i: r.i,
            s: r.s,
        }
    }

    #[test]
    fn test_rustify() {
        let s = S {
            u: JsonUnit::unit(),
            e: JsonEnum::wrap(E::A),
            m: MichelsonMap::new(),
            b: "42".to_owned(),
            i: 1,
            s: "foo".to_owned(),
        };
        let r = convert_struct(s.clone());
        let expected = RustStruct {
            u: (),
            e: E::A,
            m: HashMap::new(),
            b: 42,
            i: 1,
            s: "foo".to_owned(),
        };
        assert_eq!(r, expected);
        let s2 = convert_struct_back(r);
        assert_eq!(s2, s);
    }
}
