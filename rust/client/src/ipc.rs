pub mod clientipc {
    tonic::include_proto!("clientipc");
}

use clientipc::client_event_service_server::ClientEventService;
use clientipc::file_transfer_status::Typ;
use clientipc::session_data::Kind as SessionKind;
use clientipc::value::Kind;
use clientipc::{
    client_ipc_server::{ClientIpc, ClientIpcServer},
    file_transfer_status::Progress,
    msg::Type,
    session_data, ClientEvent, CoordinatorStartRequest, CoordinatorStartResponse, DeviceStatus,
    FileCloseResponse, FileData, FileDeleteRequest, FileList, FileMetadataResponse,
    FileReadRequest, FileReadResponse, FileTransferRequest, FileTransferStatus, FileWriteRequest,
    FileWriteResponse, GenKeysRequest, GenKeysResponse, GetKeyRequest, GetSaveDataRequest,
    LocalPortForwardRequest, LocalPortForwardResponse, Msg, NatFilterRequest, NatFilterResponse,
    NewConnectionRequest, NewConnectionResponse, NewSessionRequest, NewSessionResponse, PublicKey,
    SessionData, SessionMap, SessionRequest, SettingCheckRequest, SettingCheckResponse, Settings,
    SettingsRequest, SftpRequest, SftpRequestResponse, StreamResponse, SubscribeRequest, UserData,
    Value,
};
use clientipc::{FileDeleteResponse, FileRenameRequest, FileRenameResponse};
use coordinator::coordinator_client::CoordinatorClient;
use futures::{stream, Stream, StreamExt};
use log4rs::append::file;
use russh_sftp::{client::SftpSession, protocol::Stat};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeStruct, Serializer};
use std::{any::Any, collections::HashMap, net::Ipv6Addr, path::PathBuf, pin::Pin, sync::Arc};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc,
    time::Instant,
};
use url::Url;
use uuid::Uuid;

use crate::client::{Client, Session};
use log::info;
use tokio::io::BufReader;
use tokio::sync::Mutex;

#[cfg(windows)]
use tokio::net::TcpListener;
#[cfg(windows)]
use tokio_stream::wrappers::TcpListenerStream;

#[cfg(unix)]
use tokio::net::UnixListener;
#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;

use tonic::{transport::Server, Request, Response, Status};

use std::path::Path;

use common::utils::keygen::generate_keypair;
use russh_sftp::protocol::OpenFlags;

struct ClientIpcHandler {
    client: Arc<Mutex<Client>>,
}
struct ClientEventsHandler {
    client: Arc<Mutex<Client>>,
}

#[tonic::async_trait]
impl ClientEventService for ClientEventsHandler {
    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<ClientEvent, Status>> + Send + 'static>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let mut receiver = {
            let mut client = self.client.lock().await;
            client.event_bus.subscribe().await
        };

        let res = async_stream::try_stream! {
            loop {
                let Ok(event) = receiver.recv().await else {
                   log::error!("Failed to receive event");
                   break;
                };
                yield event;
            }
        };
        Ok(Response::new(Box::pin(res) as Self::SubscribeStream))
    }
}

#[tonic::async_trait]
impl ClientIpc for ClientIpcHandler {
    type OpenChannelStream = Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send + 'static>>;
    type FileDownloadStream =
        Pin<Box<dyn Stream<Item = Result<FileTransferStatus, Status>> + Send + 'static>>;

    type FileUploadStream = Self::FileDownloadStream;

    async fn start_coordinator(
        &self,
        request: Request<CoordinatorStartRequest>,
    ) -> Result<Response<CoordinatorStartResponse>, Status> {
        let mut client = self.client.lock().await;
        if client.check_coordinator_enabled() {
            return Ok(Response::new(CoordinatorStartResponse { started: true }));
        }
        if let Err(e) = client.init_coordinator().await {
            log::error!("Failed to start coordinator! {}", e);
        };

        Ok(Response::new(CoordinatorStartResponse {
            started: client.check_coordinator_enabled(),
        }))
    }

    async fn check_settings_validity(
        &self,
        request: Request<SettingCheckRequest>,
    ) -> Result<Response<SettingCheckResponse>, Status> {
        let mut client = self.client.lock().await;

        Ok(Response::new(SettingCheckResponse {
            valid: client.check_coordinator_enabled(),
        }))
    }

    async fn get_active_sessions(
        &self,
        request: Request<SessionRequest>,
    ) -> Result<Response<SessionMap>, Status> {
        let mut client = self.client.lock().await;
        let mut new_map = HashMap::new();
        let mut parent_map = HashMap::new();

        for (k, v) in client.sessions.iter() {
            let mut session = v.lock().await;
            session.data.session_id = Some(session.id.clone());
            new_map.insert(session.id.clone(), session.data.clone());
            parent_map.insert(
                session.server_id.clone(),
                DeviceStatus {
                    //Since we're loading these from memory
                    //Todo check if connection has timed out
                    connected: client
                        .connections
                        .get(&session.server_id.clone())
                        .unwrap()
                        .close_reason()
                        .is_none(),
                },
            );
        }

        if let Ok(user_data) = Client::get_json_as::<UserData>(
            Client::get_save_file(&client.data_folder_path)
                .await
                .unwrap(),
        )
        .await
        {
            for (k, mut data) in user_data.saved_sessions.iter() {
                let mut final_data = data.clone();
                final_data.session_id = Some(k.to_string());
                new_map.insert(k.to_string(), final_data.clone());
                parent_map.insert(
                    final_data.device_id,
                    DeviceStatus {
                        connected: client.sessions.contains_key(k)
                            && client
                                .connections
                                .get(&data.device_id)
                                .unwrap()
                                .close_reason()
                                .is_none(),
                    },
                );
            }
        }

        Ok(Response::new(SessionMap {
            map: new_map,
            parents: parent_map,
        }))
    }

    async fn local_port_forward(
        &self,
        request: Request<SessionData>,
    ) -> Result<Response<LocalPortForwardResponse>, Status> {
        let request = request.into_inner();
        let Some(crate::ipc::clientipc::session_data::Kind::Lpf(ref lpf_data)) = request.kind
        else {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Session kind must be LPF",
            ));
        };

        let session = {
            let mut client = self.client.lock().await;

            let session_guard = match client.sessions.get_mut(&request.session_id.unwrap()) {
                Some(session) => session,
                None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
            };
            session_guard.clone()
        };

        session
            .lock()
            .await
            .direct_tcpip_forward(
                &lpf_data.local_host,
                lpf_data.local_port,
                &lpf_data.remote_host,
                lpf_data.remote_port,
            )
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(LocalPortForwardResponse {}))
    }

    async fn get_settings(
        &self,
        request: Request<SettingsRequest>,
    ) -> Result<Response<Settings>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = Client::get_settings_file(&client.data_folder_path)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let settings = Client::get_json_as::<Settings>(file)
            .await
            .unwrap_or(Settings {
                coordinator_url: "quic://example.com:2223".into(),
                device_id: "Your-Device-ID".into(),
            });

        Ok(Response::new(settings))
    }

    async fn get_save_data(
        &self,
        request: Request<GetSaveDataRequest>,
    ) -> Result<Response<UserData>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = Client::get_save_file(&client.data_folder_path)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let data = Client::get_json_as::<UserData>(file)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(data))
    }

    async fn save_settings(
        &self,
        request: Request<Settings>,
    ) -> Result<Response<Settings>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = Client::get_settings_file(&client.data_folder_path)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let data = Client::save_json_as::<Settings>(file, request)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(data))
    }

    async fn save_user_data(
        &self,
        request: Request<UserData>,
    ) -> Result<Response<UserData>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;

        let file = Client::get_save_file(&client.data_folder_path)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        let data = Client::save_json_as::<UserData>(file, request)
            .await
            .map_err(|e| Status::new(tonic::Code::Internal, e.to_string()))?;

        Ok(Response::new(data))
    }

    async fn get_nat_filter_type(
        &self,
        request: Request<NatFilterRequest>,
    ) -> Result<Response<NatFilterResponse>, Status> {
        let nat_type_res = CoordinatorClient::get_nat_type().await;
        let nat_type = match nat_type_res {
            Ok(nat_type) => nat_type,
            Err(e) => {
                return Err(Status::new(
                    tonic::Code::Internal,
                    format!("Could not perform NAT Test: {}", e),
                ));
            }
        };
        Ok(Response::new(NatFilterResponse {
            r#type: nat_type as i32,
        }))
    }

    async fn open_sftp_channel(
        &self,
        request: Request<SessionData>,
    ) -> Result<Response<SftpRequestResponse>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id.unwrap()) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };

        let res = session_guard.lock().await.request_sftp().await;
        match res {
            Ok(id) => Ok(Response::new(SftpRequestResponse {
                channel_id: id.to_string(),
            })),
            Err(e) => {
                log::error!("Failed to connect to SFTP server: {}", e.to_string());
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
    }

    async fn file_delete(
        &self,
        request: Request<FileDeleteRequest>,
    ) -> Result<Response<FileDeleteResponse>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else {
            return Err(Status::new(tonic::Code::NotFound, "SFTP Session not found"));
        };
        for file_data in request.data {
            if file_data.is_dir {
                if let Err(e) = sftp.remove_dir(file_data.path).await {
                    return Err(Status::new(tonic::Code::Internal, e.to_string()));
                }
            } else {
                if let Err(e) = sftp.remove_file(file_data.path).await {
                    return Err(Status::new(tonic::Code::Internal, e.to_string()));
                }
            }
        }
        Ok(Response::new(FileDeleteResponse {}))
    }

    async fn file_rename(
        &self,
        request: Request<FileRenameRequest>,
    ) -> Result<Response<FileRenameResponse>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else {
            return Err(Status::new(tonic::Code::NotFound, "SFTP Session not found"));
        };
        if let Err(e) = sftp.rename(request.old_path, request.new_path).await {
            return Err(Status::new(tonic::Code::Internal, e.to_string()));
        }
        Ok(Response::new(FileRenameResponse {}))
    }

    async fn list_directory(
        &self,
        request: Request<clientipc::Path>,
    ) -> Result<Response<FileList>, Status> {
        log::info!("Got list dir request!");
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else {
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
                is_dir: entry.metadata().is_dir(),
            });
        }
        Ok(Response::new(FileList { files: list }))
    }

    async fn file_download(
        &self,
        request: Request<FileTransferRequest>,
    ) -> Result<Response<Self::FileDownloadStream>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else {
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
            Err(e) => {
                return Err(Status::new(
                    tonic::Code::Internal,
                    format!("Failed to create local file: {}", e),
                ))
            }
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

    async fn file_upload(
        &self,
        request: Request<FileTransferRequest>,
    ) -> Result<Response<Self::FileUploadStream>, Status> {
        let request = request.into_inner();
        let mut client = self.client.lock().await;
        let session_guard = match client.sessions.get_mut(&request.session_id) {
            Some(session) => session,
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };
        let session = session_guard.lock().await;
        let Some(sftp) = &session.sftp_session else {
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
            Err(e) => {
                return Err(Status::new(
                    tonic::Code::Internal,
                    format!("Failed to create local file: {}", e),
                ))
            }
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

    //Add eventbuses for connections, to monitor their states
    //Also for sessions
    async fn new_connection(
        &self,
        request: Request<NewConnectionRequest>,
    ) -> Result<Response<NewConnectionResponse>, Status> {
        let request = request.into_inner();

        let mut client = self.client.lock().await;

        let res = client.new_connection(request.target_id.clone()).await;
        match res {
            Ok(id) => {
                log::info!("CONN OK");
                Ok(Response::new(NewConnectionResponse {
                    connection_id: request.target_id,
                }))
            }
            Err(e) => {
                log::error!("Failed to connect {}", e);
                Err(Status::new(tonic::Code::Internal, e.to_string()))
            }
        }
    }

    async fn gen_keys(
        &self,
        request: Request<GenKeysRequest>,
    ) -> Result<Response<GenKeysResponse>, Status> {
        let request = request.into_inner();
        let client = self.client.lock().await;

        let res = generate_keypair(
            client.data_folder_path.join("keys"),
            ssh_key::Algorithm::Ed25519,
            "id_ed25519",
        );

        match res {
            Ok(_) => Ok(Response::new(GenKeysResponse {})),
            Err(e) => Err(Status::new(tonic::Code::Internal, e.to_string())),
        }
    }

    async fn get_public_key(
        &self,
        request: Request<GetKeyRequest>,
    ) -> Result<Response<PublicKey>, Status> {
        let request = request.into_inner();
        let client = self.client.lock().await;

        let mut file = File::options()
            .read(true)
            .open(client.data_folder_path.join("keys/id_ed25519.pub"))
            .await?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        let res = reader.read_to_string(&mut contents).await?;

        Ok(Response::new(PublicKey { key: contents }))
    }

    async fn new_session(
        &self,
        request: Request<NewSessionRequest>,
    ) -> Result<Response<NewSessionResponse>, Status> {
        let request = request.into_inner();
        info!("IPC: Requesting new session!");
        let mut client = self.client.lock().await;

        let Some(session_data) = request.session_data else {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "You must specify session data!",
            ));
        };

        let mut session_data_cloned = session_data.clone();

        if session_data.kind.is_none() {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "You must specify the session kind!",
            ));
        };

        let res = client
            .new_session(
                session_data.device_id.clone(),
                session_data_cloned.clone(),
                session_data.username,
                session_data.session_id,
                request.private_key,
                request.known_hosts_path,
            )
            .await;

        info!("IPC: Session requested!");
        match res {
            Ok(id) => {
                //This is kinda messy
                let mut save_file = Client::get_save_file(&client.data_folder_path)
                    .await
                    .unwrap();
                let mut user_data = Client::get_json_as::<UserData>(save_file).await.unwrap();
                session_data_cloned.session_id = Some(id.clone());
                user_data
                    .saved_sessions
                    .insert(id.clone(), session_data_cloned);
                save_file = Client::get_save_file(&client.data_folder_path)
                    .await
                    .unwrap();
                let _ = Client::save_json_as(save_file, user_data).await;

                Ok(Response::new(NewSessionResponse { session_id: id }))
            }
            Err(e) => Err(Status::new(tonic::Code::Internal, e.to_string())),
        }
    }

    async fn open_channel(
        &self,
        request: Request<tonic::Streaming<Msg>>,
    ) -> Result<Response<Self::OpenChannelStream>, Status> {
        let mut stream = request.into_inner();

        let Some(msg_res) = stream.next().await else {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Expected session id as the first message!",
            ));
        };
        let Ok(msg) = msg_res else {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Error while reading initial message",
            ));
        };
        let Some(Type::ChannelInit(channel_init)) = msg.r#type else {
            return Err(Status::new(
                tonic::Code::InvalidArgument,
                "Initial message must be of type ChannelInit",
            ));
        };

        info!("IPC: Opening a channel!");

        let client_clone = self.client.clone();

        let session_id = channel_init.session_id.clone();
        let (mut msg_receiver, server_msg_sender, active, mut event_receiver) = {
            let mut client = client_clone.lock().await;
            let event_receiver = client.event_bus.subscribe().await;
            let session_guard = match client.sessions.get_mut(&channel_init.session_id) {
                Some(session) => session,
                None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
            };

            let mut session = session_guard.lock().await;
            let was_active = session.active;
            if !session.active {
                session.new_session_channel().await.unwrap();
            }

            (
                session.channel_stream.client_messages.subscribe().await,
                session.channel_stream.server_messages.new_sender().await,
                was_active,
                event_receiver,
            )
        };

        let res = async_stream::try_stream! {

            let send_handle = tokio::spawn(async move {
                while let Some(Ok(msg)) = stream.next().await {
                    match msg.r#type {
                        Some(Type::ShellRequest(_) | Type::PtyRequest(_)) if !active => {
                            let _ = server_msg_sender.send(msg);
                        }
                        Some(Type::Data(_) | Type::PtyResize(_)) => {
                            let _ = server_msg_sender.send(msg);
                        }
                        _ => {

                        }
                    }
                }
            });

            loop {
                tokio::select!{
                    msg = msg_receiver.recv() => {
                        match msg {
                            Ok(msg) => {
                                yield msg;
                            }
                            Err(e) => {
                                log::error!("ssh msg receiver error {}", e);
                            }
                        }
                    },
                    Ok(event) = event_receiver.recv() => {
                        if let Some(clientipc::client_event::Kind::Close(e)) = event.kind {
                            if e.id == session_id {break;}
                        }
                    }
                }
            }
            send_handle.abort();

            info!("Message sending loop broken");
        };

        Ok(Response::new(Box::pin(res) as Self::OpenChannelStream))
    }
}

#[cfg(unix)]
pub async fn start_grpc_server(path_str: &str) {
    use std::{future::Future, time::Duration};

    use clientipc::{
        client_event::{self, CloseEvent},
        client_event_service_server::ClientEventServiceServer,
    };
    use futures::{stream::FuturesUnordered, TryFutureExt};
    use log::error;
    use tokio::time;

    let path = path_str.to_string();
    let sock_path = format!("{}/sessio.sock", &path);
    let _ = std::fs::remove_file(&sock_path);

    let uds_res = UnixListener::bind(&sock_path);
    let Ok(uds) = uds_res else {
        info!("Is err {}", uds_res.err().unwrap());
        return;
    };

    let uds_stream = UnixListenerStream::new(uds);

    let client = Client::new(path_str.to_string()).await.unwrap();

    let mut event_receiver = client.event_bus.subscribe().await;
    let event_sender = client.event_bus.new_sender().await;

    let client = Arc::new(Mutex::new(client));

    let client_ipc_handler = ClientIpcHandler {
        client: client.clone(),
    };

    let client_events_handler = ClientEventsHandler {
        client: client.clone(),
    };

    info!("Starting grpc server!");
    let grpc_future = Server::builder()
        .add_service(ClientIpcServer::new(client_ipc_handler))
        .add_service(ClientEventServiceServer::new(client_events_handler))
        .serve_with_incoming(uds_stream);

    let event_listener_future = async {
        while let Ok(event) = event_receiver.recv().await {
            let mut client = client.lock().await;
            let _ = client.handle_event(&event).await;
        }
        Ok(()) as Result<(), Box<dyn std::error::Error>>
    };

    let conn_listener_future = async {
        let mut check_interval = time::interval(Duration::from_secs(5));

        loop {
            check_interval.tick().await;
            let mut client = client.lock().await;

            let mut keys_to_remove: Vec<String> = vec![];
            let conns = &mut client.connections;
            for (device_id, conn) in conns.iter() {
                if let Some(close_reason) = conn.close_reason() {
                    let _ = event_sender.send(ClientEvent {
                        kind: Some(client_event::Kind::Close(CloseEvent {
                            stream_type: client_event::StreamType::Transport.into(),
                            close_reason: format! {"Transport closed {}", close_reason.to_string()},
                            id: device_id.clone(),
                        })),
                    });
                    keys_to_remove.push(device_id.clone());
                }
            }
            for key in keys_to_remove {
                conns.remove(&key);
            }
            if let Some(coordinator) = client.coordinator.as_mut() {
                if let Some(_) = coordinator.c_client.is_closed() {
                    _ = coordinator.reconnect().await;
                }
            };
        }

        Ok(()) as Result<(), Box<dyn std::error::Error>>
    };

    let (grpc_result, event_result, _) =
        tokio::join!(grpc_future, event_listener_future, conn_listener_future);

    if let Err(e) = grpc_result {
        error!("gRPC server encountered an error: {:?}", e);
    }

    if let Err(e) = event_result {
        error!("Event listener encountered an error: {:?}", e);
    }

    info!("exited grpc server");
}

#[cfg(windows)]
pub async fn start_grpc_server(path_str: &str) {
    let _ = std::fs::remove_file(path_str);

    let greeter = ClientIpcHandler {
        client: Arc::new(Mutex::new(Client::default())),
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
