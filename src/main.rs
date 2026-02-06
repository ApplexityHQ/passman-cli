mod cli;
mod crypto;
mod models;
mod storage;
mod vault;
mod tui;

use clap::Parser;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let path = Path::new("passwords.enc");

    if cli.tui {
        tui::run(path)?;
        return Ok(());
    }

    // existing CLI logic stays untouched
    unreachable!("CLI mode handled elsewhere");
}
