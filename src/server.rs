use anyhow::Result;
use crate::{crypto, steg, net};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut socket: TcpStream, password: String) -> Result<()> {
    let png_bytes = net::receive_bytes(&mut socket).await?;
    let img = steg::decode_from_bytes(&png_bytes)?;
    let payload = steg::extract_payload(img)?;
    let plaintext = crypto::decrypt_message(&password, &payload)?;
    println!("received: {}", String::from_utf8_lossy(&plaintext));
    Ok(())
}

pub async fn run(addr: &str, password: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {}", addr);
    loop {
        let (socket, peer) = listener.accept().await?;
        let pass = password.to_string();
        println!("accepted from {}", peer);
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, pass).await {
                eprintln!("error: {:?}", e);
            }
        });
    }
}
