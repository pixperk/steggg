use anyhow::{Context, Result};
use crate::{crypto, steg, net};
use std::fs::File;
use std::io::{self, Read, BufWriter, Write};
use std::net::SocketAddr;
use tokio::net::TcpStream;

pub async fn run(addr: SocketAddr, cover_path: &str, password: &str, secret_path: &str) -> Result<()> {
    let mut plaintext = Vec::new();
    if secret_path == "-" {
        io::stdin().read_to_end(&mut plaintext)?;
    } else {
        let mut f = File::open(secret_path)?;
        f.read_to_end(&mut plaintext)?;
    }

    let payload = crypto::encrypt_message(password, &plaintext)?;

    let img = image::open(cover_path).context("open cover image failed")?;
    let stego = steg::embed_payload(img, &payload)?;
    let png_bytes = steg::encode_to_png(&stego)?;

    let mut out = BufWriter::new(File::create("stego.png")?);
    out.write_all(&png_bytes)?;
    out.flush()?;

    let mut stream = TcpStream::connect(addr).await?;
    net::send_bytes(&mut stream, &png_bytes).await?;
    Ok(())
}
