use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
};
use sha2::{Digest, Sha256};

pub const NONCE_SIZE: usize = 24;

pub fn derive_key_from_password(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::default();
    hasher.update(password.as_bytes());
    let res = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&res);
    key
}

pub fn encrypt_message(password: &str, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key_bytes = derive_key_from_password(password);
    let aead = XChaCha20Poly1305::new(&key_bytes.into());

    let mut nonce = [0u8; NONCE_SIZE];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut nonce);

    let ciphertext = aead
        .encrypt(&nonce.into(), plaintext)
        .map_err(|e| anyhow::anyhow!("encryption failed: {:?}", e))?;

    let mut payload = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    payload.extend_from_slice(&nonce);
    payload.extend_from_slice(&ciphertext);
    Ok(payload)
}


pub fn decrypt_message(password: &str, payload: &[u8]) -> anyhow::Result<Vec<u8>> {
    if payload.len() < NONCE_SIZE {
        return Err(anyhow::anyhow!("payload too short"));
    }

    let (nonce_bytes, ciphertext) = payload.split_at(NONCE_SIZE);
    let key_bytes = derive_key_from_password(password);
    let aead = XChaCha20Poly1305::new(&key_bytes.into());

    let plaintext = aead
        .decrypt(nonce_bytes.into(), ciphertext)
        .map_err(|e| anyhow::anyhow!("decryption failed: {:?}", e))?;
    Ok(plaintext)
}