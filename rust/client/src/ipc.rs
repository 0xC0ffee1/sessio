pub mod clientipc {
    tonic::include_proto!("clientipc");
}

use futures::{stream, Stream, StreamExt};
use russh_sftp::{client::SftpSession, protocol::Stat};
use url::Url;
use uuid::Uuid;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc, time::Instant};
use std::{pin::Pin, sync::Arc};
use clientipc::{
    client_ipc_server::{ClientIpc, ClientIpcServer}, msg::Type, FileCloseResponse, FileData, FileList, FileMetadataResponse, FileReadRequest, FileReadResponse, file_transfer_status::Progress, FileTransferRequest, FileTransferStatus, FileWriteRequest, FileWriteResponse, GenKeysRequest, GenKeysResponse, Msg, NewConnectionRequest, NewConnectionResponse, NewSessionRequest, NewSessionResponse, SftpRequest, SftpRequestResponse, StreamResponse
};
use clientipc::file_transfer_status::Typ;


use log::info;
use tokio::sync::Mutex;
use crate::client::{Client, Session};

#[cfg(windows)]
use tokio_stream::wrappers::TcpListenerStream;
#[cfg(windows)]
use tokio::net::TcpListener;

#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
#[cfg(unix)]
use tokio::net::UnixListener;

use tonic::{Request, Status, Response, transport::Server};

use std::path::Path;

use russh_sftp::protocol::OpenFlags;
use common::utils::keygen::generate_keypair;

struct ClientIpcHandler {
    client: Arc<Mutex<Client>>
}


#[tonic::async_trait]
impl ClientIpc for ClientIpcHandler {
    type OpenChannelStream =
        Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send  + 'static>>;
    type FileDownloadStream =
        Pin<Box<dyn Stream<Item = Result<FileTransferStatus, Status>> + Send  + 'static>>;

    type FileUploadStream = Self::FileDownloadStream;

    async fn open_sftp_channel(&self, request: Request<SftpRequest>) 
    -> Result<Response<SftpRequestResponse>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        
        let res = session_guard.lock().await.request_sftp().await;
        match res {
            Ok(id) => {
                Ok(Response::new(SftpRequestResponse{
                    channel_id: id.to_string()
                }))
            }
            Err(e) => {
                log::error!("Failed to connect to SFTP server: {}", e.to_string());
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
   }

    async fn list_directory(&self, request: Request<clientipc::Path>) 
    -> Result<Response<FileList>, Status> {
        log::info!("Got list dir request!");
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else{
            return Err(Status::new(tonic::Code::NotFound, "SFTP Session not found"));
        };
        //info!("current path: {:?}", sftp.canonicalize(&request.path).await.unwrap());

        let Ok(dir) = sftp.read_dir(&request.path).await else {
            return Err(Status::new(tonic::Code::NotFound, "Directory not found"));
        };
        
        let mut list = Vec::<FileData>::new();
        for entry in dir {
            info!("DIR FILE {}", entry.file_name());
            list.push(FileData {
                file_name: entry.file_name(),
                file_path: format!("{0}/{1}", &request.path, entry.file_name()),
                file_size: entry.metadata().size.unwrap_or(0),
                is_dir: entry.metadata().is_dir()
            });
        }
        Ok(Response::new(FileList {
            files: list 
        }))
    }


    async fn file_download(&self, request: Request<FileTransferRequest>) 
    -> Result<Response<Self::FileDownloadStream>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else{
            return Err(Status::new(tonic::Code::NotFound, "SFTP Session not found"));
        };
        let mut remote_file = sftp
            .open_with_flags(
                request.remote_path,
                OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE | OpenFlags::READ,
            )
            .await
            .unwrap();

        let mut local_file = match File::create(&request.local_path).await {
            Ok(file) => file,
            Err(e) => return Err(Status::new(tonic::Code::Internal, format!("Failed to create local file: {}", e))),
        };
        
        let res = async_stream::try_stream! {
            let mut buf = vec![0u8; 1024*512];
            let mut bytes_read: i32 = 0;
            loop {
                let n: usize = match remote_file.read(&mut buf).await {
                    Ok(n) if n == 0 => break, // EOF
                    Ok(n) => n,
                    Err(e) => {
                        //yield Err(Status::new(tonic::Code::Internal, format!("Failed to read from remote file: {}", e)));
                        log::error!("Failed to read from remote file: {}", e);
                        break;
                    },
                };
                bytes_read += n as i32;
                if let Err(e) = local_file.write_all(&buf[..n]).await {
                    //yield Err(Status::new(tonic::Code::Internal, format!("Failed to write to local file: {}", e)));
                    log::error!("Failed to write to local file: {}", e);
                    break;
                }

                let progress = Progress {
                    bytes_read: bytes_read
                };

                let file_transfer_status = FileTransferStatus {
                    typ: Some(Typ::Progress(progress)),
                };

                yield file_transfer_status;
            }
            yield FileTransferStatus {
                typ: Some(Typ::Completed(Default::default())),
            };
        };
        
        Ok(Response::new(Box::pin(res) as Self::FileDownloadStream))
    }


    async fn file_upload(&self, request: Request<FileTransferRequest>) 
    -> Result<Response<Self::FileUploadStream>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else{
            return Err(Status::new(tonic::Code::NotFound, "SFTP Session not found"));
        };
        let mut remote_file = sftp
            .open_with_flags(
                request.remote_path,
                OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE | OpenFlags::READ,
            )
            .await
            .unwrap();
        
        info!("Opening {}", request.local_path);
        let mut local_file = match File::open(&request.local_path).await {
            Ok(file) => file,
            Err(e) => return Err(Status::new(tonic::Code::Internal, format!("Failed to create local file: {}", e))),
        };

        let res = async_stream::try_stream! {
        let mut buf = vec![0u8; 1024*512];
        let mut bytes_written: i32 = 0;
        loop {
            let n = match local_file.read(&mut buf).await {
                Ok(n) if n == 0 => break, // EOF
                Ok(n) => n,
                Err(e) => {
                    log::error!("Failed to read from local file: {}", e);
                    break;
                }
            };

            let start_time = Instant::now();
            if let Err(e) = remote_file.write_all(&buf[..n]).await {
                log::error!("Failed to write to remote file: {}", e);
                break;
                //return Err(Status::new(tonic::Code::Internal, format!("Failed to write to remote file: {}", e)));
            }
            let elapsed_time = start_time.elapsed();

            log::info!("Buffer written in {:?}", elapsed_time);

            bytes_written += n as i32;
            let progress = Progress {
                bytes_read: bytes_written
            };

            let file_transfer_status = FileTransferStatus {
                typ: Some(Typ::Progress(progress)),
            };

            yield file_transfer_status;

        }
        };
        
        Ok(Response::new(Box::pin(res) as Self::FileUploadStream))
    }


    async fn new_connection(&self, request: Request<NewConnectionRequest>) 
    -> Result<Response<NewConnectionResponse>, Status> {
        let request = request.into_inner();

        let Ok(url) = request.coordinator_url.parse() else {
            return Err(Status::new(tonic::Code::InvalidArgument, "Invalid coordinator URL!"));
        };

        let mut client = self.client.lock().await;
        let res = client.new_connection(request.target_id.clone(), url).await;
        match res {
            Ok(id) => {
                log::info!("CONN OK");
                Ok(Response::new(NewConnectionResponse{
                    connection_id: request.target_id
                }))
            }
            Err(e) => {
                log::error!("Failed to connect {}", e);
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
    }

    async fn gen_keys(&self, request: Request<GenKeysRequest>) -> Result<Response<GenKeysResponse>, Status> {
        let request = request.into_inner();
        let res = generate_keypair(&request.key_path, ssh_key::Algorithm::Ed25519, "id_ed25519");
        match res {
            Ok(_) => {
                Ok(Response::new(GenKeysResponse{
                    key_path: request.key_path
                }))
            }
            Err(e) => {
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
    }
    
    async fn new_session(&self, request: Request<NewSessionRequest>) 
    -> Result<Response<NewSessionResponse>, Status> {
        let request = request.into_inner();
        info!("IPC: Requesting new session!");
        let mut client = self.client.lock().await;

        let username = request.username.clone();

        let res = client.new_session(request.connection_id.clone(), 
        request.username, 
        request.private_key, 
        request.known_hosts_path).await;
        
        info!("IPC: Session requested!");
        match res {
            Ok(id) => {
                Ok(Response::new(NewSessionResponse{
                    session_id: id
                }))
            }
            Err(e) => {
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
    }

    async fn open_channel(&self, request: Request<tonic::Streaming<Msg>>)
    -> Result<Response<Self::OpenChannelStream>, Status> {
        let mut stream = request.into_inner();

        let Some(msg_res) = stream.next().await else {
            return Err(Status::new(tonic::Code::InvalidArgument, "Expected session id as the first message!"));
        };
        let Ok(msg) = msg_res else {
            return Err(Status::new(tonic::Code::InvalidArgument, "Error while reading initial message"));  
        };
        let Some(Type::ChannelInit(channel_init)) = msg.r#type else {
            return Err(Status::new(tonic::Code::InvalidArgument, "Initial message must be of type ChannelInit"));  
        };

        info!("IPC: Opening a channel!");

        let client_clone = self.client.clone();

        let mut client = client_clone.lock().await;
        
        let session_guard = match client.sessions.get_mut(&channel_init.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };

        info!("IPC: Opening a channel! #1");
        
        info!("IPC: Opening a channel! #2");

        let channel_id = {
            let mut session = session_guard.lock().await;
            info!("IPC: Opening a channel! #3");
            session.new_channel().await.unwrap()
        };

        info!("IPC: Opening a channel! #4");

        let session_clone = session_guard.clone();

        let res = async_stream::try_stream! {

            let (input_tx, input_rx) = mpsc::channel(32);
            let (output_tx, mut output_rx) = mpsc::channel(32);

            let input_guard = Arc::new(Mutex::new(input_rx));
            let output_guard = Arc::new(Mutex::new(output_tx));

            let o_clone = output_guard.clone();

            tokio::spawn(async move {
                while let Some(msg) = stream.next().await {
                    match msg {
                        Ok(msg) => {
                            match msg.r#type {
                                Some(Type::Data(data)) => {
                                    //Sends the data to the shell stream
                                    info!("IPC: Forwarding data!");
                                    input_tx.send(data.payload).await.unwrap();
                                }
                                Some(Type::PtyRequest(req)) => {
                                    info!("IPC: Opening a pty!");
                                    let mut session = session_clone.lock().await;
                                    let _ = session.request_pty(&channel_id, req.col_width, req.row_height).await;
                                }
                                Some(Type::PtyResize(req)) => {
                                    let mut session = session_clone.lock().await;
                                    let _ = session.resize_pty(&channel_id, req.col_width, req.row_height).await;
                                }
                                Some(Type::ShellRequest(req)) => {
                                    info!("IPC: Opening a shell!");
                                    
                                    let session = session_clone.clone();
                                    //ignoring until tests complete
                                    let input = input_guard.clone();
                                    let output = o_clone.clone();
                                    tokio::spawn(async move {
                                        let task = {
                                            let session = session.lock().await;
                                            let channel_guard = session.channels.get(&channel_id);
                                            Session::request_shell(channel_guard.unwrap().clone(), input, output)
                                        };

                                        if let Err(e) = task.await {
                                            eprintln!("Failed to request shell: {:?}", e);
                                        }
                                    });
                                    
                                }
                                _ => {
                                    // Handle other cases
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to receive message: {:?}", e);
                            break;
                        }
                    }
                }
                info!("Message listening loop broken");
            });
            

            // Process output from the shell and send back to the ipc listener
            while let Some(data) = output_rx.recv().await {
                let msg = Msg {
                    r#type: Some(Type::Data(crate::ipc::clientipc::msg::Data { payload: data })),
                };
                yield msg;
            }
            info!("Message sending loop broken");
        };

        Ok(Response::new(Box::pin(res) as Self::OpenChannelStream))        
    }
}


#[cfg(unix)]
pub async fn start_grpc_server(path_str: &str) {
    let path = path_str.to_string();
    let _ = std::fs::remove_file(path_str);

    let uds_res = UnixListener::bind(&path);
    let Ok(uds) = uds_res else {
            info!("Is err {}", uds_res.err().unwrap());
            return;
    };
    
    let uds_stream = UnixListenerStream::new(uds);

    let greeter = ClientIpcHandler {
        client: Arc::new(Mutex::new(Client::default()))
    };

    info!("Starting grpc server!");
    let res = Server::builder()
        .add_service(ClientIpcServer::new(greeter))
        .serve_with_incoming(uds_stream)
        .await;

    info!("exited grpc server, is err {}", res.is_err());
}

#[cfg(windows)]
pub async fn start_grpc_server(path_str: &str) {
    let _ = std::fs::remove_file(path_str);

    let greeter = ClientIpcHandler {
        client: Arc::new(Mutex::new(Client::default()))
    };

    info!("Starting grpc server!");
    let res = Server::builder()
        .add_service(ClientIpcServer::new(greeter))
        .serve(path_str.parse().unwrap())
        .await;


    info!("exited grpc server, is err {}", res.is_err());
}



pub fn start_server_new_runtime(path_str: &str) {
    let path = path_str.to_string();
    let _ = std::fs::remove_file(path_str);
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            start_grpc_server(&path).await;
        });
    });
}