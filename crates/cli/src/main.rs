
use anyhow::Result;
use clap::Arg;
use clap::{Parser, Subcommand};

/// The main CLI struct that represents the command-line interface.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// The available commands for the CLI.
#[derive(Subcommand)]
enum Commands {
    /// Commands related to projects.
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
}

/// The available subcommands for the `Project` command.
#[derive(Subcommand)]
enum ProjectCommands {
    /// Creates a new project.
    Create { dpy_file: String },
    // Kill { project_id: Option<u32> },
    // List {},
    // Start { project_id: Option<u32> },
    // Stop { project_id: Option<u32> },
    // Pause { project_id: Option<u32> },
    // Resume { project_id: Option<u32> },
    // Restart { project_id: Option<u32> },
    // Status { project_id: Option<u32> },
}


fn main() { 
       let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
   // match &cli.command {
}