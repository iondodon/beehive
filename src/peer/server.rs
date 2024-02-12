use tokio::net::TcpListener;

use super::command;

pub async fn accept_peer_connections(cc_listener: TcpListener) {
    log::info!("Listening peer connections on port 6666");

    loop {
        let (socket, addr) = match cc_listener.accept().await {
            Ok((socket, addr) )=> (socket, addr),
            Err(e) => {
                log::error!("Failed to accept peer connection {:?}", e);
                continue;
            }
        };

        log::info!("Accepted control connection from peer {}", addr);

        tokio::spawn(async move { command::listen_control_commands(socket).await });
    }
}