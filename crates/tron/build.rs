fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos");

    tonic_prost_build::configure()
        .build_server(false)
        .boxed("BlockExtention")
        .compile_protos(&["protos/api/api.proto"], &["protos"])?;

    Ok(())
}
