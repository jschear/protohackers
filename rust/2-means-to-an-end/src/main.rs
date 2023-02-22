use anyhow::Result;
use core::mem::size_of;
use futures::prelude::*;
use std::mem::size_of;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[derive(Debug)]
enum Message {
    Insert { timestamp: i32, price: i32 },
    Query { mintime: i32, maxtime: i32 },
}

impl TryFrom<[u8; 9]> for Message {
    type Error = &'static str;

    fn try_from(buf: [u8; 9]) -> Result<Message, &'static str> {
        let (query_type, buf) = buf.split_first().unwrap();
        match query_type {
            b'I' => {
                // TODO
            }
            b'Q' => {
                // TODO
            }
            _ => Err("Unknown query type."),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) {
    let mut buf = [0; 9];
    loop {
        let result = socket.read_exact(&mut buf).await;
        match result {
            Ok(n) => {
                if n != 9 {
                    eprintln!("Read unexpected number of bytes {n}: {buf:?}");
                    continue;
                }
            }

            Err(e) => {
                eprintln!("Failed to read from socket: {e}");
                continue;
            }
            _ => (),
        }

        let message = match buf.try_into() {
            Err(e) => {
                eprintln!("Encountered error deserializing buffer: {e}");
                return;
            }
            Ok(message) => message,
        };
    }
}
