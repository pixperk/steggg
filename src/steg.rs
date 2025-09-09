use anyhow::{anyhow, Result};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use image::{DynamicImage, ImageEncoder, RgbaImage};
use std::io::Cursor;
pub fn embed_payload(image : DynamicImage, payload : &[u8]) -> anyhow::Result<RgbaImage>{
    let mut rgba = image.to_rgba8();
    let capacity_bits = rgba.len();
    let needed_bits = 32 + payload.len() * 8; // 32 bits for length + payload bits
    if needed_bits > capacity_bits {
        return Err(anyhow::anyhow!("Image too small to hold payload"));
    }
    let mut all_bytes = Vec::with_capacity(4 + payload.len());
    all_bytes.write_u32::<BigEndian>(payload.len() as u32)?;
    all_bytes.extend_from_slice(payload);

    let mut bit_iter = all_bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1))
        .peekable();

     for byte in rgba.iter_mut() {
        if bit_iter.peek().is_none() {
            break;
        }
        let next_bit = bit_iter.next().unwrap();
        *byte = (*byte & 0xFE) | next_bit;
    }

    Ok(rgba)
}

pub fn extract_payload(img: DynamicImage) -> anyhow::Result<Vec<u8>> {
    let rgba = img.to_rgba8();
    let mut bits = rgba.iter().map(|b| (b & 1) );

    let mut len: u32 = 0;
    for _ in 0..32 {
        let bit = bits.next().ok_or_else(|| anyhow!("no bits for length"))?;
        len = (len << 1) | (bit as u32);
    }

    let mut payload = vec![0u8; len as usize];
    for byte_idx in 0..(len as usize) {
        let mut b = 0u8;
        for _ in 0..8 {
            let bit = bits.next().ok_or_else(|| anyhow!("not enough bits"))?;
            b = (b << 1) | bit;
        }
        payload[byte_idx] = b;
    }
    Ok(payload)
}

pub fn encode_to_png(img: &RgbaImage) -> anyhow::Result<Vec<u8>> {
    let mut png_bytes = Vec::new();
    {
        let mut encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        encoder.write_image(
            img.as_raw(),
            img.width(),
            img.height(),
            image::ColorType::Rgba8.into(),
        )?;
    }
    Ok(png_bytes)
}

pub fn decode_from_bytes(png_bytes: &[u8]) -> anyhow::Result<DynamicImage> {
    let reader = image::io::Reader::new(Cursor::new(png_bytes))
        .with_guessed_format()?;
    let img = reader.decode()?;
    Ok(img)
}