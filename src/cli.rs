use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "passman")]
#[command(about = "Secure CLI password manager written in Rust")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        service: String,
        username: String,
    }, 
    Get {
        service: String,
    },
    List,
    Delete {
        service: String,
    },
}




/*
    UX layering
    CLI Parsing
*/
