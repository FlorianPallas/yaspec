use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Reference;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Referenceable<T> {
    Reference(Reference),
    Value(T),
}
