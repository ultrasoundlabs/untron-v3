fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos");
    println!("cargo:rerun-if-env-changed=PROTOC_INCLUDE");

    let mut includes: Vec<String> = vec!["protos".to_string()];

    if let Ok(path) = std::env::var("PROTOC_INCLUDE") {
        if !path.trim().is_empty() {
            includes.push(path);
        }
    } else {
        for candidate in [
            "/usr/local/include",
            "/usr/include",
            "/opt/homebrew/include",
            "/usr/local/opt/protobuf/include",
            "/opt/homebrew/opt/protobuf/include",
        ] {
            let any_proto = std::path::Path::new(candidate).join("google/protobuf/any.proto");
            if any_proto.exists() {
                includes.push(candidate.to_string());
            }
        }
    }

    let include_refs: Vec<&str> = includes.iter().map(|s| s.as_str()).collect();

    tonic_prost_build::configure()
        .build_server(false)
        .boxed("BlockExtention")
        .compile_protos(&["protos/api/api.proto"], &include_refs)?;

    Ok(())
}
