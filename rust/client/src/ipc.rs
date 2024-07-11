pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod clientipc {
    tonic::include_proto!("clientipc");
}

use futures::{stream, Stream, StreamExt};
use url::Url;
use uuid::Uuid;
use tokio::{io::AsyncReadExt, sync::mpsc};
use std::{pin::Pin, sync::Arc};
use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

use clientipc::{
    client_ipc_server::{ClientIpc, ClientIpcServer},
    Msg, StreamResponse, msg::Type,
    GenKeysRequest,GenKeysResponse,
    NewSessionRequest, NewSessionResponse,
    NewConnectionRequest, NewConnectionResponse,
};

use log::info;
use tokio::sync::Mutex;
use crate::client::Client;

#[cfg(windows)]
use crate::platform::windows::UnixListenerStream;
#[cfg(windows)]
use uds_windows::UnixListener;

use tokio::io::AsyncWriteExt;

#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
#[cfg(unix)]
use tokio::net::UnixListener;

use tonic::{Request, Status, Response, transport::Server};

use std::path::Path;

use common::utils::keygen::generate_keypair;

struct ClientIpcHandler {
    client: Arc<Mutex<Client>>
}


#[tonic::async_trait]
impl ClientIpc for ClientIpcHandler {
    type OpenChannelStream =
        Pin<Box<dyn Stream<Item = Result<Msg, Status>> + Send  + 'static>>;

    async fn new_connection(&self, request: Request<NewConnectionRequest>) 
    -> Result<Response<NewConnectionResponse>, Status> {
        let request = request.into_inner();

        let Ok(url) = request.coordinator_url.parse() else {
            return Err(Status::new(tonic::Code::InvalidArgument, "Invalid coordinator URL!"));
        };

        let mut client = self.client.lock().await;
        let res = client.new_connection(request.target_id, url).await;
        match res {
            Ok(id) => {
                Ok(Response::new(NewConnectionResponse{
                    connection_id: id.to_string()
                }))
            }
            Err(e) => {
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

        let mut client = self.client.lock().await;

        let res = client.new_session(uuid::Uuid::parse_str(&request.connection_id).unwrap(), request.username, request.private_key).await;
        match res {
            Ok(id) => {
                Ok(Response::new(NewSessionResponse{
                    session_id: id.to_string()
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

        let session_id = match uuid::Uuid::parse_str(&channel_init.session_id) {
            Ok(id) => id,
            Err(_) => return Err(Status::new(tonic::Code::InvalidArgument, "Invalid session ID format")),
        };
        
        let client_clone = self.client.clone();

        let mut client = client_clone.lock().await;
        
        let session_guard = match client.sessions.get_mut(&session_id) {
            Some(session) => Arc::new(Mutex::new(session)),
            None => return Err(Status::new(tonic::Code::NotFound, "Session not found")),
        };

        let session = session_guard.lock().await;

        let channel_id = session.lock().await.new_channel().await.unwrap();

        let channel = session.lock().await.get_channel(&channel_id);

        let session_clone = session.clone();

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
                                    input_tx.send(data.payload).await.unwrap();
                                }
                                Some(Type::PtyRequest(req)) => {
                                    let mut session = session_clone.lock().await;
                                    let _ = session.request_pty(req.col_width, req.row_height).await;
                                }
                                Some(Type::ShellRequest(req)) => {
                                    let mut session = session_clone.lock().await;

                                    //ignoring until tests complete
                                    let _ = session.request_shell(&channel_id, input_guard.clone(), o_clone.clone()).await;
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
            });

            // Process output from the shell and send back to the ipc listener
            while let Some(data) = output_rx.recv().await {
                let msg = Msg {
                    r#type: Some(Type::Data(crate::ipc::clientipc::msg::Data { payload: data })),
                };
                yield msg;
            }
        };

        Ok(Response::new(Box::pin(res) as Self::OpenChannelStream))        
    }
}



#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/tmp/tonic/helloworld";

    std::fs::create_dir_all(Path::new(path).parent().unwrap())?;

    let uds = UnixListener::bind(path)?;
    let uds_stream = UnixListenerStream::new(uds);

    let greeter = ClientIpcHandler {
        client: Arc::new(Mutex::new(Client::default()))
    };

    Server::builder()
        .add_service(ClientIpcServer::new(greeter))
        .serve_with_incoming(uds_stream)
        .await?;

    Ok(())
}


pub fn start_grpc_server(path_str: &str) {
    let path = path_str.to_string();
    info!("Here 1");
    let _ = std::fs::remove_file(path_str);
    std::thread::spawn(move || {
        info!("Here 2");
        //Static runtime: https://stackoverflow.com/questions/68317698/how-to-reuse-tokio-runtime-in-rust-ffi-library
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            info!("Here 3");
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
        });
    });
}