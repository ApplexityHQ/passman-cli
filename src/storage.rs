use std::fs;
use std::path::Path;

use anyhow::{Result, bail};

use crate::crypto;
use crate::vault::Vault;
use crate::models::PasswordEntry;


const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;


pub fn save_vault(
    path: &Path,
    vault: &Vault,
    master_password: &str,
    salt: &[u8; SALT_LEN],
) -> Result<()> {

    // 1. Serialize vault entries → JSON
    let plaintext = serde_json::to_vec(&vault.entries)
        .map_err(|_| anyhow::anyhow!("Failed to serialize vault"))?;

    // 2. Derive key
    let key = crypto::derive_key(master_password, salt);

    // 3. Encrypt
    let (ciphertext, nonce) = crypto::encrypt(&plaintext, &key)?;

    // 4. Build binary blob
    let mut file_data = Vec::new();
    file_data.extend_from_slice(salt);
    file_data.extend_from_slice(&nonce);
    file_data.extend_from_slice(&ciphertext);

    // 5. Atomic write
    fs::write(path, file_data)?;

    Ok(())
}

pub fn load_vault(
    path: &Path,
    master_password: &str,
) -> Result<(Vault, [u8; SALT_LEN])> {
    if !path.exists() {
        bail!("Vault file does not exist");
    }

    let data = fs::read(path)?;

    if data.len() < SALT_LEN + NONCE_LEN {
        bail!("Vault file corrupted");
    }

    // 1. Split file
    let salt: [u8; SALT_LEN] = data[..SALT_LEN].try_into().unwrap();
    let nonce: [u8; NONCE_LEN] = data[SALT_LEN..SALT_LEN + NONCE_LEN]
        .try_into()
        .unwrap();
    let ciphertext = &data[SALT_LEN + NONCE_LEN..];

    // 2. Derive key
    let key = crypto::derive_key(master_password, &salt);

    // 3. Decrypt
    let plaintext = crypto::decrypt(ciphertext, &key, &nonce)?;

    // 4. Deserialize
    let entries: Vec<PasswordEntry> =
        serde_json::from_slice(&plaintext)
            .map_err(|_| anyhow::anyhow!("Failed to parse vault"))?;

    Ok((Vault { entries }, salt))
}


pub fn create_new_vault(
    _master_password: &str,
) -> Result<(Vault, [u8; SALT_LEN])> {
    let salt = crypto::generate_salt();
    let vault = Vault::new();
    Ok((vault, salt))
}



/*
    filesystem only
    file I/O
*/
