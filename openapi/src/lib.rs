use indexmap::indexmap;
use types_openapi as openapi;

pub fn parse_openapi(openapi: &openapi::v3_0::OpenAPI) -> yaspec::YASpec {
    let mut entities = indexmap! {};
    let mut services = indexmap! {};

    let output = yaspec::YASpec {
        name: openapi.info.title.clone(),
        description: openapi.info.description.clone(),
        version: openapi.info.version.clone(),
        yaspec: env!("CARGO_PKG_VERSION").to_owned(),
        entities,
        services,
    };

    // openapi.components.as_ref().and_then(|components| {
    //     components.schemas.as_ref().map(|schemas| {
    //         schemas.iter().for_each(|(name, schema)| {
    //             // TODO
    //         });
    //     })
    // });

    output
}
