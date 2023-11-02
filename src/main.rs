use clap::{Parser, Subcommand};
use commands::{start, init};
use repositories::factory;

mod commands;
mod domain;
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
    }
}


fn main() {
    let time_repo = factory::get_time_repository().unwrap();
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Init => {
            init::init_command(time_repo);
        },
        Commands::Start { timestamp } => {
            start::start_command(timestamp);
        },
        Commands::End { timestamp } => {
            println!("'myapp start' was used, ts is: {timestamp:?}")
        }
    }
}
