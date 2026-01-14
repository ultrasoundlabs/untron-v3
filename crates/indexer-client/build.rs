use progenitor::{GenerationSettings, Generator, InterfaceStyle, TagStyle};
use std::fs::File;
use typify::UnknownPolicy;

fn sanitize_openapi_json(doc: &mut serde_json::Value) {
    fn walk(v: &mut serde_json::Value) {
        match v {
            // Treat completely-untyped object schemas as arbitrary JSON.
            //
            // PostgREST often emits json/jsonb columns as `type: object` in the schema even when
            // runtime values can be arrays/scalars. For client correctness, force these to our
            // `JsonValue` schema so they deserialize as `serde_json::Value`.
            serde_json::Value::Object(map) => {
                let is_object = map.get("type").and_then(|t| t.as_str()) == Some("object");
                let has_structural = map.contains_key("properties")
                    || map.contains_key("required")
                    || map.contains_key("items")
                    || map.contains_key("oneOf")
                    || map.contains_key("anyOf")
                    || map.contains_key("allOf")
                    || map.contains_key("$ref");
                if is_object && !has_structural {
                    let additional_props = map.get("additionalProperties");
                    let is_open_object = additional_props.is_none()
                        || additional_props.and_then(|v| v.as_bool()) == Some(true);
                    if is_open_object {
                        *v = serde_json::json!({ "$ref": "#/components/schemas/JsonValue" });
                        return;
                    }
                }

                // OpenAPI 3.0 does not support `type: ["T","null"]` (3.1 JSON Schema style).
                // Convert to `type: "T", nullable: true` when possible.
                if let Some(serde_json::Value::Array(types)) = map.get("type") {
                    let has_null = types.iter().any(|t| t.as_str() == Some("null"));
                    if has_null {
                        let non_null = types
                            .iter()
                            .filter_map(|t| t.as_str())
                            .find(|t| *t != "null");
                        if let Some(non_null) = non_null {
                            map.insert(
                                "type".to_string(),
                                serde_json::Value::String(non_null.to_string()),
                            );
                            map.insert("nullable".to_string(), serde_json::Value::Bool(true));
                        }
                    }
                }

                // Progenitor currently panics on schemas with explicit `type: "null"`.
                // Drop those from oneOf/anyOf/allOf unions.
                for key in ["oneOf", "anyOf", "allOf"] {
                    if let Some(serde_json::Value::Array(items)) = map.get_mut(key) {
                        let mut had_null = false;
                        items.retain(|item| {
                            let is_null_schema = item
                                .as_object()
                                .and_then(|o| o.get("type"))
                                .and_then(|t| t.as_str())
                                == Some("null");
                            if is_null_schema {
                                had_null = true;
                            }
                            !is_null_schema
                        });
                        if had_null {
                            map.insert("nullable".to_string(), serde_json::Value::Bool(true));
                        }
                    }
                }

                for v in map.values_mut() {
                    walk(v);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr {
                    walk(v);
                }
            }
            _ => {}
        }
    }

    // This crate is specifically for the PostgREST indexer API. If someone accidentally
    // drops the *merged* Untron API spec in here, strip Realtor-only endpoints/schemas so
    // codegen remains stable.
    if let Some(paths) = doc
        .get_mut("paths")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in ["/realtor", "/payout_config"] {
            paths.remove(key);
        }
        let lease_paths: Vec<String> = paths
            .keys()
            .filter(|k| k.starts_with("/leases"))
            .cloned()
            .collect();
        for k in lease_paths {
            paths.remove(&k);
        }
    }

    if let Some(schemas) = doc
        .get_mut("components")
        .and_then(serde_json::Value::as_object_mut)
        .and_then(|c| c.get_mut("schemas"))
        .and_then(serde_json::Value::as_object_mut)
    {
        for name in [
            "CreateLeaseRequest",
            "CreateLeaseResponse",
            "SetPayoutConfigRequest",
            "SetPayoutConfigResponse",
            "RealtorInfoResponse",
            "RealtorTargetPairResponse",
            "LeaseViewResponse",
            "LeasePayoutConfigView",
            "LeasePayoutConfigVersionView",
            "LeaseClaimView",
            "ErrorResponse",
        ] {
            schemas.remove(name);
        }
    }

    walk(doc);
}

fn main() {
    println!("cargo:rerun-if-changed=openapi.json");

    let file = File::open("openapi.json").expect("open openapi.json");
    let mut raw: serde_json::Value =
        serde_json::from_reader(file).expect("parse openapi.json as JSON");
    sanitize_openapi_json(&mut raw);

    let spec: openapiv3::OpenAPI =
        serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&raw.to_string()))
            .unwrap_or_else(|e| panic!("parse openapi.json: {} at {}", e, e.path()));

    let mut settings = GenerationSettings::default();
    settings
        .with_interface(InterfaceStyle::Builder)
        .with_tag(TagStyle::Merged)
        .with_timeout(30)
        .with_unknown_crates(UnknownPolicy::Allow);

    settings.with_replacement("PgNumeric", "serde_json::Number", std::iter::empty());
    settings.with_replacement("PgJson", "serde_json::Value", std::iter::empty());
    settings.with_replacement("NumericValue", "serde_json::Number", std::iter::empty());
    settings.with_replacement("JsonValue", "serde_json::Value", std::iter::empty());

    let mut generator = Generator::new(&settings);
    let tokens = generator.generate_tokens(&spec).expect("generate tokens");
    let ast = syn::parse2(tokens).expect("parse tokens");
    let content = prettyplease::unparse(&ast);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_path = std::path::Path::new(&out_dir).join("codegen.rs");
    std::fs::write(out_path, content).expect("write codegen.rs");
}
