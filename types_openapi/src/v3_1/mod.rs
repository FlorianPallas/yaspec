use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use types_jsonschema::draft_2020_12::*;

pub mod migrate;
pub mod util;
pub use util::*;

pub type Any = serde_json::Value;
pub type Map<K, V> = IndexMap<K, V>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    pub openapi: String,
    pub info: Info,
    pub json_schema_dialect: Option<String>,
    pub servers: Option<Vec<Server>>,
    pub paths: Map<String, Referenceable<PathItem>>,
    pub webhooks: Option<Map<String, Referenceable<PathItem>>>,
    pub components: Option<Components>,
    pub security: Option<Vec<SecurityRequirement>>,
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDocumentation>,
}

/// The object provides metadata about the API. The metadata MAY be used by the clients if needed, and MAY be presented in editing or documentation generation tools for convenience.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The title of the API.
    pub title: String,
    /// A short summary of the API.
    pub summary: Option<String>,
    /// A description of the API. [CommonMark](https://spec.openapis.org/oas/v3.1.0#bib-commonmark) syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API.
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    pub license: Option<License>,
    /// The version of the OpenAPI document (which is distinct from the [OpenAPI Specification version](https://spec.openapis.org/oas/v3.1.0#oasVersion) or the API implementation version).
    pub version: String,
}

/// Contact information for the exposed API.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// The identifying name of the contact person/organization.
    pub name: Option<String>,
    /// The URL pointing to the contact information.
    pub url: Option<String>,
    /// The email address of the contact person/organization.
    pub email: Option<String>,
}

/// License information for the exposed API.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// The license name used for the API.
    pub name: String,
    pub identifier: Option<String>,
    pub url: Option<String>,
}

/// An object representing a Server.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    /// A URL to the target host. This URL supports Server Variables and MAY be relative, to indicate that the host location is relative to the location where the OpenAPI document is being served. Variable substitutions will be made when a variable is named in {brackets}.
    pub url: String,
    /// An optional string describing the host designated by the URL. [CommonMark](https://spec.openapis.org/oas/v3.1.0#bib-commonmark) syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A map between a variable name and its value. The value is used for substitution in the server’s URL template.
    pub variables: Option<Map<String, ServerVariable>>,
}

/// An object representing a Server Variable for server URL template substitution.
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    /// An enumeration of string values to be used if the substitution options are from a limited set. The array MUST NOT be empty.
    pub enum_: Option<Vec<String>>,
    /// The default value to use for substitution, which SHALL be sent if an alternate value is not supplied. Note this behavior is different than the Schema Object’s treatment of default values, because in those cases parameter values are optional. If the enum is defined, the value MUST exist in the enum’s values.
    pub default: String,
    /// An optional description for the server variable. [CommonMark](https://spec.openapis.org/oas/v3.1.0#bib-commonmark) syntax MAY be used for rich text representation.
    pub description: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable [Schema Objects](https://spec.openapis.org/oas/v3.1.0#schema-object).
    pub schemas: Option<Map<String, Schema>>,
    /// An object to hold reusable [Response Objects](https://spec.openapis.org/oas/v3.1.0#response-object).
    pub responses: Option<Map<String, Referenceable<Response>>>,
    /// An object to hold reusable [Parameter Objects](https://spec.openapis.org/oas/v3.1.0#parameter-object).
    pub parameters: Option<Map<String, Referenceable<Parameter>>>,
    /// An object to hold reusable [Example Objects](https://spec.openapis.org/oas/v3.1.0#example-object).
    pub examples: Option<Map<String, Referenceable<Example>>>,
    /// An object to hold reusable [Request Body Objects](https://spec.openapis.org/oas/v3.1.0#request-body-object).
    pub request_bodies: Option<Map<String, Referenceable<RequestBody>>>,
    /// An object to hold reusable [Header Objects](https://spec.openapis.org/oas/v3.1.0#header-object).
    pub headers: Option<Map<String, Referenceable<Header>>>,
    /// An object to hold reusable [Security Scheme Objects](https://spec.openapis.org/oas/v3.1.0#security-scheme-object).
    pub security_schemes: Option<Map<String, Referenceable<SecurityScheme>>>,
    /// An object to hold reusable [Link Objects](https://spec.openapis.org/oas/v3.1.0#link-object).
    pub links: Option<Map<String, Referenceable<Link>>>,
    /// An object to hold reusable [Callback Objects](https://spec.openapis.org/oas/v3.1.0#callback-object).
    pub callbacks: Option<Map<String, Referenceable<Callback>>>,
    /// An object to hold reusable [Path Item Objects](https://spec.openapis.org/oas/v3.1.0#path-item-object).
    pub path_items: Option<Map<String, Referenceable<PathItem>>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub trace: Option<Operation>,
    pub servers: Option<Vec<Server>>,
    pub parameters: Option<Vec<Referenceable<Parameter>>>,
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
    pub security: Option<Vec<SecurityRequirement>>,
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
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
    pub schema: Option<Schema>,
    pub content: Option<Map<String, MediaType>>,
    pub example: Option<Any>,
    pub examples: Option<Map<String, Referenceable<Example>>>,
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
    pub content: Option<Map<String, MediaType>>,
    pub example: Option<Any>,
    pub examples: Option<Map<String, Referenceable<Example>>>,
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
#[serde(rename_all = "camelCase")]
pub struct Reference {
    #[serde(rename = "$ref")]
    pub path: String,
    pub summary: Option<String>,
    pub description: Option<String>,
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

pub type SecurityRequirement = Map<String, Vec<String>>;
