//! Various JSON-serialisable that need special encoding logic
use serde::{Deserialize, Serialize};

/// Unambiguous representation of `()` in JSON
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonUnit {
   __unit__: ()
}

impl JsonUnit {
   pub fn unit() -> Self {
       JsonUnit { __unit__: () }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleEnumWrapper<E> {
   pub __enum__: E
}
