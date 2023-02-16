use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Number;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_serde::formats::Json;
use tokio_util::codec::LinesCodec;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    method: String,
    number: Number,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    method: String,
    prime: bool,
}

type JsonCodec = Json<Request, Response>;

type LinesFramed = tokio_util::codec::Framed<TcpStream, LinesCodec>;
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

async fn process(mut socket: TcpStream) {
    let lines_framed = LinesFramed::new(socket, LinesCodec::new());
    let serde_framed = PrimeProtocolFramed::new(lines_framed, Json::<Request, Response>::default());

    // TODO: why can't the compiler find this method?
    serde_framed.next();

    // while let Some(msg) = serde_framed.try_next().await.unwrap() {
    //     println!("GOT: {:?}", msg);
    // }
}
