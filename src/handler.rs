use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
};
use std::net::SocketAddr;
use log::{debug, info};
use crate::shared::Message;


pub async fn handler(mut stream: TcpStream, addr: SocketAddr) {
    info!("Accepted new connection from address: {}", addr);
    let mut buf = vec![];
    stream.read(&mut buf).await.ok();
    let message = Message::from_bytes(buf);
    if message.is_none() {
        return;
    }
    debug!("Got new message: {:?}", message);
}