use clap::{Parser, Subcommand};
use colored::Colorize;
use log::error;
use snafu::{ResultExt, Whatever};

use self::app_config::AppConfig;

mod logger;
mod app_config;

#[derive(Parser, Debug)]
#[command(name = "memq cli", version, about = "A program to query local markdown files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

            match AppConfig::add_paths(paths) {
                Ok(_) => println!("done"),
                Err(e) => match e {
                    app_config::AppConfigError::InvalidPath { path } => error!("Error: invalid path={}", path),
                    app_config::AppConfigError::LoadConfig { source: _ }  => error!("Error: cannot load config"),
                    app_config::AppConfigError::SaveConfig { source: _ } => error!("Error: cannot save config"),
                },
            }
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
