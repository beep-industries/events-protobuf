use std::io::Result;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let proto_root = "./src/proto";

    // Recursively collect all .proto files in the proto_root directory
    let protos: Vec<_> = WalkDir::new(proto_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "proto")
                .unwrap_or(false)
        })
        .map(|e| e.path().to_owned())
        .collect();

    prost_build::compile_protos(&protos, &[proto_root])?;

    Ok(())
}
