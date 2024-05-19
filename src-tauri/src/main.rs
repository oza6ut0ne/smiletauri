// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

async fn process_request(stream: &mut TcpStream, _addr: SocketAddr) -> Result<(), std::io::Error> {
    let mut received = Vec::with_capacity(1024);
    let mut buf = vec![0; 1024];

    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("{}", String::from_utf8_lossy(&received));
                break;
            }
            Ok(n) => received.append(&mut buf[0..n].to_vec()),
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return Err(e);
            }
        };
    }
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let bind_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:2525".to_string());
    let listener = TcpListener::bind(bind_addr).await?;

    let _task = tokio::spawn(async move {
        loop {
            let (mut socket, addr) = listener.accept().await?;
            tokio::spawn(async move { process_request(&mut socket, addr).await });
        }

        #[allow(unreachable_code)]
        Ok::<(), std::io::Error>(())
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
