use std::sync::{PoisonError, RwLockWriteGuard};

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::spawn;

mod state;

use state::{State, Value, STATE};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    log::info!("Server running on localhost:7878");

    loop {
        let (mut socket, addr) = listener.accept().await?;

        log::info!("Accepted connection from {}", addr);

        spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                handle_command(&buf[..n]);
                
                if let Err(e) = socket.write_all(&buf[..n]).await {
                    log::error!("Failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

fn handle_command(command: &[u8])  {
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
                    let _val = get(*key);
                    
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
    let (key, value) = (key.to_string(), value.to_string());

    let mut state_lock = STATE.write()?;

    let state_map = &mut state_lock.state_map;

    state_map.insert(key, Box::new(value));
    Ok(())
}

fn get(key: &str) -> Option<Value>{
    let key = key.to_string();

    let state_lock = STATE.read();

    if state_lock.is_err() {
        return None;
    }

    let state_lock = state_lock.unwrap();
    let state_map = &state_lock.state_map;

    let a = state_map.get(&key);
    let b = a.expect("adasdsads");

    Some(b.clone())
}