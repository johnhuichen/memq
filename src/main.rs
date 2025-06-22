use clap::{Parser, Subcommand};
use colored::Colorize;
use log::error;
use snafu::{ResultExt, Whatever};

use self::app_config::AppConfig;
use self::models::fuzzy_query::FuzzyQuery;
use self::traits::query::Query;

mod app_config;
mod logger;
mod models;
mod traits;

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

    #[command(subcommand)]
    Query(QueryCommands),
}

#[derive(Subcommand, Debug)]
enum PathCommands {
    /// Add a path to watchlist.
    /// Example: memq path add /path/to/file /path/to/dir
    Add {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// Remove a path from watchlist.
    /// Example: memq path remove /path/to/example.md /path/to/example
    Remove {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// List paths in the watchlist.
    /// Example: memq path list
    List {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand, Debug)]
enum QueryCommands {
    /// Query docs with a keyword
    /// Example: memq query keyword foobar
    Keyword {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// The keyword used to query md docs
        #[arg(required = true)]
        keyword: String,
    },

    /// Sync docs with a query strategy
    /// Example: memq query sync
    Sync {
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
            let mut config = AppConfig::load().whatever_context("Failed to load config")?;

            match config.add_docs(paths) {
                Ok(_) => println!("Done"),
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
        Commands::Path(PathCommands::Remove { paths, verbose }) => {
            logger::init(*verbose);
            let mut config = AppConfig::load().whatever_context("Failed to load config")?;

            match config.remove_doc_paths(paths) {
                Ok(_) => println!("Done"),
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
        Commands::Path(PathCommands::List { verbose }) => {
            logger::init(*verbose);
            let config = AppConfig::load().whatever_context("Failed to load config")?;

            match config.print_docs() {
                Ok(_) => {}
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
        Commands::Query(QueryCommands::Keyword { verbose, keyword }) => {
            logger::init(*verbose);

            match FuzzyQuery::query(keyword) {
                Ok(result) => println!("{}", result),
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
        Commands::Query(QueryCommands::Sync { verbose }) => {
            logger::init(*verbose);
            let config = AppConfig::load().whatever_context("Failed to load config")?;

            match FuzzyQuery::sync(&config) {
                Ok(_) => {},
                Err(e) => error!("{}", format!("{e}").bright_red()),
            }
        }
    };

    Ok(())
}
