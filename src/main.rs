use clap::{Parser, Subcommand};
use commands::{end, init, start, day};
use repositories::factory;

mod commands;
mod domain;
mod helper;
mod repositories;

#[derive(Debug, Parser)]
#[command(name = "time-tracker")]
#[command(about = "CLI application to track working times", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    Start {
        #[arg(long, short)]
        timestamp: Option<String>,
    },
    End {
        #[arg(long, short)]
        timestamp: Option<String>,
        #[arg(long, short)]
        comment: Option<String>,
    },
    Day {
        #[arg(long, short)]
        timestamp: Option<String>,
    },
}

fn main() {
    let time_repo = factory::get_time_repository().unwrap();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init::init_command(time_repo);
        }
        Commands::Start { timestamp } => {
            start::start_command(time_repo, timestamp);
        }
        Commands::End { timestamp, comment } => {
            end::end_command(time_repo, timestamp, comment);
        }
        Commands::Day { timestamp } => {
            day::day_command(time_repo, timestamp)
        }
    }
}
