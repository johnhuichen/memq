use clap::{Parser, Subcommand};
use colored::Colorize;
use log::error;
use snafu::Whatever;

use self::app_config::AppConfig;

mod app_config;
mod logger;

#[derive(Parser, Debug)]
#[command(
    name = "memq cli",
    version,
    about = "A program to query local markdown files"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand)]
    Path(PathCommands),
}

#[derive(Subcommand, Debug)]
enum PathCommands {
    /// Add a path to watchlist.
    /// Example: memq add /path/to/example.md /path/to/example
    Add {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// Remove a path from watchlist.
    /// Example: memq remove /path/to/example.md /path/to/example
    Remove {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// List paths in the watchlist.
    /// Example: memq list
    Show {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[snafu::report]
fn main() -> Result<(), Whatever> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Path(PathCommands::Add { paths, verbose }) => {
            logger::init(*verbose);

            match AppConfig::add_doc_paths(paths) {
                Ok(_) => println!("Done"),
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
        Commands::Path(PathCommands::Remove { paths, verbose }) => {
            logger::init(*verbose);

            match AppConfig::remove_doc_paths(paths) {
                Ok(_) => println!("Done"),
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
        Commands::Path(PathCommands::Show { verbose }) => {
            logger::init(*verbose);

            match AppConfig::show_doc_paths() {
                Ok(_) => {}
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
    };

    Ok(())
}
