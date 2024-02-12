use tokio::net::TcpListener;

use super::command;

pub async fn accept_client_connections(cmd_listener: TcpListener) {
    log::info!("Listening client connections on port 7777");

    loop {
        let (socket, addr) = match cmd_listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                log::error!("Failed to accept client connection {:?}", e);
                continue;
            }
        };

        log::info!("Accepted connection from client {}", addr);

        tokio::spawn(async move { command::listen_for_commands(socket).await });
    }
}