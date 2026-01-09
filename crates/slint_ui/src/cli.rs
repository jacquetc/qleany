use crate::app_context::AppContext;
use crate::cli_options;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::sync::Arc;

/// The main CLI struct that represents the command-line interface.
#[derive(Parser)]
#[command(author, version)]
#[command(about = "Scaffolding generator for C++/Qt6 and Rust projects", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Path to the Qleany manifest, defaults to current directory
    #[arg(default_value = ".")]
    qleany_folder: PathBuf,
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// The available commands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
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
    Check {},
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
    Show {},
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LanguageOption {
    Rust,
    CppQt,
}

/// The available subcommands for the `List` command.
#[derive(Debug, Subcommand)]
pub enum ListCommands {
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
    Common {},
}

/// The available subcommands for the `Generate` command.
#[derive(Debug, Subcommand)]
pub enum GenerateCommands {
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


/// Run the CLI with the given application context. Returns Some(()) if the application should
/// continue running as GUI, None otherwise.
pub fn run_cli(app_context: &Arc<AppContext>) -> Option<()> {
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
        None => {
            if verbose {
                println!("No command provided. Exiting.");
            }
            return Some(());
        }
        Some(command) => match command {
            Commands::New {
                folder_path,
                language,
            } => {
                if verbose {
                    println!(
                        "Creating new project in folder: {:?} with language: {:?}",
                        folder_path, language
                    );
                }
                cli_options::new::execute(&app_context, folder_path, language);
            }
            Commands::List {
                already_written,
                command,
            } => {
                if verbose {
                    println!(
                        "Listing with already_written: {:?}, command: {:?}",
                        already_written, command
                    );
                }
                // Implement the logic for listing
            }
            Commands::Check {} => {
                if verbose {
                    println!("Checking project...");
                }
                // Implement the logic for checking the project
            }
            Commands::Generate {
                dry_run,
                in_temp,
                file,
                command,
            } => {
                if verbose {
                    println!(
                        "Generating with dry_run: {:?}, in_temp: {:?}, file: {:?}, command: {:?}",
                        dry_run, in_temp, file, command
                    );
                }
                // Implement the logic for generating files
            }
            Commands::Show {} => {
                if verbose {
                    println!("Showing manifest...");
                }

                // Implement the logic for displaying the manifest
            }
        },
    }
    None
}
