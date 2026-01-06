use progenitor::{GenerationSettings, Generator, InterfaceStyle, TagStyle};
use std::fs::File;
use typify::UnknownPolicy;

fn main() {
    println!("cargo:rerun-if-changed=openapi.json");

    let file = File::open("openapi.json").expect("open openapi.json");
    let spec: openapiv3::OpenAPI = serde_json::from_reader(file).expect("parse openapi.json");

    let mut settings = GenerationSettings::default();
    settings
        .with_interface(InterfaceStyle::Builder)
        .with_tag(TagStyle::Merged)
        .with_timeout(30)
        .with_unknown_crates(UnknownPolicy::Allow);

    settings.with_replacement("PgNumeric", "serde_json::Number", std::iter::empty());
    settings.with_replacement("PgJson", "serde_json::Value", std::iter::empty());

    let mut generator = Generator::new(&settings);
    let tokens = generator.generate_tokens(&spec).expect("generate tokens");
    let ast = syn::parse2(tokens).expect("parse tokens");
    let content = prettyplease::unparse(&ast);

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_path = std::path::Path::new(&out_dir).join("codegen.rs");
    std::fs::write(out_path, content).expect("write codegen.rs");
}
