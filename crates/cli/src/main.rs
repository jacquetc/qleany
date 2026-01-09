use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;
use clap::Arg;
use clap::{Parser, Subcommand, ValueEnum};
use common::direct_access::root::RootRelationshipField;
use direct_access::{global_controller, root_controller};
use handling_manifest::handling_manifest_controller;
use crate::app_context::AppContext;

mod app_context;
mod options;

/// The main CLI struct that represents the command-line interface.
#[derive(Parser)]
#[command(author, version)]
#[command(about = "Scaffolding generator for C++/Qt6 and Rust projects", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to the Qleany manifest, defaults to current directory
    #[arg(default_value = ".")]
    qleany_folder: PathBuf,
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

/// The available commands for the CLI.
#[derive(Subcommand)]
enum Commands {
    ///

    /// Commands related to New.
    New {
        folder_path: Option<String>,
        #[arg(short, long, value_enum)]
        language: Option<LanguageOption>,
    },
    List {
        #[arg(short, long)]
        already_written: bool,
        #[command(subcommand)]
        command: Option<ListCommands>,
    },
    Check {
    },
    Generate {
        /// Display the files that would be generated without actually creating them
        #[arg(short, long)]
        dry_run: bool,
        /// Generate files in a temporary folder named "temp"
        #[arg(short, long)]
        in_temp: bool,
        /// Select the file to be generated
        #[arg(short, long)]
        file: Option<PathBuf>,

        #[command(subcommand)]
        command: Option<GenerateCommands>,

    },
    /// Display the manifest
    Show {
    },
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LanguageOption {
    Rust,
    CppQt,
}

/// The available subcommands for the `List` command.
#[derive(Debug, Subcommand)]
enum ListCommands {
    All,
    Features {
        /// Select the feature to be listed
        #[arg(short, long)]
        name: Option<String>,
    },
    Entities {
        /// Select the feature to which the entities belong
        #[arg(short, long)]
        name: Option<String>,
    },
    Common {
    },
}

/// The available subcommands for the `Generate` command.
#[derive(Debug, Subcommand)]
enum GenerateCommands {
    All,
    Feature {
        /// Select the feature to be generated
        #[arg(short, long)]
        name: Option<String>,
    },
    Entities {
        /// Select the feature to which the entities belong
        #[arg(short, long)]
        name: Option<String>,
    },
    File {
        /// Select the file to be generated
        file: PathBuf,
    },
}

fn main() {
    // Initialize logging
    env_logger::init();

    // Create the application context (backend state)
    let app_context = Arc::new(AppContext::new());

    let cli = Cli::parse();

    let verbose = cli.verbose;

    let qleany_path = match &cli.qleany_folder.exists() {
        true => cli.qleany_folder.clone(),
        false => {
            eprintln!("The specified path does not exist: {:?}", cli.qleany_folder);

            std::process::exit(1);
        }
    };
    if verbose {
        println!("Qleany folder path: {:?}", qleany_path);
    }

    match &cli.command {
        Commands::New { folder_path, language } => {
            if verbose {
                println!("Creating new project in folder: {:?} with language: {:?}", folder_path, language);
            }
            options::new::execute(&app_context, folder_path, language);

        }
        Commands::List { already_written, command } => {
            if verbose {
                println!("Listing with already_written: {:?}, command: {:?}", already_written, command);
            }
            // Implement the logic for listing
        }
        Commands::Check {} => {
            if verbose {
                println!("Checking project...");
            }
            // Implement the logic for checking the project
        }
        Commands::Generate { dry_run, in_temp, file, command } => {
            if verbose {
                println!("Generating with dry_run: {:?}, in_temp: {:?}, file: {:?}, command: {:?}", dry_run, in_temp, file, command);
            }
            // Implement the logic for generating files
        }
        Commands::Show {} => {
            if verbose {
                println!("Showing manifest...");
            }

            // Implement the logic for displaying the manifest
        }
    }
}
