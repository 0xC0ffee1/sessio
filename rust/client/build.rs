fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try multiple possible locations for proto files
    let proto_paths = vec![
        ("../../proto/client_ipc.proto", "../../proto"),  // Local development
        ("../proto/client_ipc.proto", "../proto"),        // Alternative structure
        ("/proto/client_ipc.proto", "/proto"),            // Docker/Cross build
    ];
    
    for (proto_file, proto_dir) in proto_paths {
        if std::path::Path::new(proto_file).exists() {
            tonic_build::configure()
                .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
                .compile(
                    &[proto_file],
                    &[proto_dir]
                )?;
            return Ok(());
        }
    }
    
    // If we get here, proto files weren't found in any expected location
    panic!("Proto files not found. Tried: ../../proto, ../proto, and /proto");
}