//! Types, traits and macros for conversion between Rust and Taquito-style JSON data.
//!
//! This module works in tandem with the javascript pre-/postprocessor in `src/json_converter.js`.
//!
//! ## Caveats
//!
//! - all types are expected to implement `PartialEq + Eq + Debug + Clone + serde::Serialize + serde::Deserialize`
//! - Tuples and tuple `enum`s are to be avoided due to their representation
//!    being indistinguishable from that of `struct`s/`record`s in the Michelson schema
//! - Record `enum`s need the explicit `#[serde(rename_all = "camelCase")]` annotiation
//!   to comply with Michelson naming conventions

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
use std::convert::From;
use std::hash::Hash;

use crate::Result;

/// Numbers are generally represented as `String`s (the can are unbounded)
pub type JsonBigNumber = String;

/// Trait to wrap and unwrap a type from the Taquito/Tezos-specific format
///
/// Implementations are provided for most types that can appear in Michelson or Ligo.
/// It's expected that only application-specific datataypes (generally `struct`s) need a manual
/// implementation. See [`wrapped_struct!`].
///
/// Usually it's more convenient to use the provided generic functions
// (`to_wrapped_* and `from_wrapped_*`) from this module
/// than the trait functions directly.
pub trait JsonWrapped: Clone {
    /// The transport type that can be serialised to JSON, used with Taquito,
    /// and mapped to the Michelson data schema.
    ///
    /// Note that this should still be a _plain Rust type_ that result
    // in the expected JSON format when serialised.
    type JsonType;

    /// The Michelson data schema if available, the empty string otherwise.
    ///
    /// Implementors should provide it if it's known at compile-time and access it using [`get_schema`](JsonWrapped::get_schema).
    const SCHEMA_STR: &'static str = "";

    fn to_wrapped_json(&self) -> Result<Self::JsonType>;

    fn from_wrapped_json(value: &Self::JsonType) -> Result<Self>;

    fn get_schema() -> Result<serde_json::Value> {
        if Self::SCHEMA_STR.is_empty() {
            Err(crate::Error::NoSchema)
        } else {
            Ok(serde_json::from_str(Self::SCHEMA_STR)?)
        }
    }
}

/// Generic helper function for the implementors of [`JsonWrapped`] trait
pub fn from_wrapped_json<T: JsonWrapped>(v: &T::JsonType) -> Result<T> {
    T::from_wrapped_json(v)
}

/// Generic helper function for the implementors of [`JsonWrapped`] trait
pub fn to_wrapped_json<T: JsonWrapped>(t: &T) -> Result<T::JsonType> {
    t.to_wrapped_json()
}

/// Generic helper function for the implementors of [`JsonWrapped`] trait
pub fn from_wrapped_str<T: JsonWrapped>(s: &str) -> Result<T>
where
    <T as JsonWrapped>::JsonType: DeserializeOwned,
{
    let x: <T as JsonWrapped>::JsonType = serde_json::from_str(s)?;
    Ok(T::from_wrapped_json(&x)?)
}

/// Generic helper function for the implementors of [`JsonWrapped`] trait
pub fn to_wrapped_string<T: JsonWrapped>(t: &T) -> Result<String>
where
    <T as JsonWrapped>::JsonType: Serialize,
{
    let x = t.to_wrapped_json()?;
    Ok(serde_json::to_string(&x)?)
}

/// Generic helper function for the implementors of [`JsonWrapped`] trait
pub fn from_wrapped_value<T: JsonWrapped>(v: serde_json::Value) -> Result<T>
where
    <T as JsonWrapped>::JsonType: DeserializeOwned,
{
    let x: <T as JsonWrapped>::JsonType = serde_json::from_value(v)?;
    Ok(T::from_wrapped_json(&x)?)
}

/// Generic helper function for the implementors of [`JsonWrapped`] trait
pub fn to_wrapped_value<T: JsonWrapped>(t: T) -> Result<serde_json::Value>
where
    <T as JsonWrapped>::JsonType: Serialize,
{
    let x = t.to_wrapped_json()?;
    Ok(serde_json::to_value(x)?)
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

impl JsonWrapped for () {
    type JsonType = JsonUnit;

    fn to_wrapped_json(&self) -> Result<Self::JsonType> {
        Ok(JsonUnit::unit())
    }

    fn from_wrapped_json(_value: &Self::JsonType) -> Result<Self> {
        Ok(())
    }
}

/// Unambiguous representation of simple enums in JSON
///
/// Note that `From<JsonEnum<E>> for E` can't be implemented in this crate for external types in Rust.
/// Use [`JsonEnum::wrap`] instead.
///

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct JsonEnum<E> {
    __enum__: E,
}

impl<E: EncodeableEnum> JsonEnum<E> {
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

impl<E: EncodeableEnum> From<E> for JsonEnum<E> {
    fn from(e: E) -> Self {
        JsonEnum { __enum__: e }
    }
}

/// Marker trait for simple `enum`  types for auto-implementing the [`JsonWrapped`] trait
pub trait EncodeableEnum: Clone {}

impl<E: EncodeableEnum> JsonWrapped for E {
    type JsonType = JsonEnum<E>;

    fn to_wrapped_json(&self) -> Result<Self::JsonType> {
        Ok(JsonEnum {
            __enum__: self.clone(),
        })
    }

    fn from_wrapped_json(value: &Self::JsonType) -> Result<Self> {
        Ok(value.__enum__.clone())
    }
}

/// Macro to generate the trivial implementations for the [`JsonWrapped`] trait.
///
/// ## Examples
///
/// ```text
/// use zfx_michelson::json_wrapper;
///
/// json_wrapper!(String as Self);    // No wrapping necessary
/// json_wrapper!(Vec<T> as Self; T); // Ditto, but with generics
/// json_wrapper!(u64 as String);     // Wrapped version is a `String`
/// ```
#[macro_export]
macro_rules! json_wrapper {
    ($typ:ty as Self) => {
        impl $crate::JsonWrapped for $typ {
            type JsonType = $typ;

            fn to_wrapped_json(&self) -> $crate::Result<Self::JsonType> {
                Ok(self.clone())
            }

            fn from_wrapped_json(value: &Self::JsonType) -> $crate::Result<Self> {
                Ok(value.clone())
            }
        }
    };
    ($typ:ty as String) => {
        impl $crate::JsonWrapped for $typ {
            type JsonType = String;

            fn to_wrapped_json(&self) -> $crate::Result<String> {
                Ok(self.to_string())
            }

            fn from_wrapped_json(value: &String) -> $crate::Result<Self> {
                Ok(value.as_str().parse().map_err(|_| $crate::Error::EncodingError(format!("JsonWrapped: Unparseable string {:?}", value)))?)
            }
        }
    };
    ($typ:ty as Self; $($g:tt),+) => {
        impl<$($g:std::clone::Clone,)+> $crate::JsonWrapped for $typ {
            type JsonType = $typ;

            fn to_wrapped_json(&self) -> $crate::Result<Self::JsonType> {
                Ok(self.clone())
            }

            fn from_wrapped_json(value: &Self::JsonType) -> $crate::Result<Self> {
                Ok(value.clone())
            }
        }
    }
}

// For `Option` and `Vec`, we need to process the contained data
impl<T: JsonWrapped> JsonWrapped for Option<T> {
    type JsonType = Option<T::JsonType>;

    fn to_wrapped_json(&self) -> Result<Self::JsonType> {
        match self.as_ref() {
            None => Ok(None),
            Some(x) => Ok(Some(to_wrapped_json(x)?)),
        }
    }

    fn from_wrapped_json(value: &Self::JsonType) -> Result<Self> {
        match value.as_ref() {
            None => Ok(None),
            Some(x) => Ok(Some(from_wrapped_json(x)?)),
        }
    }
}

impl<T: JsonWrapped> JsonWrapped for Vec<T> {
    type JsonType = Vec<T::JsonType>;

    fn to_wrapped_json(&self) -> Result<Self::JsonType> {
        let mut v = vec![];
        for x in self.iter() {
            v.push(to_wrapped_json(x)?);
        }
        Ok(v)
    }

    fn from_wrapped_json(value: &Self::JsonType) -> Result<Self> {
        let mut v = vec![];
        for x in value.iter() {
            v.push(from_wrapped_json(x)?);
        }
        Ok(v)
    }
}

impl<T> JsonWrapped for HashSet<T>
where
    T: Eq + Hash + JsonWrapped,
    <T as JsonWrapped>::JsonType: Eq + Hash,
{
    type JsonType = HashSet<T::JsonType>;

    fn to_wrapped_json(&self) -> Result<Self::JsonType> {
        let mut h = HashSet::new();
        for x in self.iter() {
            let _ = h.insert(to_wrapped_json(x)?);
        }
        Ok(h)
    }

    fn from_wrapped_json(value: &Self::JsonType) -> Result<Self> {
        let mut h = HashSet::new();
        for x in value.iter() {
            let _ = h.insert(from_wrapped_json(x)?);
        }
        Ok(h)
    }
}

// Basic types
json_wrapper!(String as Self);

json_wrapper!(u8 as String);
json_wrapper!(u16 as String);
json_wrapper!(u32 as String);
json_wrapper!(u64 as String);
json_wrapper!(u128 as String);

json_wrapper!(i8 as String);
json_wrapper!(i16 as String);
json_wrapper!(i32 as String);
json_wrapper!(i64 as String);
json_wrapper!(i128 as String);

json_wrapper!(isize as String);
json_wrapper!(usize as String);

json_wrapper!(bool as Self);
json_wrapper!(char as Self);

/// Macro to derive the `JsonWrapped` trait  for Rust `struct`
///
/// All fields must already implement the trait for the macro to work.
///
/// ## Example
///
/// ```
/// use zfx_michelson::*;
///
/// // The source LIGO record `{ int: int; string: string }`
/// pub const SCHEMA: &str = r#"
///    { "prim": "pair", "args":
///            [ { "prim": "int", "annots": [ "%int" ] },
///              { "prim": "string", "annots": [ "%string" ] } ] } "#;
///
/// wrapped_struct! {
///     // creates a `pub struct`
///     Struct {
///        int: isize,
///        string: String,
///      }
///      // also a `pub struct`, usable independently
///      as WrappedStruct
///      // The matching Michelson schema
///      with_schema SCHEMA
/// }
/// ```
#[macro_export]
macro_rules! wrapped_struct{
    { $name:ident { $($field:ident : $type:ty),* } as $jname:ident with_schema $schema:expr} => {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
        pub struct $name {
           $($field: $type),+
        }
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
        pub struct $jname {
          $($field: <$type as $crate::JsonWrapped>::JsonType),*
        }

        impl $crate::JsonWrapped for $name {
            type JsonType = $jname;
            const SCHEMA_STR: &'static str = $schema;

            fn to_wrapped_json(&self) -> $crate::Result<Self::JsonType> {
                Ok($jname {
                   $($field: self.$field.to_wrapped_json()?),*
                })
            }

            fn from_wrapped_json(value: &Self::JsonType) -> $crate::Result<Self> {
               Ok($name {
                   $($field: <$type>::from_wrapped_json(&value.$field)?),*
                })
             }
        }

    };
    { $name:ident { $($field:ident : $type:ty),* } as $jname:ident} => {
        wrapped_struct!{ $name{ $($field : $type),* } as $jname with_schema "" }
    };
    { $name:ident { $($field:ident : $type:ty),* , } as $jname:ident} => {
        wrapped_struct!{ $name{ $($field : $type),* } as $jname with_schema ""}
    };
    { $name:ident { $($field:ident : $type:ty),* , } as $jname:ident with_schema $schema:expr} => {
        wrapped_struct!{ $name{ $($field : $type),* } as $jname with_schema $schema }
    };
}

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
        m: <HashMap<std::string::String, usize> as JsonWrapped>::JsonType,
        b: JsonBigNumber,
        i: String,
        s: String,
    }

    #[test]
    fn test_serialise_struct() {
        let s = S {
            u: JsonUnit::unit(),
            e: JsonEnum::wrap(E::A),
            m: MichelsonMap::new(),
            b: "42".to_owned(),
            i: "1".to_owned(),
            s: "foo".to_owned(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let expected = "{\"u\":{\"__unit__\":null},\"e\":{\"__enum__\":\"A\"},\
            \"m\":{\"MichelsonMap\":{}},\"b\":\"42\",\"i\":\"1\",\"s\":\"foo\"}";
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
            m: from_wrapped_json(&s.m).unwrap(),
            b: s.b.parse().unwrap(),
            i: s.i.parse().unwrap(),
            s: s.s,
        }
    }

    fn convert_struct_back(r: RustStruct) -> S {
        S {
            u: r.u.into(),
            e: JsonEnum::wrap(r.e),
            m: r.m.to_wrapped_json().unwrap(),
            b: r.b.to_string(),
            i: r.i.to_string(),
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
            i: "1".to_owned(),
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

    impl JsonWrapped for RustStruct {
        type JsonType = S;

        fn to_wrapped_json(&self) -> Result<S> {
            Ok(S {
                u: self.u.to_wrapped_json()?,
                e: self.e.to_wrapped_json()?,
                m: self.m.to_wrapped_json()?,
                b: self.b.to_wrapped_json()?,
                i: self.i.to_wrapped_json()?,
                s: to_wrapped_json(&self.s)?,
            })
        }
        fn from_wrapped_json(wrapped: &S) -> Result<Self> {
            Ok(Self {
                u: from_wrapped_json(&wrapped.u)?,
                e: from_wrapped_json(&wrapped.e)?,
                m: from_wrapped_json(&wrapped.m)?,
                b: from_wrapped_json(&wrapped.b)?,
                i: from_wrapped_json(&wrapped.i)?,
                s: from_wrapped_json(&wrapped.s)?,
            })
        }
    }

    impl EncodeableEnum for E {}

    #[test]
    fn test_rustify_via_trait() {
        let s = S {
            u: JsonUnit::unit(),
            e: JsonEnum::wrap(E::A),
            m: MichelsonMap::new(),
            b: "42".to_owned(),
            i: "1".to_owned(),
            s: "foo".to_owned(),
        };
        let r: RustStruct = from_wrapped_json(&s).unwrap();
        let expected = RustStruct {
            u: (),
            e: E::A,
            m: HashMap::new(),
            b: 42,
            i: 1,
            s: "foo".to_owned(),
        };
        assert_eq!(r, expected);
        let s2 = r.to_wrapped_json().unwrap();
        assert_eq!(s2, s);
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum EE {
        A,
        B(String, isize),
        C { x: String, y: isize },
    }
    json_wrapper!(EE as Self);

    #[test]
    fn test_complex_enum() {
        let a = EE::A;
        let b = EE::B("foo".to_owned(), 42);
        let c = EE::C {
            x: "foo".to_owned(),
            y: 42,
        };

        println!("{:?}", to_wrapped_json(&a));
        println!("{:?}", to_wrapped_json(&b));
        println!("{:?}", to_wrapped_json(&c));

        println!("{}", to_wrapped_string(&a).unwrap());
        println!("{}", to_wrapped_string(&b).unwrap());
        println!("{}", to_wrapped_string(&c).unwrap());
    }

    #[test]
    fn test_option() {
        let none: Option<String> = None;
        let some = Some(String::from("foo"));
        println!("{}", to_wrapped_string(&none).unwrap());
        println!("{}", to_wrapped_string(&some).unwrap());
    }

    wrapped_struct! { Stru { a:isize, b: String, c: bool } as WrappedStru }

    #[test]
    fn test_rustify_via_trait_and_macro() {
        let s = WrappedStru {
            a: "1".to_owned(),
            b: "foo".to_owned(),
            c: true,
        };
        let r: Stru = from_wrapped_json(&s).unwrap();
        let expected = Stru {
            a: 1,
            b: "foo".to_owned(),
            c: true,
        };
        assert_eq!(r, expected);
        let s2 = r.to_wrapped_json().unwrap();
        assert_eq!(s2, s);
    }
}
