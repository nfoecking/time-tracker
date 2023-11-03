use clap::{Parser, Subcommand};
use commands::{start, init, end};
use repositories::factory;

mod commands;
mod domain;
mod repositories;
mod helper;

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
    }
}


fn main() {
    let time_repo = factory::get_time_repository().unwrap();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            init::init_command(time_repo);
        },
        Commands::Start { timestamp } => {
            start::start_command(time_repo, timestamp);
        },
        Commands::End { timestamp, comment } => {
            end::end_command(time_repo, timestamp, comment);
        }
    }
}
