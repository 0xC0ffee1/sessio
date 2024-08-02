fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["../../proto/client_ipc.proto"],
            &["../../proto"]
        )?;
    Ok(())

}