use clap::{arg, command, Parser};
use log::info;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::io::Result;
use tokio::net::TcpListener;

mod shared;
mod handler;
use handler::handler;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    host: Ipv4Addr,
    #[arg()]
    port: u16,
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init(); // Set up log to print to stdout
    let args = Args::parse();
    // Translate arguments to ipv4 address
    let addr = SocketAddr::from((args.host, args.port));
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on address {}", &addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        handler(stream, addr).await;
    }
}
