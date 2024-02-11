use std::{fmt::Display, sync::Arc};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

use crate::state::Value;

mod command;


#[derive(Debug)]
enum CmdResponseStatus {
    Success(Arc<dyn Value>),
    Failure
}

impl Display for CmdResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CmdResponseStatus::Success(v) => format!("Success({})\n", v),
            CmdResponseStatus::Failure => format!("Failure\n"),
        })
    }
}

pub async fn accept_cmd_connections(cmd_listener: TcpListener) {
    log::info!("Listening command connections on port 7777");

    loop {
        let (socket, addr) = match cmd_listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                log::error!("Failed to accept command connection {:?}", e);
                continue;
            }
        };

        log::info!("Accepted command connection from {}", addr);

        tokio::spawn(async move { listen_for_commands(socket).await });
    }
}

async fn listen_for_commands(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => {
                log::warn!("Empty payload. Probably client forcefully closed the connection.");
                return;
            },
            Ok(n) => n,
            Err(e) => {
                log::error!("Failed to read from socket; err = {:?}", e);
                return;
            }
        };

        let status = handle_command(&buf[..n]);
        
        if let Err(e) = socket.write_all(status.to_string().as_bytes()).await {
            log::error!("Failed to responde to client; err = {:?}", e);
            return;
        }
    }
}


fn handle_command(command: &[u8]) -> CmdResponseStatus {
    if let Ok(data) = std::str::from_utf8(command) {
        let data = data.trim_matches(char::from(0)).trim();
        let parts: Vec<&str> = data.split_whitespace().collect();
        
        match parts.as_slice() {
            ["SET", key, value] => {
                match command::set(*key, *value) {
                    Ok(_) => CmdResponseStatus::Success(Arc::new(())),
                    Err(_) => CmdResponseStatus::Failure,
                }
            },
            ["GET", key] => {
                match command::get(*key) {
                    Ok(Some(v)) => CmdResponseStatus::Success(v.clone()),
                    Ok(None) => CmdResponseStatus::Success(Arc::new(())),
                    Err(_) => CmdResponseStatus::Failure
                }
            },
            _ => { 
                log::error!("Incorrect format"); 
                CmdResponseStatus::Failure
            }  
        }
    } else {
        log::error!("Data is not valid UTF-8");
        CmdResponseStatus::Failure
    }
}
