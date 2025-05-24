use clap::{Parser, Subcommand};

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
        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// Remove a path from watchlist
    /// Example: memq remove /path/to/example.md /path/to/example
    Remove {
        /// The path will be query for markdown files. The path can be for a file or folder.
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// List paths in the watchlist
    /// Example: memq list
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { paths } => {
            println!("Add paths: {:?}", paths);
        }
        Commands::Remove { paths } => {
            println!("Remove paths: {:?}", paths);
        }
        Commands::List {} => {
            println!("List paths");
        }
    }
}
