use clap::{Parser, Subcommand};
use colored::Colorize;
use log::info;
use snafu::Whatever;

mod logger;
mod config;

#[derive(Parser, Debug)]
#[command(name = "mycli", version, about = "A CLI with subcommands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a path to watchlist
    /// Example: memq add /path/to/example.md /path/to/example
    Add {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// Remove a path from watchlist
    /// Example: memq remove /path/to/example.md /path/to/example
    Remove {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// List paths in the watchlist
    /// Example: memq list
    List {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { paths, verbose } => {
            logger::init(*verbose);

            info!("This is a log message");
            println!("{}", format!("Add paths: {paths:?}").bright_blue());
        }
        Commands::Remove { paths, verbose } => {
            logger::init(*verbose);

            println!("Remove paths: {:?}", paths);
        }
        Commands::List { verbose } => {
            logger::init(*verbose);

            println!("List paths");
        }
    };

    Ok(())
}
