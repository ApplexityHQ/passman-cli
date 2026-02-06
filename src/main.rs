mod cli;
mod crypto;
mod models;
mod storage;
mod vault;

use clap::Parser;
use std::path::Path;

use cli::Cli;
use models::PasswordEntry;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let path = Path::new("passwords.enc");

    // Prompt master password
    let master_password =
        rpassword::prompt_password("Master password: ")?;

    // Load or create vault
    let (mut vault, salt): (vault::Vault, [u8; 16]) = if path.exists() {
        storage::load_vault(path, &master_password)?
    } else {
        println!("No vault found. Creating new vault...");
        storage::create_new_vault(&master_password)?
    };

    match cli.command {
        // ADD
        cli::Commands::Add { service, username } => {
            let password =
                rpassword::prompt_password("Service password: ")?;

            vault.add_entry(PasswordEntry {
                service,
                username,
                password,
            })?;

            storage::save_vault(path, &vault, &master_password, &salt)?;
            println!("Entry added");
        }

        // GET
        cli::Commands::Get { service } => {
            match vault.get_entry(&service) {
                Some(entry) => {
                    println!("Service  : {}", entry.service);
                    println!("Username : {}", entry.username);
                    println!("Password : {}", entry.password);
                }
                None => println!("Service not found"),
            }
        }

        // LIST
        cli::Commands::List => {
            let services = vault.list_services();
            if services.is_empty() {
                println!("Vault is empty");
            } else {
                for s in services {
                    println!("{}", s);
                }
            }
        }

        // DELETE
        cli::Commands::Delete { service } => {
            vault.delete_entry(&service)?;
            storage::save_vault(path, &vault, &master_password, &salt)?;
            println!("Entry deleted");
        }
    }

    Ok(())
}
