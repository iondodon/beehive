use tokio::net::TcpListener;

mod command;

pub async fn accept_control_connections(cc_listener: TcpListener) {
    log::info!("Listening control connections on port 6666");

    loop {
        let (_socket, addr) = match cc_listener.accept().await {
            Ok((socket, addr) )=> (socket, addr),
            Err(e) => {
                log::error!("Failed to accept command connection {:?}", e);
                continue;
            }
        };

        log::info!("Accepted control connection from {}", addr);
    }
}