use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use tokio::io;
use log::info;
use rand::rngs::OsRng;


pub fn generate_keypair<P: AsRef<Path>>(path: P, algorithm: Algorithm, file_name: &str) -> io::Result<()> {
    let path = path.as_ref();

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut csprng = OsRng;
    let private_key = PrivateKey::random(&mut csprng, algorithm)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to generate private key: {}", e)))?;
    
    // Save the private key in OpenSSH format
    let private_key_ssh = private_key.to_openssh(LineEnding::LF)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to encode private key to OpenSSH format: {}", e)))?;
    let private_key_path = path.join(file_name);
    if let Some(parent) = private_key_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut private_key_file = File::create(&private_key_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to create private key file: {}", e)))?;
    private_key_file.write_all(private_key_ssh.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to write private key to file: {}", e)))?;

    // Save the public key in OpenSSH format
    let public_key = private_key.public_key();
    let public_key_ssh = public_key.to_openssh()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to encode public key to OpenSSH format: {}", e)))?;
    let public_key_path = path.join(format!("{}.pub", file_name));
    if let Some(parent) = public_key_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut public_key_file = File::create(&public_key_path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to create public key file: {}", e)))?;
    public_key_file.write_all(public_key_ssh.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to write public key to file: {}", e)))?;

    info!("Generated public key: {}", public_key_ssh);

    Ok(())
}