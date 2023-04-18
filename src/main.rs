mod miniflux_transfer;
mod szurubooru_upload;
mod miniflux;
mod szurubooru;

use clap::{Parser, Subcommand};
use miniflux::MinifluxContext;
use szurubooru::SzurubooruContext;

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
    MinifluxTransfer,
    Upload {
        #[arg(short, long)]
        source_directory: String,

        // Can use option multiple times, e.g. -t a -t b -t c -> ["a", "b", "c"]
        #[arg(short, long)]
        tags: Vec<String>,

        #[arg(short, long)]
        poll_name: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::MinifluxTransfer => {
            let secret = rpassword::prompt_password("Secret: ").unwrap();
            let mut split = secret.split(' ');
            // szurubooru_token is base64 of username:uuid-token, when convering remember not to append \n with echo -n. 
            let (miniflux_token, szurubooru_token) = (split.next().unwrap(), split.next().unwrap());
            let miniflux_context = MinifluxContext::new(miniflux_token.to_owned());
            let szurubooru_context = SzurubooruContext::new(szurubooru_token.to_owned());
            miniflux_transfer::perform(miniflux_context, szurubooru_context);
        },
        Commands::Upload{ source_directory, poll_name: name, tags } => {
            println!("{tags:?}");
            let token = rpassword::prompt_password("Token: ").unwrap();
            let szurubooru_context = SzurubooruContext::new(token.to_owned());
            szurubooru_upload::perform(szurubooru_context, source_directory, name, tags);
        }
    }
}

