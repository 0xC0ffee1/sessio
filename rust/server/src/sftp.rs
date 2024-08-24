use anyhow::Context;
use async_trait::async_trait;
use homedir::home;
use log::{debug, error, info};
use russh_sftp::protocol::{
    Data, File, FileAttributes, Handle, Name, OpenFlags, Status, StatusCode, Version,
};
use russh_sftp::server::Handler;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::metadata;
use tokio::fs::{self, File as TokioFile, OpenOptions, ReadDir};
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;

pub struct SftpSession {
    version: Option<u32>,
    root_dir_read_done: bool,
    open_directories: HashMap<String, OpenDir>,
    open_files: HashMap<String, TokioFile>,
    user: String,
}

struct OpenDir {
    dir: ReadDir,
    read: bool,
}

impl SftpSession {
    pub fn new(user: String) -> Self {
        SftpSession {
            version: None,
            root_dir_read_done: false,
            open_directories: HashMap::new(),
            open_files: HashMap::new(),
            user,
        }
    }
    fn success(id: u32) -> Status {
        Status {
            id,
            status_code: StatusCode::Ok,
            error_message: "Ok".to_string(),
            language_tag: "en-US".to_string(),
        }
    }

    fn get_user_relative_path(&mut self, filename: &String) -> Result<PathBuf, StatusCode> {
        let path = home(&self.user)
            .map_err(|e| StatusCode::Failure)?
            .ok_or(StatusCode::NoSuchFile)?
            .join(filename);
        Ok(path)
    }
}

#[async_trait]
impl Handler for SftpSession {
    type Error = StatusCode;

    fn unimplemented(&self) -> Self::Error {
        StatusCode::OpUnsupported
    }

    async fn init(
        &mut self,
        version: u32,
        extensions: HashMap<String, String>,
    ) -> Result<Version, Self::Error> {
        if self.version.is_some() {
            error!("duplicate SSH_FXP_VERSION packet");
            return Err(StatusCode::ConnectionLost);
        }

        self.version = Some(version);
        info!("version: {:?}, extensions: {:?}", self.version, extensions);
        Ok(Version::new())
    }

    async fn open(
        &mut self,
        id: u32,
        filename: String,
        _pflags: OpenFlags,
        _attrs: FileAttributes,
    ) -> Result<Handle, Self::Error> {
        info!("OPENING FILE {}", filename);

        let path = self.get_user_relative_path(&filename)?;

        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(path)
            .await
            .map_err(|_e| StatusCode::NoSuchFile)?;

        self.open_files.insert(filename.clone(), file);

        Ok(Handle {
            id,
            handle: filename,
        })
    }

    async fn read(
        &mut self,
        id: u32,
        handle: String,
        offset: u64,
        len: u32,
    ) -> Result<Data, Self::Error> {
        if let Some(file) = self.open_files.get_mut(&handle) {
            file.seek(std::io::SeekFrom::Start(offset))
                .await
                .map_err(|_| StatusCode::Failure)?;
            let mut buffer = vec![0; len as usize];
            let n = file
                .read(&mut buffer)
                .await
                .map_err(|_| StatusCode::Failure)?;
            buffer.truncate(n);

            return Ok(Data { id, data: buffer });
        }
        Err(StatusCode::Failure)
    }

    async fn close(&mut self, id: u32, handle: String) -> Result<Status, Self::Error> {
        self.open_directories.remove(&handle);

        if let Some(file) = self.open_files.remove(&handle) {
            drop(file);
        }

        Ok(Status {
            id,
            status_code: StatusCode::Ok,
            error_message: "Ok".to_string(),
            language_tag: "en-US".to_string(),
        })
    }

    async fn remove(&mut self, id: u32, path: String) -> Result<Status, Self::Error> {
        debug!("remove: {id} {path}");

        let path = self.get_user_relative_path(&path)?;

        // Try to remove as a file
        if let Err(e) = fs::remove_file(&path).await {
            if e.kind() == io::ErrorKind::NotFound {
                error!("File not found: {}", path.display());
                return Err(StatusCode::NoSuchFile);
            }
        }

        Ok(SftpSession::success(id))
    }

    async fn rmdir(&mut self, id: u32, path: String) -> Result<Status, Self::Error> {
        debug!("rmdir: {id} {path}");

        let path = self.get_user_relative_path(&path)?;

        match fs::remove_dir(&path).await {
            Ok(_) => Ok(SftpSession::success(id)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                error!("Directory not found: {}", path.display());
                Err(StatusCode::NoSuchFile)
            }
            Err(e) => {
                error!("Failed to remove directory {}: {:?}", path.display(), e);
                Err(StatusCode::Failure)
            }
        }
    }

    async fn rename(
        &mut self,
        id: u32,
        oldpath: String,
        newpath: String,
    ) -> Result<Status, Self::Error> {
        debug!("rename: {id} from {oldpath} to {newpath}");

        let oldpath = self.get_user_relative_path(&oldpath)?;
        let newpath = self.get_user_relative_path(&newpath)?;

        match fs::rename(&oldpath, &newpath).await {
            Ok(_) => Ok(SftpSession::success(id)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                error!("File or directory not found: {}", oldpath.display());
                Err(StatusCode::NoSuchFile)
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                error!("Destination already exists: {}", newpath.display());
                Err(StatusCode::Failure)
            }
            Err(e) => {
                error!(
                    "Failed to rename from {} to {}: {:?}",
                    oldpath.display(),
                    newpath.display(),
                    e
                );
                Err(StatusCode::Failure)
            }
        }
    }

    async fn write(
        &mut self,
        id: u32,
        handle: String,
        offset: u64,
        data: Vec<u8>,
    ) -> Result<Status, Self::Error> {
        if let Some(file) = self.open_files.get_mut(&handle) {
            file.seek(std::io::SeekFrom::Start(offset))
                .await
                .map_err(|_| StatusCode::Failure)?;
            file.write_all(&data)
                .await
                .map_err(|_| StatusCode::Failure)?;

            return Ok(Status {
                id,
                status_code: StatusCode::Ok,
                error_message: "Ok".to_string(),
                language_tag: "en-US".to_string(),
            });
        }
        Err(StatusCode::Failure)
    }

    async fn opendir(&mut self, id: u32, path: String) -> Result<Handle, Self::Error> {
        info!("opendir: {}", path);

        let cleaned_path = if path.starts_with('/') {
            &path[1..]
        } else {
            &path
        };

        let path_full = home(&self.user)
            .map_err(|e| StatusCode::Failure)?
            .ok_or(StatusCode::NoSuchFile)?
            .join(cleaned_path);

        info!("Opening dir {}", path_full.display());

        let read_dir = fs::read_dir(&path_full)
            .await
            .map_err(|_| StatusCode::Failure)?;

        self.open_directories.insert(
            path.clone(),
            OpenDir {
                dir: read_dir,
                read: false,
            },
        );

        Ok(Handle { id, handle: path })
    }

    async fn readdir(&mut self, id: u32, handle: String) -> Result<Name, Self::Error> {
        info!("readdir handle: {}", handle);

        if let Some(mut read_dir) = self.open_directories.get_mut(&handle) {
            let mut files = Vec::new();
            if !read_dir.read {
                while let Some(entry) = read_dir.dir.next_entry().await.map_err(|e| {
                    error!("{}", e.to_string());
                    StatusCode::Failure
                })? {
                    let file_name = entry.file_name().into_string().unwrap();
                    let file_path = entry.path();
                    info!("path {}", file_path.display());

                    // Get file metadata
                    let Ok(metadata) = metadata(&file_path).await else {
                        continue;
                    };

                    let mut file_attrs = FileAttributes {
                        size: Some(metadata.len()),
                        uid: None, // Setting these to None as placeholders
                        user: None,
                        gid: None,
                        group: None,
                        mtime: None,
                        atime: None,
                        permissions: None,
                    };

                    file_attrs.set_dir(metadata.is_dir());
                    file_attrs.set_regular(metadata.is_file());
                    file_attrs.set_symlink(metadata.is_symlink());

                    files.push(File {
                        filename: file_name.clone(),
                        longname: file_name.clone(),
                        attrs: file_attrs,
                    });
                }
                read_dir.read = true;
                info!("Returned files: {}", files.len());
                return Ok(Name { id, files });
            } else {
                return Err(StatusCode::Eof);
            }
        }
        Err(StatusCode::Eof)
    }

    async fn realpath(&mut self, id: u32, path: String) -> Result<Name, Self::Error> {
        info!("realpath: {}", path);

        let path_full = self.get_user_relative_path(&path)?;

        let canonical_path = fs::canonicalize(&path_full)
            .await
            .map_err(|_| StatusCode::Failure)?;
        let canonical_path_str = canonical_path
            .to_str()
            .ok_or(StatusCode::Failure)?
            .to_string();

        Ok(Name {
            id,
            files: vec![File {
                filename: canonical_path_str.clone(),
                longname: canonical_path_str.clone(),
                attrs: FileAttributes::default(),
            }],
        })
    }
}
