use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use anyhow::Context;
use russh_keys::key::PublicKey;
use ssh_key::{Algorithm, LineEnding, PrivateKey};
use tokio::io::{self, AsyncReadExt};
use log::info;
use rand::rngs::OsRng;

pub async fn read_authorized_keys(user: Option<&str>) -> anyhow::Result<Vec<PublicKey>> {

    let path = if let Some(user) = user {
        homedir::home(user)?
    } else {
        homedir::my_home()?
    }.with_context(|| format!("Home directory not found for user"))?
    .join(".sessio/authorized_keys");

    if !path.exists() {
        // Create the file and its parent directories if they don't exist
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::File::create(&path).await?;
    }

    let mut file = tokio::fs::File::open(&path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let mut keys = Vec::new();

    for line in contents.lines() {
        let mut split = line.split_whitespace();
        
        split.next();

        if let Ok(public_key) = russh_keys::parse_public_key_base64(split.next().unwrap()) {
            keys.push(public_key);
        }
        else {
            anyhow::bail!("Failed to read authorized public key {}", line)
        }
    }

    Ok(keys)
}

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