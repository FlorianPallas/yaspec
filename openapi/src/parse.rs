// use std::iter;

// use yaspec::util::WordCaseExt;

// pub fn to_entity(
//     path: Vec<String>,
//     schema: &json::Schema,
//     entities: &mut IndexMap<String, yaspec::Entry<yaspec::Entity>>,
// ) -> yaspec::Entry<yaspec::Schema> {
//     let type_ = schema.type_.as_ref().expect("expected type present");
//     assert!(!type_.is_empty());

//     // TODO: const

//     let description = schema.description.clone();
//     let mut child_entities: IndexMap<String, yaspec::Entry<yaspec::Entity>> =
//         IndexMap::new();

//     let schema = if type_.len() == 1 {
//         let type_ = type_.get(0).unwrap();

//         match type_ {
//             json::SchemaType::String
//             | json::SchemaType::Number
//             | json::SchemaType::Integer
//             | json::SchemaType::Boolean
//             | json::SchemaType::Null
//             | json::SchemaType::Any
//             | json::SchemaType::Array => yaspec::Schema::Alias {
//                 shape: to_shape(path, schema, entities),
//             },
//             json::SchemaType::Object => yaspec::Schema::Record {
//                 fields: schema
//                     .properties
//                     .as_ref()
//                     .map(|properties| {
//                         properties
//                             .into_iter()
//                             .map(|(name, field_schema)| {
//                                 let is_required = schema
//                                     .required
//                                     .as_ref()
//                                     .map(|required| required.contains(&name))
//                                     .unwrap_or_default();

//                                 // openapi does not enforce titles on nested schemas, so we
//                                 // need to generate a title based on the field name, or use
//                                 // the title if it is present (ideal case)
//                                 let path = if let Some(ref title) = field_schema.title {
//                                     vec![title.to_owned()]
//                                 } else {
//                                     path.clone()
//                                         .into_iter()
//                                         .chain(iter::once(name.to_owned()))
//                                         .collect::<Vec<_>>()
//                                 };

//                                 (
//                                     name.to_owned(),
//                                     to_field(path, field_schema, entities, !is_required),
//                                 )
//                             })
//                             .collect::<IndexMap<String, yaspec::Entry<yaspec::Shape>>>()
//                     })
//                     .unwrap_or_default(),
//             },
//         }
//     } else {
//         let schema = yaspec::Schema::Union {
//             fields: type_
//                 .iter()
//                 .map(|t| {
//                     (
//                         t.to_string(),
//                         yaspec::Entry {
//                             inner: to_shape(
//                                 path.clone(),
//                                 &json::Schema {
//                                     type_: Some(vec![t.clone()]),
//                                     ..Default::default()
//                                 },
//                                 &mut child_entities,
//                             ),
//                             description: None,
//                             metadata: indexmap! {},
//                         },
//                     )
//                 })
//                 .collect(),
//         };

//         schema
//     };

//     entities.extend(child_entities.into_iter());

//     yaspec::Entry {
//         description,
//         metadata: indexmap! {},
//         inner: yaspec::Entity { schema },
//     }
// }

// pub fn to_field(
//     path: Vec<String>,
//     schema: &json::Schema,
//     entities: &mut IndexMap<String, yaspec::Entry<yaspec::Entity>>,
//     nullable: bool,
// ) -> yaspec::Entry<yaspec::Shape> {
//     // yaspec uses a wrapper type to represent nullable fields
//     let shape = if nullable {
//         yaspec::Shape::Nullable {
//             inner: Box::new(to_shape(path, schema, entities)),
//         }
//     } else {
//         to_shape(path, schema, entities)
//     };

//     yaspec::Entry {
//         inner: shape,
//         description: schema.description.clone(),
//         metadata: indexmap! {},
//     }
// }

// pub fn to_shape(
//     path: Vec<String>,
//     schema: &json::Schema,
//     entities: &mut IndexMap<String, yaspec::Entry<yaspec::Entity>>,
// ) -> yaspec::Shape {
//     println!("to_shape {:?} {:?}", path, schema.type_);

//     if let Some(ref path) = schema.ref_ {
//         return yaspec::Shape::Entity {
//             // TODO: support proper path resolution
//             target: path.replace("#/components/schemas/", ""),
//         };
//     }

//     let type_ = schema
//         .type_
//         .as_ref()
//         .expect("cannot infer shape from schema without type");
//     assert!(!type_.is_empty());

//     if type_.len() == 1 {
//         let type_ = type_.get(0).unwrap();

//         match type_ {
//             json::SchemaType::String => yaspec::Shape::String,
//             json::SchemaType::Number => yaspec::Shape::Float,
//             json::SchemaType::Integer => yaspec::Shape::Int,
//             json::SchemaType::Boolean => yaspec::Shape::Bool,
//             json::SchemaType::Object => {
//                 let target = path.to_pascal_case();
//                 let entity = to_entity(path, schema, entities);
//                 entities.insert(target.clone(), entity);
//                 yaspec::Shape::Entity { target }
//             }
//             json::SchemaType::Array => {
//                 let items = schema.items.as_ref().expect("expected items present");
//                 let json::BoolOr::Value(items) = items else {
//                     panic!("Expected items to be a value");
//                 };
//                 yaspec::Shape::List {
//                     inner: Box::new(to_shape(path, items, entities)),
//                 }
//             }
//             json::SchemaType::Null => unimplemented!(),
//             json::SchemaType::Any => unimplemented!(),
//         }
//     } else {
//         if type_.contains(&json::SchemaType::Null) {
//             let mut inner_schema: json::Schema = schema.clone();
//             inner_schema.type_ = inner_schema.type_.map(|x| {
//                 x.into_iter()
//                     .filter(|x| x != &json::SchemaType::Null)
//                     .collect::<Vec<_>>()
//             });
//             return yaspec::Shape::Nullable {
//                 inner: Box::new(to_shape(path, &inner_schema, entities)),
//             };
//         } else {
//             let target = path.to_pascal_case();
//             let entity = to_entity(path, schema, entities);
//             entities.insert(target.clone(), entity);
//             return yaspec::Shape::Entity { target };
//         }
//     }
// }
