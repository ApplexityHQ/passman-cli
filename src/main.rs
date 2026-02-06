mod crypto;
mod models;
mod vault;
mod storage;

use std::path::Path;

fn main() -> anyhow::Result<()> {
    let path = Path::new("passwords.enc");
    let master = "master123";

    let (mut vault, salt): (vault::Vault, [u8; 16]) = if path.exists() {
        storage::load_vault(path, master)?
    } else {
        storage::create_new_vault(master)?
    };

    vault.add_entry(models::PasswordEntry {
        service: "github".into(),
        username: "bhawesh".into(),
        password: "super-secret".into(),
    })?;

    storage::save_vault(path, &vault, master, &salt)?;

    println!("Vault saved securely.");

    Ok(())
}
