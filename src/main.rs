use std::sync::{Arc, PoisonError, RwLockReadGuard, RwLockWriteGuard};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

mod state;

use state::{State, Value, STATE};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cmd_listener = TcpListener::bind("127.0.0.1:7777").await?;
    tokio::spawn(async move { accept_cmd_connections(cmd_listener).await }).await?;

    Ok(())
}

async fn accept_cmd_connections(cmd_listener: TcpListener) {
    log::info!("Listening commands on port 7777");

    loop {
        let (socket, addr) = match cmd_listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(_) => {
                log::error!("Failed to accept command connection");
                continue;
            }
        };

        log::info!("Accepted connection from {}", addr);

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

        let _ = handle_command(&buf[..n]);
        
        if let Err(e) = socket.write_all(&buf[..n]).await {
            log::error!("Failed to write to socket; err = {:?}", e);
            return;
        }
    }
}

fn handle_command(command: &[u8]) {
    if let Ok(data) = std::str::from_utf8(command) {
        let data = data.trim_matches(char::from(0)).trim();
        let parts: Vec<&str> = data.split_whitespace().collect();
        
        match parts.len() {
            3 => match parts.as_slice() {
                ["SET", key, value] => {
                    set(*key, *value).unwrap();
                }
                _ => log::error!("Unknown command or incorrect format"),
            },
            2 => match parts.as_slice() {
                ["GET", key] => {
                    match get(*key) {
                        Ok(Some(val)) => log::debug!("{}", val),
                        Ok(None) => log::debug!("Not found"),
                        Err(e) => log::debug!("{}", e)
                    };
                },
                _ => log::error!("Unknown command or incorrect format"),
            }
            _ => { 
                log::error!("Incorrect format"); 
            }
        }
    } else {
        log::error!("Data is not valid UTF-8");
    }
}

fn set(key: &str, value: &str) -> Result<(), PoisonError<RwLockWriteGuard<'static, State>>> {
    log::info!("SET {} to {}", key, value);

    let (key, value) = (key.to_string(), value.to_string());

    let mut state_lock = STATE.write()?;

    let store = &mut state_lock.store;

    store.insert(key, Arc::new(value));
    Ok(())
}

fn get(key: &str) -> Result<Option<Arc<dyn Value>>, PoisonError<RwLockReadGuard<'static, State>>> {
    log::info!("GET {}", key);

    let key = key.to_string();
     
    let lock = STATE.read()?;

    return match lock.store.get(&key) {
        Some(val) => Ok(Some(val.clone())),
        None => Ok(None)
    };
}