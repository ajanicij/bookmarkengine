use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "write")]
struct Args {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Write {
        #[arg(long, short)]
        index: PathBuf,

        #[arg(long, short)]
        bookmark: PathBuf,

        #[arg(long, short, default_value = "")]
        max_age: String,
    },

    Search {
        #[arg(long, short)]
        index: PathBuf,

        #[arg(long, short)]
        query: String,
    }
}

fn main() {
    let args = Args::parse();
    println!("command: {:?}", args.command);
}
