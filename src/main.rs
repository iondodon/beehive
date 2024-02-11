use tokio::{net::TcpListener, try_join};

mod state;
mod client_server;
mod peer_server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let peer_listener = TcpListener::bind("127.0.0.1:6666").await?;
    let apc_task = tokio::spawn(async move { 
        peer_server::accept_peer_connections(peer_listener).await 
    });

    let client_listener = TcpListener::bind("127.0.0.1:7777").await?;
    let acc_taks = tokio::spawn(async move { 
        client_server::accept_client_connections(client_listener).await 
    });

    try_join!(apc_task, acc_taks)?;

    Ok(())
}
