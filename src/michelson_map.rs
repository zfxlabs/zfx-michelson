//! This module contains the `MichelsonMap` type and associated functionality

use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// `MichelsonMap` is a simple newtype over `HashMap` with a different serialisation format.
///
/// `MichelsonMap` behaves exactly like a `HashMap`, except it is serialised tagged as `MichelsonMap`.
/// `MichelsonMap` is intended as the Rust counterpart of [Taquito](https://tezostaquito.io/)'s `MichelsonMap`
///  class, in order to ensure seamless integration with it.
///
///  ## Example
///
///  ```
///  # use zfx_michelson::MichelsonMap;
///  # use std::collections::HashMap;
///  use serde_json;
///
///  // `HashMap`
///  let mut map = HashMap::new();
///  map.insert(String::from("foo"), 42);
///  let json = serde_json::to_string(&map).unwrap();
///  assert_eq!(json, "{\"foo\":42}");
///
///  /// `MichelsonMap`
///  let mut map = MichelsonMap::new();
///  map.insert(String::from("foo"), 42);
///  let json = serde_json::to_string(&map).unwrap();
///  assert_eq!(json, "{\"MichelsonMap\":{\"foo\":42}}"); // !!!
///  ```
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    /// The wrapped `HashMap`
    #[serde(rename = "MichelsonMap")]
    inner: HashMap<K, V>,
}

impl<K, V> MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    /// Creates an empty `MichelsonMap`.
    pub fn new() -> Self {
        MichelsonMap {
            inner: HashMap::new(),
        }
    }
}

impl<K, V> fmt::Debug for MichelsonMap<K, V>
where
    K: fmt::Debug + PartialEq + Eq + Hash,
    V: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_map().entry(&"MichelsonMap", &self.inner).finish()
    }
}

impl<K, V> Clone for MichelsonMap<K, V>
where
    K: Clone + PartialEq + Eq + Hash,
    V: Clone,
{
    fn clone(&self) -> Self {
        MichelsonMap {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> Deref for MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, V> From<HashMap<K, V>> for MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn from(inner: HashMap<K, V>) -> Self {
        MichelsonMap { inner }
    }
}

impl<K, V> From<MichelsonMap<K, V>> for HashMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn from(m: MichelsonMap<K, V>) -> Self {
        m.inner
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn from(arr: [(K, V); N]) -> Self {
        MichelsonMap { inner: arr.into() }
    }
}

impl<K, V> Default for MichelsonMap<K, V>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        MichelsonMap {
            inner: HashMap::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn test_basic_serialise() {
        let m: MichelsonMap<_, _> = [("a", 1), ("b", 2)].into();
        let json = serde_json::to_string(&m).unwrap();
        let expected_a = "{\"MichelsonMap\":{\"a\":1,\"b\":2}}";
        let expected_b = "{\"MichelsonMap\":{\"b\":2,\"a\":1}}";
        println!("{:?}", json);
        assert!(json == expected_a || json == expected_b);
    }

    #[test]
    fn test_behaves_as_hashmap() {
        let mut m = MichelsonMap::default();
        assert_eq!(None, m.insert(1, 1));
        m.extend([(2, 2), (3, 3)]);
        println!("{:?}", m);
        let json = serde_json::to_string(&m).unwrap();
        println!("{:?}", json);
    }
}
