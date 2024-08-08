pub mod clientipc {
    tonic::include_proto!("clientipc");
}

use futures::{stream, Stream, StreamExt};
use russh_sftp::{client::SftpSession, protocol::Stat};
use url::Url;
use uuid::Uuid;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc, time::Instant};
use std::{any::Any, collections::HashMap, net::Ipv6Addr, path::PathBuf, pin::Pin, sync::Arc};
use clientipc::{
    client_ipc_server::{ClientIpc, ClientIpcServer}, file_transfer_status::Progress, msg::Type, session_data, DeviceStatus, FileCloseResponse, FileData, FileList, FileMetadataResponse, FileReadRequest, FileReadResponse, FileTransferRequest, FileTransferStatus, FileWriteRequest, FileWriteResponse, GenKeysRequest, GenKeysResponse, GetKeyRequest, GetSaveDataRequest, InitData, InitResponse, LocalPortForwardRequest, LocalPortForwardResponse, Msg, NewConnectionRequest, NewConnectionResponse, NewSessionRequest, NewSessionResponse, PublicKey, SessionData, SessionMap, SessionRequest, Settings, SettingsRequest, SftpRequest, SftpRequestResponse, StreamResponse, UserData, Value
};
use clientipc::file_transfer_status::Typ;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeStruct, Serializer};
use clientipc::value::Kind;
use clientipc::session_data::Kind as SessionKind;

use tokio::io::BufReader;
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

    async fn init_client(&self, request: Request<InitData>)
    -> Result<Response<InitResponse>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        client.set_data_folder(PathBuf::from(request.data_folder_path));

        Ok(Response::new(InitResponse {}))
    }

    async fn get_active_sessions(&self, request: Request<SessionRequest>)
    -> Result<Response<SessionMap>, Status> {
        let mut client = self.client.lock().await;
        let mut new_map = HashMap::new();
        let mut parent_map = HashMap::new();
        
        for (k, v) in client.sessions.iter() {
            let mut session = v.lock().await;
            session.data.session_id = Some(session.id.clone());
            new_map.insert(session.id.clone(), session.data.clone());
            parent_map.insert(session.server_id.clone(), DeviceStatus {
                //Since we're loading these from memory
                //Todo check if connection has timed out
                connected: client.connections.get(&session.server_id.clone()).unwrap().close_reason().is_none()
            });
        }

        if let Ok(user_data) = Client::get_json_as::<UserData>(client.get_save_file().await.unwrap()).await {
            for (k, mut data) in user_data.saved_sessions.iter() {
                let mut final_data = data.clone();
                final_data.session_id = Some(k.to_string());
                new_map.insert(k.to_string(), final_data.clone());
                parent_map.insert(final_data.device_id, DeviceStatus {
                    connected: client.sessions.contains_key(k) && client.connections.get(&data.device_id).unwrap().close_reason().is_none()
                });
            }   
        }

        Ok(Response::new(SessionMap {
            map: new_map,
            parents: parent_map
        }))
    }

    async fn local_port_forward(&self, request: Request<SessionData>) 
    -> Result<Response<LocalPortForwardResponse>, Status> {
        let request = request.into_inner();
        let Some(crate::ipc::clientipc::session_data::Kind::Lpf(ref lpf_data)) = request.kind else {
            return Err(Status::new(tonic::Code::InvalidArgument, "Session kind must be LPF"));
        };

        let session = {
            let mut client = self.client.lock().await;
    
            let session_guard = match client.sessions.get_mut(&request.session_id.unwrap()) {
                Some(session) => session,
                None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
            };
            session_guard.clone()
        };

        session.lock().await.direct_tcpip_forward(&lpf_data.local_host, 
            lpf_data.local_port, &lpf_data.remote_host, lpf_data.remote_port).await.map_err(|e| 
                Status::new(tonic::Code::Internal, e.to_string()))?;
        
        Ok(Response::new(LocalPortForwardResponse {}))
    }

    async fn get_settings(&self, request: Request<SettingsRequest>) 
    -> Result<Response<Settings>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = client.get_settings_file().await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;
    
        let settings = Client::get_json_as::<Settings>(file).await.unwrap_or(Settings {
            coordinator_url: "quic://example.com:2223".into(),
            device_id: "Your-Device-ID".into()
        });

        Ok(Response::new(settings))
    }

    async fn get_save_data(&self, request: Request<GetSaveDataRequest>) 
    -> Result<Response<UserData>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = client.get_save_file().await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;
    
        let data = Client::get_json_as::<UserData>(file).await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;

        Ok(Response::new(data))
    }

    async fn save_settings(&self, request: Request<Settings>) 
    -> Result<Response<Settings>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = client.get_settings_file().await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;
    
        let data = Client::save_json_as::<Settings>(file, request).await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;

        Ok(Response::new(data))
    }

    async fn save_user_data(&self, request: Request<UserData>) 
    -> Result<Response<UserData>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = client.get_save_file().await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;
    
        let data = Client::save_json_as::<UserData>(file, request).await.map_err(|e| {
            Status::new(tonic::Code::Internal, e.to_string())
        })?;

        Ok(Response::new(data))
    }

    async fn open_sftp_channel(&self, request: Request<SessionData>) 
    -> Result<Response<SftpRequestResponse>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id.unwrap()) {
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

        let dir = match sftp.read_dir(&request.path).await {
            Ok(dir) => dir,
            Err(e) => {
                return Err(Status::new(tonic::Code::NotFound, e.to_string()));
            }
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
        
        let ipv6_addr: Option<Ipv6Addr> = request.own_ipv6.and_then(|s| s.parse().ok());

        let res = client.new_connection(request.target_id.clone(), url, ipv6_addr).await;
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
        let client = self.client.lock().await;
        let Some(data_folder) = &client.data_folder_path else {
            return Err(Status::new(tonic::Code::FailedPrecondition, "Data folder not set! Possibly init rpc not called"));
        };

        let res = generate_keypair(data_folder.join("keys"), ssh_key::Algorithm::Ed25519, "id_ed25519");

        match res {
            Ok(_) => {
                Ok(Response::new(GenKeysResponse{}))
            }
            Err(e) => {
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
    }

    async fn get_public_key(&self, request: Request<GetKeyRequest>) -> Result<Response<PublicKey>, Status> {
        let request = request.into_inner();
        let client = self.client.lock().await;
        let Some(data_folder) = &client.data_folder_path else {
            return Err(Status::new(tonic::Code::FailedPrecondition, "Data folder not set! Possibly init rpc not called"));
        };

        let mut file = File::options().read(true).open(data_folder.join("keys/id_ed25519.pub")).await?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        let res = reader.read_to_string(&mut contents).await?;

        Ok(Response::new(PublicKey{key: contents}))
    }
    
    async fn new_session(&self, request: Request<NewSessionRequest>) 
    -> Result<Response<NewSessionResponse>, Status> {
        let request = request.into_inner();
        info!("IPC: Requesting new session!");
        let mut client = self.client.lock().await;

        let Some(session_data) = request.session_data else {
            return Err(Status::new(tonic::Code::InvalidArgument, "You must specify session data!"));
        };

        let mut session_data_cloned = session_data.clone();

        if session_data.kind.is_none() {
            return Err(Status::new(tonic::Code::InvalidArgument, "You must specify the session kind!"));
        };

        let res = client.new_session(session_data.device_id.clone(), 
        session_data_cloned.clone(),
        session_data.username,
        session_data.session_id,
        request.private_key,
        request.known_hosts_path
        ).await;


        
        info!("IPC: Session requested!");
        match res {
            Ok(id) => {
                if session_data_cloned.session_id.is_none() {
                    //This is kinda messy
                    let mut save_file = client.get_save_file().await.unwrap();
                    let mut user_data = Client::get_json_as::<UserData>(save_file).await.unwrap();
                    session_data_cloned.session_id = Some(id.clone());
                    user_data.saved_sessions.insert(id.clone(), session_data_cloned);
                    save_file = client.get_save_file().await.unwrap();
                    let _ = Client::save_json_as(save_file, user_data).await;
                }

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

        let channel_id = {
            let mut session = session_guard.lock().await;
            session.new_channel().await.unwrap()
        };

        let session_clone = session_guard.clone();

        let res = async_stream::try_stream! {

            let (input_tx, input_rx) = mpsc::channel(32);
            let (output_tx, mut output_rx) = mpsc::channel(32);

            let input_guard = Arc::new(Mutex::new(input_rx));
            let output_guard = Arc::new(Mutex::new(output_tx));

            let o_clone = output_guard.clone();

            tokio::spawn(async move {
                while let Some(msg) = stream.next().await {
                    let msg_clone = msg.clone().unwrap();
                    match msg {
                        Ok(msg) => {
                            match msg.r#type {
                                Some(Type::Data(data)) => {
                                    //Sends the data to the shell stream
                                    input_tx.send(msg_clone).await.unwrap();
                                }
                                Some(Type::PtyRequest(req)) => {
                                    info!("IPC: Opening a pty!");
                                    let mut session = session_clone.lock().await;
                                    let _ = session.request_pty(&channel_id, req.col_width, req.row_height).await;
                                }
                                Some(Type::PtyResize(req)) => {
                                    input_tx.send(msg_clone).await.unwrap();
                                }
                                Some(Type::ShellRequest(req)) => {
                                    info!("IPC: Opening a shell!");
                                    
                                    let session = session_clone.clone();
                                    //ignoring until tests complete
                                    let input = input_guard.clone();
                                    let output = o_clone.clone();
                                    tokio::spawn(async move {
                                        let task = {
                                            let ses = session.lock().await;
                                            let channel_guard = ses.channels.get(&channel_id).unwrap().clone();

                                            Session::request_shell(channel_guard, input, output)
                                        };

                                        if let Err(e) = task.await {
                                            eprintln!("Failed to request shell: {:?}", e);
                                        }
                                    });
                                }
                                _ => {}
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