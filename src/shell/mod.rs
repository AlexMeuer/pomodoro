use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, Subcommand};

use crate::shell::commands::start::{StartTimerOptions, start_timer};

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Sets a custom state file
    #[arg(short, long, value_name = "FILE")]
    state: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        // TODO: parse human-readable durations?
        /// The length of the timer, in seconds.
        duration: u64,
    },
}

pub fn run() -> ExitCode {
    let cli = Cli::parse();

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    match cli.command {
        Commands::Start { duration } => {
            match start_timer(StartTimerOptions {
                duration_secs: duration,
                state: None,
            }) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("Error starting timer: {e}");
                    ExitCode::FAILURE
                }
            }
        }
    }
}
