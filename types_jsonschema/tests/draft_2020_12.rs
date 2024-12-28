use insta::assert_ron_snapshot;
use types_jsonschema::draft_2020_12::Schema;

#[test]
fn string_email() {
    let value = r#"
        {
            "type": "string",
            "format": "email"
        }
    "#;

    let schema = serde_json::from_str::<Schema>(&value).unwrap();
    assert_ron_snapshot!(schema);
}
