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

impl<T> Referenceable<T> {
    pub fn reference(self) -> Option<Reference> {
        match self {
            Referenceable::Reference(reference) => Some(reference),
            Referenceable::Value(_) => None,
        }
    }

    pub fn value(self) -> Option<T> {
        match self {
            Referenceable::Reference(_) => None,
            Referenceable::Value(value) => Some(value),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Referenceable<U> {
        match self {
            Referenceable::Reference(r) => Referenceable::Reference(r),
            Referenceable::Value(v) => Referenceable::Value(op(v)),
        }
    }
}
