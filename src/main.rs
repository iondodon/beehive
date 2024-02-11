use tokio::{net::TcpListener, try_join};

mod state;
mod client_server;
mod control_server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cc_listener = TcpListener::bind("127.0.0.1:6666").await?;
    let cc_task = tokio::spawn(async move { 
        control_server::accept_control_connections(cc_listener).await 
    });

    let cmd_listener = TcpListener::bind("127.0.0.1:7777").await?;
    let acc_taks = tokio::spawn(async move { 
        client_server::accept_cmd_connections(cmd_listener).await 
    });

    try_join!(cc_task, acc_taks)?;

    Ok(())
}
