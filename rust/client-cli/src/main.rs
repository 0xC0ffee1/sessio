use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crossterm::terminal;
use hyper_util::rt::TokioIo;
use tonic::transport::{Endpoint, Uri};
use tonic::transport::Channel;
use tonic::{Request};
use tokio::sync::mpsc;
use tokio::net::UnixStream;
use clientipc::client_ipc_client::ClientIpcClient;
use clientipc::{
    SessionRequest, LocalPortForwardRequest, FileTransferRequest, SessionData,
    NewSessionRequest, Msg, NewConnectionRequest, CoordinatorStartRequest,
    AccountData, AccountDataRequest, InstallRequest, InstallResponse,
    GenKeysRequest, GetKeyRequest, CoordinatorStatusRequest
};
use tower::service_fn;
use prettytable::{Table, row, cell};
use tokio::io::{AsyncReadExt};
use std::time::Duration;

use crossterm::{
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, size as terminal_size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, read, Event, KeyCode}
};
use log::info;

pub mod clientipc {
    tonic::include_proto!("clientipc");
}

// Helper functions for colored output
fn log(msg: &str) {
    eprintln!("[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

fn error(msg: &str) {
    eprintln!("\x1b[31m[ERROR]\x1b[0m {}", msg);
}

fn warning(msg: &str) {
    eprintln!("\x1b[33m[WARNING]\x1b[0m {}", msg);
}

fn success(msg: &str) {
    eprintln!("\x1b[32m[SUCCESS]\x1b[0m {}", msg);
}

#[derive(Parser)]
#[command(name = "sessio", about = "CLI to interact with sessio-clientd")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List devices and active port forwards
    List,

    /// Connect to interactive shell on device (ephemeral)
    Shell {
        device_id: String,
    },

    /// Start SFTP session for file operations (ephemeral)
    Sftp {
        device_id: String,
    },

    /// Port forwarding management (persistent)
    Forward {
        #[command(subcommand)]
        action: ForwardAction,
    },

    /// File operations such as push and pull
    File {
        #[command(subcommand)]
        operation: FileOperation,
    },

    /// Install the client with an install key
    Install {
        /// The install key provided by the coordinator
        #[arg(long, short = 'k')]
        install_key: String,
        
        /// The coordinator URL
        #[arg(long, short = 'c', default_value = "http://127.0.0.1:2223")]
        coordinator: String
    },

    /// Show status of client daemon, server, and available devices
    Status,

    /// Server management commands
    Server {
        #[command(subcommand)]
        command: ServerCommands,
    },
}

#[derive(Subcommand)]
enum ServerCommands {
    /// Start the sessio server service
    Start,
    
    /// Stop the sessio server service
    Stop,
    
    /// Restart the sessio server service
    Restart,
    
    /// Show status of the sessio server service
    Status,
    
    /// Uninstall the sessio server from the system
    Uninstall {
        /// Remove all configuration and data files
        #[arg(long)]
        purge: bool,
    },
}

#[derive(Subcommand)]
enum ForwardAction {
    /// Start port forwarding
    Start {
        device_id: String,
        #[arg(help = "local_port:remote_host:remote_port (e.g., 8080:localhost:80)")]
        port_spec: String,
    },
    /// Stop port forwarding
    Stop {
        device_id: String,
        local_port: u16,
    },
    /// List active port forwards
    List,
}

#[derive(Subcommand)]
enum FileOperation {
    /// Push a file to the remote device
    Push {
        file_path: String,
        remote_path: String,
        device_id: String,
    },

    /// Pull (download) a file from the remote device
    Pull {
        remote_path: String,
        file_path: String,
        device_id: String,
    },
}


async fn new_session(client: &mut ClientIpcClient<Channel>, session_data: SessionData) -> anyhow::Result<String>{
    let connection_request = tonic::Request::new(NewConnectionRequest {
        coordinator_url: "".into(),
        target_id: session_data.device_id.clone(),
        own_ipv6: None,
    });

    client.start_coordinator(CoordinatorStartRequest{}).await?;
    let connection_response = client.new_connection(connection_request).await?;

    // Request a new session from the server
    let request = tonic::Request::new(NewSessionRequest {
        private_key: "/home/s/.sessio/keys/id_ed25519".to_string(),  
        known_hosts_path: "".to_string(), 
        session_data: Some(session_data),
    });

    let response = client.new_session(request).await?;

    // Retrieve session ID
    let session_id = response.into_inner().session_id;
    
    Ok(session_id)
}

async fn start_interactive_shell(client: &mut ClientIpcClient<Channel>, session_id: String) -> anyhow::Result<()> {
    // Initialize Crossterm for terminal manipulation
    let mut stdout_std = std::io::stdout();

    // Enable raw mode and enter the alternate screen
    enable_raw_mode().unwrap();
    execute!(stdout_std, EnterAlternateScreen, Clear(ClearType::All)).unwrap();

    // Get terminal size
    let (w, h) = terminal_size()?;

    // Create an mpsc channel for sending PTY requests to the gRPC server
    let (tx, mut rx) = mpsc::channel(32);

    let outbound = async_stream::stream! {
        yield Msg {
            r#type: Some(clientipc::msg::Type::ChannelInit(
                clientipc::msg::ChannelInit {
                    session_id: session_id.to_string()
                },
            )),
        };
        while let Some(msg) = rx.recv().await {
            yield msg;
        }
    };

    // Create a gRPC stream for the PTY session
    let mut stream = client
        .open_channel(Request::new(outbound))
        .await?
        .into_inner();

    // Send the initial PTY request with terminal size
    let initial_pty_request = Msg {
        r#type: Some(clientipc::msg::Type::PtyRequest(
            clientipc::msg::PtyRequest {
                col_width: w as u32,
                row_height: h as u32,
            },
        )),
    };

    tx.send(initial_pty_request).await?;

    // Immediately send shell request
    tx.send(Msg {
        r#type: Some(clientipc::msg::Type::ShellRequest(clientipc::msg::ShellRequest{})),
    }).await?;

    tx.send(Msg {
        r#type: Some(clientipc::msg::Type::PtyResize(clientipc::msg::PtyResize{
            col_width: w as u32,
            row_height: h as u32
        })),
    }).await?;

    let event_tx = tx.clone();

    let mut stdin = tokio::io::stdin();
    tokio::spawn(async move {
        let mut stdin_closed = false;
        let mut buf = vec![0; 1024];
        loop {
            tokio::select! {
                r = stdin.read(&mut buf), if !stdin_closed => {
                    match r {
                        Ok(0) => {
                            stdin_closed = true;
                        },
                        Ok(n) => { 
                            let data = &buf[..n];
                            tx.send(Msg {
                                r#type: Some(clientipc::msg::Type::Data(clientipc::msg::Data{
                                    payload: data.to_vec()
                                })),
                            }).await;
                        },
                        Err(e) => {
                            
                        }
                    };
                }
            }
        }
    });

    loop {
        tokio::select! {
            msg = stream.message() => {
                if let Ok(Some(msg)) = msg {
                    if let Some(clientipc::msg::Type::Data(data)) = msg.r#type {
                        stdout_std.write_all(&data.payload)?;
                        stdout_std.flush()?;
                    }
                } else {
                    break;
                }
            }

            // Poll for terminal events such as resizing
            _ = tokio::time::sleep(Duration::from_millis(100)), if event::poll(Duration::from_millis(0))? => {
                if let Event::Resize(width, height) = event::read()? {
                    event_tx.send(Msg {
                        r#type: Some(clientipc::msg::Type::PtyResize(clientipc::msg::PtyResize{
                            col_width: width as u32,
                            row_height: height as u32
                        })),
                    }).await;
                }
            }
        }
    }
    
    // Exit the alternate screen and disable raw mode
    disable_raw_mode().unwrap();
    execute!(stdout_std, LeaveAlternateScreen).unwrap();
    
    Ok(())
}



async fn install_client(
    client: &mut ClientIpcClient<Channel>,
    install_key: String, 
    coordinator: String
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Installing client with install key...");
    
    let install_request = InstallRequest {
        install_key,
        coordinator_url: coordinator
    };
    
    let request = tonic::Request::new(install_request);
    let response = client.install(request).await?;
    let install_response = response.into_inner();
    
    if install_response.success {
        success("Installation successful!");
        println!("Device ID: {}", install_response.device_id);
        println!("Client is now registered and ready to use.");
        Ok(())
    } else {
        let error_msg = install_response.error.unwrap_or_else(|| "Unknown error".to_string());
        error(&format!("Installation failed: {}", error_msg));
        Err(error_msg.into())
    }
}

async fn show_status(client: &mut ClientIpcClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sessio Status");
    println!("=============");
    
    // Check client daemon status
    let client_status = check_service_status("sessio-clientd", false).await;
    println!("\nClient Daemon: {}", format_service_status(&client_status));
    
    // Check server status
    let server_status = check_service_status("sessio-server", true).await;
    println!("Server:        {}", format_service_status(&server_status));
    
    // Get account info from client daemon
    match client.get_account_data(AccountDataRequest {}).await {
        Ok(response) => {
            let account_data = response.into_inner();
            if account_data.is_registered {
                println!("\nDevice ID:     {}", account_data.device_id);
                println!("Coordinator:   {}", account_data.coordinator_url);
                
                // Fetch devices from coordinator using gRPC
                let coord_request = tonic::Request::new(CoordinatorStatusRequest {});
                match client.get_coordinator_status(coord_request).await {
                    Ok(response) => {
                        let devices = response.into_inner().devices;
                        if !devices.is_empty() {
                            println!("\nAvailable Devices:");
                            let mut table = Table::new();
                            table.add_row(row!["DEVICE ID", "OS", "STATUS", "CATEGORIES"]);
                            
                            for device in devices {
                                let status = if device.is_online { "Online" } else { "Offline" };
                                let categories = if device.categories.is_empty() {
                                    "-".to_string()
                                } else {
                                    device.categories.join(", ")
                                };
                                
                                table.add_row(row![device.device_id, device.os_name, status, categories]);
                            }
                            
                            table.printstd();
                        }
                        else {
                            println!("\nNo devices found");
                        }
                    }
                    Err(e) => {
                        eprintln!("\nFailed to fetch devices from coordinator: {}", e);
                    }
                }
            } else {
                println!("\nNot registered. Run 'sessio install' to register this device.");
            }
        }
        Err(e) => {
            eprintln!("\nFailed to get account data: {}", e);
        }
    }
    
    Ok(())
}


#[derive(Debug)]
struct ServiceStatus {
    is_active: bool,
    is_enabled: bool,
    status_text: String,
}

async fn check_service_status(service_name: &str, system_wide: bool) -> ServiceStatus {
    use tokio::process::Command;
    
    let systemctl_cmd = if system_wide { "systemctl" } else { "systemctl" };
    let systemctl_args = if system_wide { 
        vec!["is-active", service_name] 
    } else { 
        vec!["--user", "is-active", service_name] 
    };
    
    // Check if service is active
    let is_active = Command::new(systemctl_cmd)
        .args(&systemctl_args)
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    // Check if service is enabled
    let enabled_args = if system_wide { 
        vec!["is-enabled", service_name] 
    } else { 
        vec!["--user", "is-enabled", service_name] 
    };
    
    let is_enabled = Command::new(systemctl_cmd)
        .args(&enabled_args)
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    let status_text = match (is_active, is_enabled) {
        (true, true) => "Active (enabled)".to_string(),
        (true, false) => "Active (disabled)".to_string(),
        (false, true) => "Inactive (enabled)".to_string(),
        (false, false) => "Inactive (disabled)".to_string(),
    };
    
    ServiceStatus {
        is_active,
        is_enabled,
        status_text,
    }
}

fn format_service_status(status: &ServiceStatus) -> String {
    if status.is_active {
        format!("{} {}", "✓", status.status_text)
    } else {
        format!("{} {}", "✗", status.status_text)
    }
}

async fn handle_server_command(command: ServerCommands) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::process::Command;
    
    match command {
        ServerCommands::Start => {
            log("Starting sessio-server service...");
            let output = Command::new("sudo")
                .args(&["systemctl", "start", "sessio-server"])
                .output()
                .await?;
            
            if output.status.success() {
                success("Server started successfully");
            } else {
                error("Failed to start server");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        ServerCommands::Stop => {
            log("Stopping sessio-server service...");
            let output = Command::new("sudo")
                .args(&["systemctl", "stop", "sessio-server"])
                .output()
                .await?;
            
            if output.status.success() {
                success("Server stopped successfully");
            } else {
                error("Failed to stop server");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        ServerCommands::Restart => {
            log("Restarting sessio-server service...");
            let output = Command::new("sudo")
                .args(&["systemctl", "restart", "sessio-server"])
                .output()
                .await?;
            
            if output.status.success() {
                success("Server restarted successfully");
            } else {
                error("Failed to restart server");
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        ServerCommands::Status => {
            let status = check_service_status("sessio-server", true).await;
            println!("Sessio Server: {}", format_service_status(&status));
            
            // Show detailed status
            let output = Command::new("sudo")
                .args(&["systemctl", "status", "sessio-server", "--no-pager"])
                .output()
                .await?;
            
            println!("\n{}", String::from_utf8_lossy(&output.stdout));
        }
        
        ServerCommands::Uninstall { purge } => {
            log("Uninstalling sessio-server...");
            
            // Stop the service first
            Command::new("sudo")
                .args(&["systemctl", "stop", "sessio-server"])
                .output()
                .await?;
            
            // Disable the service
            Command::new("sudo")
                .args(&["systemctl", "disable", "sessio-server"])
                .output()
                .await?;
            
            // Remove service file
            Command::new("sudo")
                .args(&["rm", "-f", "/etc/systemd/system/sessio-server.service"])
                .output()
                .await?;
            
            // Remove binary
            Command::new("sudo")
                .args(&["rm", "-f", "/usr/local/bin/sessio-server"])
                .output()
                .await?;
            
            if purge {
                log("Removing configuration and data files...");
                
                // Remove config directory
                Command::new("sudo")
                    .args(&["rm", "-rf", "/etc/sessio"])
                    .output()
                    .await?;
                
                // Remove data directory
                Command::new("sudo")
                    .args(&["rm", "-rf", "/var/lib/sessio"])
                    .output()
                    .await?;
                
                // Remove user
                Command::new("sudo")
                    .args(&["userdel", "sessio"])
                    .output()
                    .await?;
            }
            
            // Reload systemd
            Command::new("sudo")
                .args(&["systemctl", "daemon-reload"])
                .output()
                .await?;
            
            success("Server uninstalled successfully");
            if !purge {
                println!("Configuration files were preserved. Use --purge to remove them.");
            }
        }
    }
    
    Ok(())
}

//TODO: connect to session ID if it's not yet been connected to, show inactive state in list, 
//fix reconnections so that it doesnt always ask for new shell
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Connect to the client daemon's Unix socket
    let socket_path = "/home/s/.sessio/sessio.sock";
    
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket_path = socket_path.to_string();
            async move {
                let stream = UnixStream::connect(socket_path).await
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::ConnectionRefused, 
                        format!("Failed to connect to sessio daemon: {}", e)))?;
                Ok::<_, std::io::Error>(TokioIo::new(stream))
            }
        }))
        .await?;
    
    let mut client = ClientIpcClient::new(channel);

    match cli.command {
        Commands::List => {
            // Get device status from coordinator
            let coord_request = tonic::Request::new(CoordinatorStatusRequest {});
            let coord_response = client.get_coordinator_status(coord_request).await;
            
            let mut device_info: std::collections::HashMap<String, (bool, Vec<String>)> = std::collections::HashMap::new();

            match coord_response {
                Ok(response) => {
                    // Add devices from coordinator with their online status
                    for device in response.into_inner().devices {
                        device_info.insert(device.device_id, (device.is_online, Vec::new()));
                    }
                }
                Err(e) => {
                    warning(&format!("Failed to get coordinator status: {}", e));
                    warning("Showing local session data only");
                }
            }



            // Get active sessions to show local services
            let session_request = tonic::Request::new(SessionRequest::default());
            let session_response = client.get_active_sessions(session_request).await?;

            let session_map = session_response.into_inner();
            let sessions = session_map.map;

            // Add services per device
            for (_session_id, session_data) in sessions {
                let entry = device_info.entry(session_data.device_id.clone()).or_insert((false, Vec::new()));

                match session_data.kind {
                    Some(clientipc::session_data::Kind::Pty(_)) => {
                        if session_data.active {
                            entry.1.push("Shell".to_string());
                        }
                    },
                    Some(clientipc::session_data::Kind::Sftp(_)) => {
                        if session_data.active {
                            entry.1.push("SFTP".to_string());
                        }
                    },
                    Some(clientipc::session_data::Kind::Lpf(lpf)) => {
                        let forward_info = format!("Forward({}->{}:{})",
                                                  lpf.local_port, lpf.remote_host, lpf.remote_port);
                        entry.1.push(forward_info);
                    },
                    None => {},
                };
            }

            // Display devices and their services
            println!("DEVICES:");
            let mut device_table = Table::new();
            device_table.add_row(row!["DEVICE ID", "STATUS", "ACTIVE SERVICES"]);

            for (device_id, (is_online, services)) in device_info {
                let status = if is_online { "Online" } else { "Offline" };
                let services_str = if services.is_empty() {
                    "-".to_string()
                } else {
                    services.join(", ")
                };
                device_table.add_row(row![device_id, status, services_str]);
            }

            device_table.printstd();
        }
        
        Commands::Shell { device_id } => {
            println!("Connecting to shell on {}...", device_id);
            let session_data = SessionData {
                device_id: device_id.to_string(),
                username: "root".into(),
                kind: Some(clientipc::session_data::Kind::Pty(clientipc::session_data::PtySession {})),
                ..Default::default()
            };
            
            let session_id = new_session(&mut client, session_data).await?;
            start_interactive_shell(&mut client, session_id).await?;
        }
        
        Commands::Sftp { device_id } => {
            println!("Starting SFTP session on {}...", device_id);
            let session_data = SessionData {
                device_id: device_id.to_string(),
                username: "root".into(),
                kind: Some(clientipc::session_data::Kind::Sftp(clientipc::session_data::SftpSession {})),
                ..Default::default()
            };
            
            let session_id = new_session(&mut client, session_data).await?;
            println!("SFTP session created with ID: {}", session_id);
            // TODO: Implement SFTP interactive session
        }
        
        Commands::Install { install_key, coordinator } => {
            install_client(&mut client, install_key, coordinator).await?;
        }
        
        Commands::Status => {
            show_status(&mut client).await?;
        }
        
        Commands::Server { command } => {
            handle_server_command(command).await?;
        }
        
        Commands::Forward { action } => {
            match action {
                ForwardAction::Start { device_id, port_spec } => {
                    let parts: Vec<&str> = port_spec.split(':').collect();
                    if parts.len() != 3 {
                        error("Invalid port spec. Use format: local_port:remote_host:remote_port");
                        return Ok(());
                    }
                    
                    let local_port: u16 = parts[0].parse().map_err(|_| {
                        error("Invalid local port number");
                    }).unwrap_or(0);
                    
                    let remote_host = parts[1].to_string();
                    let remote_port: u16 = parts[2].parse().map_err(|_| {
                        error("Invalid remote port number");
                    }).unwrap_or(0);
                    
                    if local_port == 0 || remote_port == 0 {
                        return Ok(());
                    }
                    
                    println!("Starting port forwarding: localhost:{} -> {}:{}", 
                             local_port, remote_host, remote_port);
                    
                    let session_data = SessionData {
                        device_id: device_id.to_string(),
                        username: "root".into(),
                        kind: Some(clientipc::session_data::Kind::Lpf(clientipc::session_data::LpfSession {
                            local_host: "127.0.0.1".to_string(),
                            local_port: local_port as u32,
                            remote_host,
                            remote_port: remote_port as u32,
                        })),
                        ..Default::default()
                    };
                    
                    let session_id = new_session(&mut client, session_data).await?;
                    
                    // Start the local port forward
                    let lpf_request = tonic::Request::new(SessionData {
                        session_id: Some(session_id.clone()),
                        device_id: device_id.to_string(),
                        username: "root".into(),
                        kind: Some(clientipc::session_data::Kind::Lpf(clientipc::session_data::LpfSession {
                            local_host: "127.0.0.1".to_string(),
                            local_port: local_port as u32,
                            remote_host: parts[1].to_string(),
                            remote_port: remote_port as u32,
                        })),
                        active: true,
                    });
                    
                    client.local_port_forward(lpf_request).await?;
                    success(&format!("Port forwarding active: localhost:{} -> {}:{}", 
                                   local_port, parts[1], remote_port));
                    println!("Press Ctrl+C to stop");
                    
                    // Keep the process running
                    tokio::signal::ctrl_c().await?;
                    println!("\nStopping port forwarding...");
                }
                
                ForwardAction::Stop { device_id, local_port } => {
                    println!("Stopping port forward on {} port {}", device_id, local_port);
                    // TODO: Implement stop functionality - need to track active forwards
                }
                
                ForwardAction::List => {
                    println!("Listing active port forwards...");
                    // TODO: Implement list functionality - need to track active forwards
                }
            }
        }
        
        _ => {}
    }
    
    Ok(())
}
