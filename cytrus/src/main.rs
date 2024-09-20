// import the flatbuffers runtime library
extern crate flatbuffers;

// import the generated code
mod api;
#[allow(dead_code, unused_imports)]
#[path = "./manifiest_generated.rs"]
mod manifiest_generated;

use anyhow::{Ok, Result};
use api::Api;
use clap::{builder::PossibleValuesParser, Parser, Subcommand};
use manifiest_generated::Manifest;

const GAMES: [&str; 9] = [
    "dofus",
    "flyn",
    "krosfighter",
    "krosmaga",
    "onemoregate",
    "retro",
    "supernanoblaster",
    "wakfu",
    "waven",
];

const PLATFORMS: [&str; 3] = ["windows", "darwin", "linux"];

#[derive(Parser)]
#[command(name = "cytrus")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// download the game
    Download {
        /// game to download
        #[arg(short, long, value_parser(PossibleValuesParser::new(GAMES)))]
        game: String,
        /// platform of the game
        #[arg(short, long, value_parser(PossibleValuesParser::new(PLATFORMS)))]
        platform: String,
        /// get the beta version or not
        #[arg(short, long)]
        beta: bool,
    },
    /// get the latest version for a given game
    Version {
        /// game to download
        #[arg(short, long, value_parser(PossibleValuesParser::new(GAMES)))]
        game: String,
        /// platform of the game
        #[arg(short, long, value_parser(PossibleValuesParser::new(PLATFORMS)))]
        platform: String,
        /// get the beta version or not
        #[arg(short, long)]
        beta: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let result = match args.command.unwrap() {
        Commands::Version {
            game,
            platform,
            beta,
        } => Api::get_latest_version(&game, &platform, &beta).await?,
        Commands::Download {
            game,
            platform,
            beta,
        } => {
            let version = Api::get_latest_version(&game, &platform, &beta).await?;

            println!("Latest version: {version}");

            let manifest_binary = Api::get_manifiest(&game, &platform, &version, &beta).await?;

            let manifest = flatbuffers::root::<Manifest>(&manifest_binary)?;

            let fragments = manifest.fragments();

            for fragment in fragments.unwrap() {
                println!(
                    "{} - {} - {}",
                    fragment.name().unwrap(),
                    fragment.bundles().unwrap_or_default().len(),
                    fragment.files().unwrap().len()
                );
                for file in fragment.files().unwrap() {
                    println!(
                        "\t{} - {} - {}",
                        file.name().unwrap(),
                        file.size_(),
                        file.chunks().unwrap_or_default().len()
                    );
                }
            }

            String::new()
        }
    };

    println!("{result}");

    Ok(())
}
