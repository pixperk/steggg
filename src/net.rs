use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub async fn send_bytes(stream : &mut TcpStream, data : &[u8]) -> anyhow::Result<()>{
    let mut len_buf = [0u8; 8];
    (&mut len_buf[..]).write_u64::<BigEndian>(data.len() as u64)?;
    stream.write_all(&len_buf).await?;
    stream.write_all(data).await?;
    Ok(())
}

pub async fn receive_bytes(stream : &mut TcpStream) -> anyhow::Result<Vec<u8>>{
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let len = ReadBytesExt::read_u64::<BigEndian>(&mut &len_buf[..])? as usize;
    let mut data = vec![0u8; len];
    stream.read_exact(&mut data).await?;
    Ok(data)
}