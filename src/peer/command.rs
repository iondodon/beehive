use std::fmt::Display;
use std::fs::Permissions;

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
enum PeerCmdResponce {
    Success,
    Failure
}

impl Display for PeerCmdResponce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PeerCmdResponce::Success => format!("Success\n"),
            PeerCmdResponce::Failure => format!("Failure\n"),
        })
    }
}

pub async fn listen_control_commands(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => {
                log::warn!("Empty payload. Probably the peer forcefully closed the connection.");
                return;
            },
            Ok(n) => n,
            Err(e) => {
                log::error!("Failed to read from socket; err = {:?}", e);
                return;
            }
        };

        let status = handle_control_command(&buf[..n]);
        
        if let Err(e) = socket.write_all(status.to_string().as_bytes()).await {
            log::error!("Failed to responde to the peer; err = {:?}", e);
            return;
        }
    }
}

fn handle_control_command(command: &[u8]) -> PeerCmdResponce {
    if let Ok(cmd) = std::str::from_utf8(command) {
        let cmd = cmd.trim_matches(char::from(0)).trim();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts.as_slice() {
            [] => todo!(),
            _ => todo!()
        }
    } else {
        log::error!("Data is not valid UTF-8");
        PeerCmdResponce::Failure
    }
}