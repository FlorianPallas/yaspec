use derive_more::derive::Deref;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Literal>),
    Object(Map<String, Literal>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Deref)]
pub struct Entry<T> {
    pub description: Option<String>,
    #[serde(default)]
    pub metadata: Map<String, Literal>,
    #[deref]
    #[serde(flatten)]
    pub inner: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct YASpec {
    pub yaspec: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    #[serde(default)]
    pub entities: Map<String, Entry<Schema>>,
    #[serde(default)]
    pub services: Map<String, Entry<Service>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum Schema {
    Record { fields: Map<String, Entry<Shape>> },
    Union { fields: Map<String, Entry<Shape>> },
    Enum { fields: Map<String, Entry<()>> },
    Alias { shape: Shape },
}

impl Default for Schema {
    fn default() -> Self {
        Schema::Record {
            fields: Map::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum Shape {
    Bool,
    Int,
    Float,
    String,
    List { inner: Box<Shape> },
    Map { key: Box<Shape>, value: Box<Shape> },
    Nullable { inner: Box<Shape> },
    Entity { target: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Service {
    #[serde(default)]
    pub actions: Map<String, Entry<Action>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Action {
    #[serde(default)]
    pub inputs: IndexMap<String, Entry<Shape>>,
    #[serde(default)]
    pub outputs: IndexMap<String, Entry<Option<Shape>>>,
}
