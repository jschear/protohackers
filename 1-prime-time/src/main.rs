mod line_bytes_codec;

use anyhow::Result;
use futures::prelude::*;
use line_bytes_codec::LineBytesCodec;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Number;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_serde::formats::Json;

#[derive(Deserialize, Debug)]
struct Request {
    method: String,
    number: Number,
}

#[derive(Serialize, Debug)]
struct Response {
    method: String,
    prime: bool,
}

type LinesFramed = tokio_util::codec::Framed<TcpStream, LineBytesCodec>;
type PrimeProtocolFramed =
    tokio_serde::Framed<LinesFramed, Request, Response, Json<Request, Response>>;

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

async fn process(socket: TcpStream) {
    let lines_framed = LinesFramed::new(socket, LineBytesCodec::new());
    let mut serde_framed =
        PrimeProtocolFramed::new(lines_framed, Json::<Request, Response>::default());

    loop {
        let request = match serde_framed.try_next().await {
            Ok(Some(request)) => request,
            Ok(None) => {
                return;
            }
            Err(e) => {
                eprintln!("request processing failed; err = {:?}", e);
                reply_error(&mut serde_framed).await;
                return;
            }
        };

        match request.method.as_str() {
            "isPrime" => {
                reply(
                    &mut serde_framed,
                    Response {
                        method: "isPrime".to_string(),
                        prime: is_prime(request.number),
                    },
                )
                .await
            }
            _ => {
                reply_error(&mut serde_framed).await;
                return;
            }
        };
    }
}

async fn reply_error(serde_framed: &mut PrimeProtocolFramed) {
    reply(
        serde_framed,
        Response {
            method: "Error".to_string(),
            prime: false,
        },
    )
    .await;
}

async fn reply(serde_framed: &mut PrimeProtocolFramed, response: Response) {
    serde_framed
        .send(response)
        .await
        .unwrap_or_else(|e| eprintln!("failed to send response; err = {:?}", e));
}

fn is_prime(number: Number) -> bool {
    match number.as_u64() {
        Some(number) => primes::is_prime(number),
        None => false,
    }
}
