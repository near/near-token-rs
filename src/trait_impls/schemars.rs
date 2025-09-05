use crate::NearToken;

use schemars_v0_8;
use schemars_v1;

impl schemars_v0_8::JsonSchema for NearToken {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        "NearToken".to_string()
    }

    fn json_schema(_: &mut schemars_v0_8::gen::SchemaGenerator) -> schemars_v0_8::schema::Schema {
        use schemars_v0_8::schema::{InstanceType, Schema, SchemaObject, SingleOrVec};
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
        })
    }
}

impl schemars_v1::JsonSchema for NearToken {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NearToken".to_string().into()
    }

    fn json_schema(_: &mut schemars_v1::SchemaGenerator) -> schemars_v1::Schema {
        schemars_v1::json_schema!({
            "type": "string",
        })
    }
}

#[cfg(test)]
mod test {
    use crate::NearToken;
    use serde_json::json;

    #[test]
    fn json_schema_json_eq_v0_8() {
        let root = schemars_v0_8::schema_for!(NearToken);
        let schema_json = serde_json::to_value(&root.schema).unwrap();
        assert_eq!(
            schema_json,
            json!({ "title": "NearToken", "type": "string" })
        );
    }

    #[test]
    fn json_schema_json_eq_v1() {
        let root = schemars_v1::schema_for!(NearToken);
        let schema_json = serde_json::to_value(&root).unwrap();
        assert_eq!(
            schema_json,
            json!({ "$schema": "https://json-schema.org/draft/2020-12/schema", "title": "NearToken", "type": "string" })
        );
    }
}
