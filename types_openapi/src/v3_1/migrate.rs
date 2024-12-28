use crate::{v3_0, v3_1};
use indexmap::IndexMap;

pub trait MigrateExt {
    fn to_v3_1(self) -> v3_1::OpenAPI;
}

/// Convert an OpenAPI 3.0.3 document to OpenAPI 3.1.0

impl MigrateExt for v3_0::OpenAPI {
    fn to_v3_1(self) -> v3_1::OpenAPI {
        assert!(self.openapi.starts_with("3.0"));

        v3_1::OpenAPI {
            openapi: "3.1.0".to_owned(),
            info: self.info.into(),
            servers: self
                .servers
                .map(|servers| servers.into_iter().map(Into::into).collect()),
            paths: self.paths.map(IndexMapInto::into).unwrap_or_default(),
            components: self.components.map(Into::into),
            security: None,
            tags: self
                .tags
                .map(|tags| tags.into_iter().map(Into::into).collect()),
            external_docs: None,
            json_schema_dialect: None,
            webhooks: None,
        }
    }
}

impl From<v3_0::Info> for v3_1::Info {
    fn from(value: v3_0::Info) -> Self {
        Self {
            title: value.title,
            summary: None,
            description: value.description,
            terms_of_service: value.terms_of_service,
            contact: value.contact.map(Into::into),
            license: value.license.map(Into::into),
            version: value.version,
        }
    }
}

impl From<v3_0::Contact> for v3_1::Contact {
    fn from(value: v3_0::Contact) -> Self {
        Self {
            name: value.name,
            url: value.url,
            email: value.email,
        }
    }
}

impl From<v3_0::License> for v3_1::License {
    fn from(value: v3_0::License) -> Self {
        Self {
            name: value.name,
            url: value.url,
            identifier: None,
        }
    }
}

impl From<v3_0::Server> for v3_1::Server {
    fn from(value: v3_0::Server) -> Self {
        Self {
            url: value.url,
            description: value.description,
            variables: value.variables.map(|variables| {
                variables
                    .into_iter()
                    .map(|(name, variable)| (name, variable.into()))
                    .collect()
            }),
        }
    }
}

impl From<v3_0::ServerVariable> for v3_1::ServerVariable {
    fn from(value: v3_0::ServerVariable) -> Self {
        Self {
            enum_: value.enum_,
            default: value.default,
            description: value.description,
        }
    }
}

impl From<v3_0::Components> for v3_1::Components {
    fn from(value: v3_0::Components) -> Self {
        Self {
            schemas: value.schemas.map(IndexMapInto::into),
            responses: value.responses.map(IndexMapInto::into),
            parameters: value.parameters.map(IndexMapInto::into),
            examples: value.examples.map(IndexMapInto::into),
            request_bodies: value.request_bodies.map(IndexMapInto::into),
            headers: value.headers.map(IndexMapInto::into),
            security_schemes: value.security_schemes.map(IndexMapInto::into),
            links: value.links.map(IndexMapInto::into),
            callbacks: value.callbacks.map(|c| {
                c.into_iter()
                    .map(|(a, b)| (a, b.map(IndexMapInto::into).into()))
                    .collect()
            }),
            path_items: None,
        }
    }
}

impl From<v3_0::PathItem> for v3_1::PathItem {
    fn from(value: v3_0::PathItem) -> Self {
        Self {
            summary: value.summary,
            description: value.description,
            get: value.get.map(Into::into),
            put: value.put.map(Into::into),
            post: value.post.map(Into::into),
            delete: value.delete.map(Into::into),
            options: value.options.map(Into::into),
            head: value.head.map(Into::into),
            patch: value.patch.map(Into::into),
            trace: value.trace.map(Into::into),
            servers: value
                .servers
                .map(|servers| servers.into_iter().map(Into::into).collect()),
            parameters: value
                .parameters
                .map(|parameters| parameters.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<v3_0::Operation> for v3_1::Operation {
    fn from(value: v3_0::Operation) -> Self {
        Self {
            tags: value.tags,
            summary: value.summary,
            description: value.description,
            external_docs: value.external_docs.map(Into::into),
            operation_id: value.operation_id,
            parameters: value
                .parameters
                .map(|parameters| parameters.into_iter().map(Into::into).collect()),
            request_body: value.request_body.map(Into::into),
            responses: value.responses.into(),
            callbacks: value.callbacks.map(|c| {
                c.into_iter()
                    .map(|(a, b)| (a, b.map(IndexMapInto::into).into()))
                    .collect()
            }),
            deprecated: value.deprecated,
            security: value.security,
            servers: value
                .servers
                .map(|servers| servers.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<v3_0::ExternalDocumentation> for v3_1::ExternalDocumentation {
    fn from(value: v3_0::ExternalDocumentation) -> Self {
        Self {
            description: value.description,
            url: value.url,
        }
    }
}

impl From<v3_0::Parameter> for v3_1::Parameter {
    fn from(value: v3_0::Parameter) -> Self {
        Self {
            name: value.name,
            in_: value.in_,
            description: value.description,
            required: value.required,
            deprecated: value.deprecated,
            allow_empty_value: value.allow_empty_value,
            style: value.style,
            explode: value.explode,
            allow_reserved: value.allow_reserved,
            schema: value.schema.map(Into::into),
            example: value.example,
            examples: value.examples.map(IndexMapInto::into),
            content: value.content.map(IndexMapInto::into),
        }
    }
}

impl From<v3_0::RequestBody> for v3_1::RequestBody {
    fn from(value: v3_0::RequestBody) -> Self {
        Self {
            description: value.description,
            content: IndexMapInto::into(value.content),
            required: value.required,
        }
    }
}

impl From<v3_0::MediaType> for v3_1::MediaType {
    fn from(value: v3_0::MediaType) -> Self {
        Self {
            schema: value.schema.map(Into::into),
            example: value.example,
            examples: value.examples.map(IndexMapInto::into),
            encoding: value.encoding.map(IndexMapInto::into),
        }
    }
}

impl From<v3_0::Encoding> for v3_1::Encoding {
    fn from(value: v3_0::Encoding) -> Self {
        Self {
            content_type: value.content_type,
            headers: value.headers.map(IndexMapInto::into),
            style: value.style,
            explode: value.explode,
            allow_reserved: value.allow_reserved,
        }
    }
}

impl From<v3_0::Responses> for v3_1::Responses {
    fn from(value: v3_0::Responses) -> Self {
        Self {
            default: value.default.map(Into::into),
            items: IndexMapInto::into(value.items),
        }
    }
}

impl From<v3_0::Response> for v3_1::Response {
    fn from(value: v3_0::Response) -> Self {
        Self {
            description: value.description,
            headers: value.headers.map(IndexMapInto::into),
            content: value.content.map(IndexMapInto::into),
            links: value.links.map(IndexMapInto::into),
        }
    }
}

impl From<v3_0::Example> for v3_1::Example {
    fn from(value: v3_0::Example) -> Self {
        Self {
            summary: value.summary,
            description: value.description,
            value: value.value,
            external_value: value.external_value,
        }
    }
}

impl From<v3_0::Link> for v3_1::Link {
    fn from(value: v3_0::Link) -> Self {
        Self {
            operation_ref: value.operation_ref,
            operation_id: value.operation_id,
            parameters: value.parameters.map(IndexMapInto::into),
            request_body: value.request_body.map(Into::into),
            description: value.description,
            server: value.server.map(Into::into),
        }
    }
}

impl From<v3_0::Header> for v3_1::Header {
    fn from(value: v3_0::Header) -> Self {
        Self {
            description: value.description,
            required: value.required,
            deprecated: value.deprecated,
            allow_empty_value: value.allow_empty_value,
            style: value.style,
            explode: value.explode,
            allow_reserved: value.allow_reserved,
            schema: value.schema.map(Into::into),
            example: value.example,
            examples: value.examples.map(IndexMapInto::into),
            content: value.content.map(IndexMapInto::into),
        }
    }
}

impl From<v3_0::Tag> for v3_1::Tag {
    fn from(value: v3_0::Tag) -> Self {
        Self {
            name: value.name,
            description: value.description,
            external_docs: value.external_docs.map(Into::into),
        }
    }
}

impl From<v3_0::Reference> for v3_1::Reference {
    fn from(value: v3_0::Reference) -> Self {
        Self {
            path: value.path,
            summary: None,
            description: None,
        }
    }
}

impl From<v3_0::Referenceable<v3_0::Schema>> for v3_1::Schema {
    fn from(value: v3_0::Referenceable<v3_0::Schema>) -> Self {
        match value {
            v3_0::Referenceable::Reference(r) => Self {
                ref_: Some(r.path),
                ..Default::default()
            },
            v3_0::Referenceable::Value(v) => v.into(),
        }
    }
}

impl From<v3_0::Schema> for v3_1::Schema {
    fn from(value: v3_0::Schema) -> Self {
        let mut schema = v3_1::Schema {
            format: value.format,
            title: value.title,
            description: value.description,
            default: value.default,
            examples: value.example.map(|e| vec![e]), // convert single example to array
            read_only: value.read_only,
            write_only: value.write_only,
            deprecated: value.deprecated,
            enum_: value.enum_,
            all_of: value
                .all_of
                .map(|s| s.into_iter().map(Into::into).collect()),
            any_of: value
                .any_of
                .map(|s| s.into_iter().map(Into::into).collect()),
            one_of: value
                .one_of
                .map(|s| s.into_iter().map(Into::into).collect()),
            not: value.not.map(|s| Box::new(s.map(Into::into).into())),
            min_length: value.min_length,
            max_length: value.max_length,
            pattern: value.pattern,
            multiple_of: value.multiple_of,
            properties: value.properties.map(IndexMapInto::into),
            additional_properties: None, // TODO: convert additionalProperties
            required: value.required,
            max_properties: value.max_properties,
            min_properties: value.min_properties,
            items: value
                .items
                .map(|s| Box::new(s.map(Into::into).into()))
                .map(|s| v3_1::BoolOr::Value(s)),
            max_items: value.max_items,
            min_items: value.min_items,
            unique_items: value.unique_items,
            ..Default::default()
        };

        // convert nullable type to type array
        if let Some(t) = value.type_ {
            let t = t.into();
            if value.nullable.unwrap_or_default() {
                schema.type_ = Some(vec![t, v3_1::SchemaType::Null]);
            } else {
                schema.type_ = Some(vec![t]);
            }
        }

        // convert single enum value to const
        if let Some(ref mut values) = schema.enum_ {
            if values.len() == 1 {
                let v = values.pop().unwrap();
                schema.enum_ = None;
                schema.const_ = Some(v);
            }
        }

        // convert exclusiveMaximum and exclusiveMinimum
        if value.exclusive_maximum.unwrap_or_default() {
            schema.exclusive_maximum = value.maximum;
        } else {
            schema.maximum = value.maximum;
        }
        if value.exclusive_minimum.unwrap_or_default() {
            schema.exclusive_minimum = value.minimum;
        } else {
            schema.minimum = value.minimum;
        }

        schema
    }
}

impl From<v3_0::SchemaType> for v3_1::SchemaType {
    fn from(value: v3_0::SchemaType) -> Self {
        match value {
            v3_0::SchemaType::Boolean => v3_1::SchemaType::Boolean,
            v3_0::SchemaType::Object => v3_1::SchemaType::Object,
            v3_0::SchemaType::Array => v3_1::SchemaType::Array,
            v3_0::SchemaType::Number => v3_1::SchemaType::Number,
            v3_0::SchemaType::String => v3_1::SchemaType::String,
            v3_0::SchemaType::Integer => v3_1::SchemaType::Integer,
        }
    }
}

impl From<v3_0::SecurityScheme> for v3_1::SecurityScheme {
    fn from(value: v3_0::SecurityScheme) -> Self {
        Self {
            type_: value.type_,
            description: value.description,
            name: value.name,
            in_: value.in_,
            scheme: value.scheme,
            bearer_format: value.bearer_format,
            flows: value.flows.map(Into::into),
            open_id_connect_url: value.open_id_connect_url,
        }
    }
}

impl From<v3_0::OAuthFlows> for v3_1::OAuthFlows {
    fn from(value: v3_0::OAuthFlows) -> Self {
        Self {
            implicit: value.implicit.map(Into::into),
            password: value.password.map(Into::into),
            client_credentials: value.client_credentials.map(Into::into),
            authorization_code: value.authorization_code.map(Into::into),
        }
    }
}

impl From<v3_0::OAuthFlow> for v3_1::OAuthFlow {
    fn from(value: v3_0::OAuthFlow) -> Self {
        Self {
            authorization_url: value.authorization_url,
            token_url: value.token_url,
            refresh_url: value.refresh_url,
            scopes: value.scopes,
        }
    }
}

pub trait IndexMapInto<T> {
    fn into(self) -> T;
}

impl<T, U: Into<T>> IndexMapInto<IndexMap<String, T>> for IndexMap<String, U> {
    fn into(self) -> IndexMap<String, T> {
        self.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
}

impl<T: Into<U>, U> From<v3_0::Referenceable<T>> for v3_1::Referenceable<U> {
    fn from(value: v3_0::Referenceable<T>) -> Self {
        match value {
            v3_0::Referenceable::Reference(r) => v3_1::Referenceable::Reference(r.into()),
            v3_0::Referenceable::Value(v) => v3_1::Referenceable::Value(v.into()),
        }
    }
}

impl<T: Into<U>, U> From<v3_0::BoolOr<T>> for v3_1::BoolOr<U> {
    fn from(value: v3_0::BoolOr<T>) -> Self {
        match value {
            v3_0::BoolOr::Bool(b) => v3_1::BoolOr::Bool(b),
            v3_0::BoolOr::Value(v) => v3_1::BoolOr::Value(v.into()),
        }
    }
}
