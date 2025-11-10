use clap::{Parser, Subcommand};
use std::process;

use crate::{finish::finish, info::info, init::init, project::load_project, start::start};

mod finish;
mod info;
mod init;
mod metadata;
mod project;
mod start;

#[derive(Parser)]
#[command(name = "clok")]
#[command(about = "A simple time tracking tool for your Rust projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init,
    /// Start a new session
    Start,
    /// Stop the current session
    Stop,
    /// Show total time spent on project
    Info,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_command(cli) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn run_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Init => {
            init()?;
            let project = load_project()?;
            println!(
                "clok is successfuly initilized for project '{}'",
                project.title()
            )
        }
        Commands::Start => {
            let mut project = load_project()?;
            start(&mut project)?;
            println!("new session for '{}' has started!", project.title());
        }
        Commands::Stop => {
            let mut project = load_project()?;
            finish(&mut project)?;
            println!("last session for '{}' has finished!", project.title())
        }
        Commands::Info => {
            let project = load_project()?;
            let info = info(&project)?;
            println!("{}", info);
        }
    }
    Ok(())
}
