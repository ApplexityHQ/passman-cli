use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use argon2::Argon2;
use rand::RngCore;
use anyhow::Result;



const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_key(master_password: &str, salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];

    let argon2 = Argon2::default();

    argon2
        .hash_password_into(
            master_password.as_bytes(),
            salt,
            &mut key,
        )
        .expect("Key derivation failed");

    key
}


pub fn encrypt(
    plaintext: &[u8],
    key: &[u8; KEY_LEN],
) -> Result<(Vec<u8>, [u8; NONCE_LEN])> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow::anyhow!("Invalid key length"))?;

    let mut nonce = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .map_err(|_| anyhow::anyhow!("Encryption failed"))?;

    Ok((ciphertext, nonce))
}



pub fn decrypt(
    ciphertext: &[u8],
    key: &[u8; KEY_LEN],
    nonce: &[u8; NONCE_LEN],
) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow::anyhow!("Invalid key length"))?;

    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| anyhow::anyhow!("Invalid password or corrupted data"))?;

    Ok(plaintext)
}



