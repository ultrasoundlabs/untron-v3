use std::ffi::OsStr;
use std::path::{Path, PathBuf};

fn collect_proto_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_proto_files(&path, out)?;
            continue;
        }

        if path.extension() == Some(OsStr::new("proto")) {
            out.push(path);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=protos");
    println!("cargo:rerun-if-changed=../../crates/indexer-client/openapi.json");

    let proto_root = PathBuf::from("protos");
    let mut protos = Vec::new();
    collect_proto_files(&proto_root, &mut protos)?;
    protos.sort();

    tonic_prost_build::configure()
        .build_server(false)
        .boxed("BlockExtention")
        .compile_protos(&["protos/api/api.proto"], &["protos"])?;

    Ok(())
}
