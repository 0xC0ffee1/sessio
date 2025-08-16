use anyhow::Context;
use log::info;
use rand::rngs::OsRng;
use russh::keys::PublicKey;
use russh::keys::ssh_key::{Algorithm, LineEnding, PrivateKey};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::io::{self, AsyncReadExt};

pub async fn read_authorized_keys(user: Option<&str>) -> anyhow::Result<Vec<PublicKey>> {
    read_authorized_keys_with_filter(user, None).await
}

pub async fn read_authorized_keys_with_filter(user: Option<&str>, match_user: Option<&str>) -> anyhow::Result<Vec<PublicKey>> {
    let path = match std::env::var("AUTHORIZED_KEYS_PATH").ok() {
        Some(path) => PathBuf::from(path),
        None => {
            let path = if let Some(user) = user {
                homedir::home(user)?
            } else {
                homedir::my_home()?
            };
            path.with_context(|| String::from("Home directory not found for user"))?
                .join(".sessio/authorized_keys")
        }
    };

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

        // Skip the key type (ssh-ed25519, ssh-rsa, etc.)
        split.next();

        // Get the public key data
        let key_data = match split.next() {
            Some(data) => data,
            None => continue, // Skip malformed lines
        };

        // Get the comment/device_id (third field)
        let comment = split.next().unwrap_or("");

        // If match_user is specified, only include keys for that user/device_id
        if let Some(target_user) = match_user {
            // Extract device_id from "device-id@os" format
            let device_id = comment.split('@').next().unwrap_or("");
            if device_id != target_user {
                continue; // Skip this key as it doesn't match the target user
            }
        }

        if let Ok(public_key) = russh::keys::parse_public_key_base64(key_data) {
            keys.push(public_key);
        } else {
            anyhow::bail!("Failed to read authorized public key {}", line)
        }
    }

    Ok(keys)
}

pub async fn read_known_hosts(user: Option<&str>) -> anyhow::Result<Vec<PublicKey>> {
    read_known_hosts_with_filter(user, None).await
}

pub async fn read_known_hosts_with_filter(user: Option<&str>, match_user: Option<&str>) -> anyhow::Result<Vec<PublicKey>> {
    let path = match std::env::var("KNOWN_HOSTS_PATH").ok() {
        Some(path) => PathBuf::from(path),
        None => {
            let path = if let Some(user) = user {
                homedir::home(user)?
            } else {
                homedir::my_home()?
            };
            path.with_context(|| String::from("Home directory not found for user"))?
                .join(".sessio/known_hosts")
        }
    };

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

        // Skip the hostname or key type (first field)
        split.next();

        // Get the public key data (second field)
        let key_data = match split.next() {
            Some(data) => data,
            None => continue, // Skip malformed lines
        };

        // Get the comment/device_id (third field)
        let comment = split.next().unwrap_or("");

        // If match_user is specified, only include keys for that user/device_id
        if let Some(target_user) = match_user {
            // Extract device_id from "device-id@os" format
            let device_id = comment.split('@').next().unwrap_or("");
            if device_id != target_user {
                continue; // Skip this key as it doesn't match the target user
            }
        }

        if let Ok(public_key) = russh::keys::parse_public_key_base64(key_data) {
            keys.push(public_key);
        } else {
            anyhow::bail!("Failed to read known host public key {}", line)
        }
    }

    Ok(keys)
}

pub fn generate_keypair<P: AsRef<Path>>(
    path: P,
    algorithm: Algorithm,
    file_name: &str,
) -> io::Result<()> {
    let path = path.as_ref();

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut csprng = OsRng;
    let private_key = PrivateKey::random(&mut csprng, algorithm).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to generate private key: {}", e),
        )
    })?;

    // Save the private key in OpenSSH format
    let private_key_ssh = private_key.to_openssh(LineEnding::LF).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to encode private key to OpenSSH format: {}", e),
        )
    })?;
    let private_key_path = path.join(file_name);
    if let Some(parent) = private_key_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut private_key_file = File::create(&private_key_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create private key file: {}", e),
        )
    })?;
    private_key_file
        .write_all(private_key_ssh.as_bytes())
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to write private key to file: {}", e),
            )
        })?;

    // Save the public key in OpenSSH format
    let public_key = private_key.public_key();
    let public_key_ssh = public_key.to_openssh().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to encode public key to OpenSSH format: {}", e),
        )
    })?;
    let public_key_path = path.join(format!("{}.pub", file_name));
    if let Some(parent) = public_key_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut public_key_file = File::create(&public_key_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create public key file: {}", e),
        )
    })?;
    public_key_file
        .write_all(public_key_ssh.as_bytes())
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to write public key to file: {}", e),
            )
        })?;

    info!("Generated public key: {}", public_key_ssh);

    Ok(())
}
