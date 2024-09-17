// import the flatbuffers runtime library
extern crate flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./manifiest_generated.rs"]
mod manifiest_generated;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// download the game
    Download {
        /// lists test values
        #[arg(short, long)]
        game: String,
    },
    /// get the latest version for a given game
    Version {
        #[arg(short, long)]
        game: String,
    },
}

fn main() {
    let args = Cli::parse();
    println!("Hello, world!");
}
