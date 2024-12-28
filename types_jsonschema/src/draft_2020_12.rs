use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use serde_with::formats::PreferMany;
use serde_with::{serde_as, skip_serializing_none, OneOrMany};

pub type Map<K, V> = indexmap::IndexMap<K, V>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    // Basics
    #[serde_as(as = "Option<OneOrMany<_, PreferMany>>")]
    pub type_: Option<Vec<SchemaType>>,
    pub format: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,

    // Declaring a Dialect
    // https://json-schema.org/understanding-json-schema/reference/schema
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(rename = "$id")]
    pub id: Option<String>,
    #[serde(rename = "$vocabulary")]
    pub vocabulary: Option<Map<String, Value>>,
    #[serde(rename = "$dynamicAnchor")]
    pub dynamic_anchor: Option<String>,
    #[serde(rename = "$anchor")]
    pub anchor: Option<String>,
    #[serde(rename = "$defs")]
    pub defs: Option<Map<String, Schema>>,

    // Annotations
    // https://json-schema.org/understanding-json-schema/reference/annotations
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub examples: Option<Vec<Value>>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub deprecated: Option<bool>,

    // Comments
    // https://json-schema.org/understanding-json-schema/reference/comments
    #[serde(rename = "$comment")]
    pub comment: Option<String>,

    // Enumerated Values
    // https://json-schema.org/understanding-json-schema/reference/enum
    pub enum_: Option<Vec<Value>>,

    // Constant Values
    // https://json-schema.org/understanding-json-schema/reference/const
    pub const_: Option<Value>,

    // Media
    // https://json-schema.org/understanding-json-schema/reference/non_json_data
    pub content_media_type: Option<String>,
    pub content_encoding: Option<String>,
    pub content_schema: Option<Box<Schema>>,

    // Composition
    // https://json-schema.org/understanding-json-schema/reference/combining
    pub all_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub one_of: Option<Vec<Schema>>,
    pub not: Option<Box<Schema>>,

    // Conditional Schemas
    // https://json-schema.org/understanding-json-schema/reference/conditionals
    pub dependent_required: Option<Map<String, Vec<String>>>,
    pub dependent_schemas: Option<Map<String, Schema>>,
    pub if_: Option<Box<Schema>>,
    pub then: Option<Box<Schema>>,
    pub else_: Option<Box<Schema>>,

    // String
    // https://json-schema.org/understanding-json-schema/reference/string
    pub min_length: Option<Number>,
    pub max_length: Option<Number>,
    pub pattern: Option<String>,

    // Numeric
    // https://json-schema.org/understanding-json-schema/reference/numeric
    pub multiple_of: Option<Number>,
    pub maximum: Option<Number>,
    pub exclusive_maximum: Option<Number>,
    pub minimum: Option<Number>,
    pub exclusive_minimum: Option<Number>,

    // Object
    // https://json-schema.org/understanding-json-schema/reference/object
    pub properties: Option<Map<String, Schema>>,
    pub pattern_properties: Option<Map<String, Schema>>,
    pub additional_properties: Option<BoolOr<Box<Schema>>>,
    pub unevaluated_properties: Option<BoolOr<Box<Schema>>>,
    pub required: Option<Vec<String>>,
    pub property_names: Option<Box<Schema>>,
    pub max_properties: Option<Number>,
    pub min_properties: Option<Number>,

    // Array
    // https://json-schema.org/understanding-json-schema/reference/array
    pub items: Option<BoolOr<Box<Schema>>>,
    pub prefix_items: Option<Vec<Schema>>,
    pub unevaluated_items: Option<BoolOr<Box<Schema>>>,
    pub contains: Option<Box<Schema>>,
    pub min_contains: Option<Number>,
    pub max_contains: Option<Number>,
    pub max_items: Option<Number>,
    pub min_items: Option<Number>,
    pub unique_items: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum BoolOr<T> {
    Bool(bool),
    Value(T),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Object,
    Array,
    Null,
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum StringFormat {
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "duration")]
    Duration,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "idn-email")]
    IdnEmail,
    #[serde(rename = "hostname")]
    Hostname,
    #[serde(rename = "idn-hostname")]
    IdnHostname,
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
    #[serde(rename = "uuid")]
    Uuid,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "uri-reference")]
    UriReference,
    #[serde(rename = "iri")]
    Iri,
    #[serde(rename = "iri-reference")]
    IriReference,
    #[serde(rename = "uri-template")]
    UriTemplate,
    #[serde(rename = "json-pointer")]
    JsonPointer,
    #[serde(rename = "relative-json-pointer")]
    RelativeJsonPointer,
    #[serde(rename = "regex")]
    Regex,
}
