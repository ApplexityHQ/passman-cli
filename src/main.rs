mod cli;
mod crypto;
mod models;
mod storage;
mod vault;

use anyhow::Result;

fn main() -> Result<()> {
    println!("passman-cli: secure CLI password manager");
    Ok(())
}
