use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub mod util;
pub use util::*;

pub type Any = serde_json::Value;
pub type Number = serde_json::Number;
pub type Map<K, V> = IndexMap<K, V>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<Vec<Server>>,
    pub paths: Option<Map<String, Referenceable<PathItem>>>,
    pub components: Option<Components>,
    pub security: Option<Vec<SecurityScheme>>,
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<Map<String, ServerVariable>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    pub enum_: Option<Vec<String>>,
    pub default: String,
    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: Option<Map<String, Referenceable<Schema>>>,
    pub responses: Option<Map<String, Referenceable<Response>>>,
    pub parameters: Option<Map<String, Referenceable<Parameter>>>,
    pub examples: Option<Map<String, Referenceable<Example>>>,
    pub request_bodies: Option<Map<String, Referenceable<RequestBody>>>,
    pub headers: Option<Map<String, Referenceable<Header>>>,
    pub security_schemes: Option<Map<String, Referenceable<SecurityScheme>>>,
    pub links: Option<Map<String, Referenceable<Link>>>,
    pub callbacks: Option<Map<String, Referenceable<Callback>>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub servers: Option<Vec<Server>>,
    pub parameters: Option<Vec<Referenceable<Parameter>>>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<Referenceable<Parameter>>>,
    pub request_body: Option<Referenceable<RequestBody>>,
    pub responses: Responses,
    pub callbacks: Option<Map<String, Referenceable<Callback>>>,
    pub deprecated: Option<bool>,
    pub security: Option<Vec<Map<String, Vec<String>>>>,
    pub servers: Option<Vec<Server>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocumentation {
    pub description: Option<String>,
    pub url: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub in_: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub schema: Option<Schema>,
    pub example: Option<Any>,
    pub examples: Option<Map<String, Referenceable<Example>>>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub content: Option<Map<String, MediaType>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: Map<String, MediaType>,
    pub required: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    pub schema: Option<Referenceable<Schema>>,
    pub example: Option<Any>,
    pub examples: Option<Map<String, Referenceable<Example>>>,
    pub encoding: Option<Map<String, Encoding>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    pub content_type: Option<String>,
    pub headers: Option<Map<String, Referenceable<Header>>>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Responses {
    pub default: Option<Referenceable<Response>>,
    #[serde(flatten)]
    pub items: Map<String, Referenceable<Response>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub description: String,
    pub headers: Option<Map<String, Referenceable<Header>>>,
    pub content: Option<Map<String, MediaType>>,
    pub links: Option<Map<String, Referenceable<Link>>>,
}

pub type Callback = Map<String, Referenceable<PathItem>>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: Option<Any>,
    pub external_value: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub operation_ref: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<Map<String, Any>>,
    pub request_body: Option<Any>,
    pub description: Option<String>,
    pub server: Option<Server>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<Schema>,
    pub example: Option<Any>,
    pub examples: Option<Map<String, Referenceable<Example>>>,
    pub content: Option<Map<String, MediaType>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub path: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub title: Option<String>,
    pub multiple_of: Option<Number>,
    pub maximum: Option<Number>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<Number>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<Number>,
    pub min_length: Option<Number>,
    pub pattern: Option<String>,
    pub max_items: Option<Number>,
    pub min_items: Option<Number>,
    pub unique_items: Option<bool>,
    pub max_properties: Option<Number>,
    pub min_properties: Option<Number>,
    pub required: Option<Vec<String>>,
    pub enum_: Option<Vec<Any>>,
    pub type_: Option<SchemaType>,
    pub not: Option<Box<Referenceable<Schema>>>,
    pub all_of: Option<Vec<Referenceable<Schema>>>,
    pub one_of: Option<Vec<Referenceable<Schema>>>,
    pub any_of: Option<Vec<Referenceable<Schema>>>,
    pub items: Option<Box<Referenceable<Schema>>>,
    pub properties: Option<Map<String, Referenceable<Schema>>>,
    pub additional_properties: Option<BoolOr<Box<Schema>>>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub default: Option<Any>,
    pub nullable: Option<bool>,
    pub discriminator: Option<Discriminator>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub example: Option<Any>,
    pub external_docs: Option<ExternalDocumentation>,
    pub deprecated: Option<bool>,
    pub xml: Option<XML>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SchemaType {
    Array,
    Boolean,
    Integer,
    Number,
    Object,
    String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    pub property_name: String,
    pub mapping: Option<Map<String, String>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XML {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapped: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    pub type_: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub in_: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: Option<OAuthFlows>,
    pub open_id_connect_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    pub client_credentials: Option<OAuthFlow>,
    pub authorization_code: Option<OAuthFlow>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlow {
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: Map<String, String>,
}

pub type SecurityRequirement = Map<String, Option<Vec<String>>>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BoolOr<T> {
    Bool(bool),
    Value(T),
}
