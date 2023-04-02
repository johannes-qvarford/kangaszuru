mod miniflux_transfer;

use std::error::Error;

use clap::{Parser, Subcommand};

/// Simple cli to perform various szurubooru-related tasks.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    MinifluxTransfer
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::MinifluxTransfer => {
            let secret = rpassword::prompt_password("Secret: ").unwrap();
            let mut split = secret.split(' ');
            // szurubooru_token is base64 of username:uuid-token, when convering remember not to append \n with echo -n. 
            let (miniflux_token, szurubooru_token) = (split.next().unwrap(), split.next().unwrap());
            miniflux_transfer::perform(miniflux_token, szurubooru_token);
        }
    }
}

