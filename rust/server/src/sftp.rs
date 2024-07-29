
use async_trait::async_trait;
use russh_sftp::protocol::{Data, File, FileAttributes, Handle, Name, OpenFlags, Status, StatusCode, Version};
use russh_sftp::server::Handler;
use std::collections::HashMap;
use std::hash::Hash;
use tokio::fs::{self, File as TokioFile, OpenOptions, ReadDir};
use tokio::fs::metadata;
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::path::PathBuf;
use log::{error, info};
use std::sync::Arc;

pub struct SftpSession {
    version: Option<u32>,
    root_dir_read_done: bool,
    open_directories: Arc<Mutex<HashMap<String, OpenDir>>>,
    open_files: Arc<Mutex<HashMap<String, TokioFile>>>,
}

struct OpenDir{
    dir: ReadDir,
    read: bool
}


impl SftpSession {
    pub fn new() -> Self {
        SftpSession {
            version: None,
            root_dir_read_done: false,
            open_directories: Arc::new(Mutex::new(HashMap::new())),
            open_files:  Arc::new(Mutex::new(HashMap::new())),
        }
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
        let file = OpenOptions::new().read(true).create(true).write(true).open(&filename).await.unwrap();
        let mut open_files = self.open_files.lock().await;
        open_files.insert(filename.clone(), file);

        Ok(Handle { id, handle: filename })
    }

    
    async fn read(
        &mut self,
        id: u32,
        handle: String,
        offset: u64,
        len: u32,
    ) -> Result<Data, Self::Error> {
        let mut open_files = self.open_files.lock().await;
        if let Some(file) = open_files.get_mut(&handle) {
            file.seek(std::io::SeekFrom::Start(offset)).await.map_err(|_| StatusCode::Failure)?;
            let mut buffer = vec![0; len as usize];
            let n = file.read(&mut buffer).await.map_err(|_| StatusCode::Failure)?;
            buffer.truncate(n);

            return Ok(Data { id, data: buffer });
        }
        Err(StatusCode::Failure)
    }

    async fn close(&mut self, id: u32, handle: String) -> Result<Status, Self::Error> {
        // Lock and remove directory handles if any
        let mut open_directories = self.open_directories.lock().await;
        open_directories.remove(&handle);

        // Lock and remove file handles if any
        let mut open_files = self.open_files.lock().await;

        if let Some(file) = open_files.remove(&handle) {
            //Just making sure it is closed lol
            drop(file); 
        }

        Ok(Status {
            id,
            status_code: StatusCode::Ok,
            error_message: "Ok".to_string(),
            language_tag: "en-US".to_string(),
        })
    }

    async fn write(
        &mut self,
        id: u32,
        handle: String,
        offset: u64,
        data: Vec<u8>,
    ) -> Result<Status, Self::Error> {
        let mut open_files = self.open_files.lock().await;
        if let Some(file) = open_files.get_mut(&handle) {
            file.seek(std::io::SeekFrom::Start(offset)).await.map_err(|_| StatusCode::Failure)?;
            file.write_all(&data).await.map_err(|_| StatusCode::Failure)?;

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

        let path_buf = PathBuf::from(&path);
        let read_dir = fs::read_dir(&path_buf).await.map_err(|_| StatusCode::Failure)?;

        let mut open_directories = self.open_directories.lock().await;
        open_directories.insert(path.clone(), OpenDir {
            dir: read_dir,
            read: false
        });

        Ok(Handle { id, handle: path })
    }

    async fn readdir(&mut self, id: u32, handle: String) -> Result<Name, Self::Error> {
        info!("readdir handle: {}", handle);

        let mut open_directories = self.open_directories.lock().await;
        if let Some(mut read_dir) = open_directories.get_mut(&handle) {
            let mut files = Vec::new();
            if !read_dir.read {
                while let Some(entry) = read_dir.dir.next_entry().await.map_err(|_| StatusCode::Failure)? {
                    let file_name = entry.file_name().into_string().unwrap();
                    let file_path = entry.path();

                    // Get file metadata
                    let metadata = metadata(&file_path).await.map_err(|_| StatusCode::Failure)?;
                    
                    let mut file_attrs = FileAttributes {
                        size: Some(metadata.len()),
                        uid: None, // Setting these to None as placeholders
                        user: None,
                        gid: None,
                        group: None,
                        mtime: None,
                        atime: None,
                        permissions: None
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
            }
            else {
                return Err(StatusCode::Eof);
            }
        }
        Err(StatusCode::Eof)
    }

    async fn realpath(&mut self, id: u32, path: String) -> Result<Name, Self::Error> {
        info!("realpath: {}", path);

        let canonical_path = fs::canonicalize(&path).await.map_err(|_| StatusCode::Failure)?;
        let canonical_path_str = canonical_path.to_str().ok_or(StatusCode::Failure)?.to_string();

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
