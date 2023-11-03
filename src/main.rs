use clap::{Parser, Subcommand};
use commands::{end, init, start, day, month, year};
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
    /// Initialize the underlying database
    Init,
    /// Start a new tracking session
    Start {
        /// Timestamp in the format 2023-01-02T07:09 which is used as the start-ts of the tracking session. 
        /// Defaults to the current timestamp.
        #[arg(long, short, verbatim_doc_comment)]
        timestamp: Option<String>,
    },
    /// Stop the active tracking session
    End {
        /// Timestamp in the format 2023-01-02T07:09 which is used as the end-ts of the tracking session.
        /// Defaults to the current timestamp.
        #[arg(long, short, verbatim_doc_comment)]
        timestamp: Option<String>,
        /// Free-text comment for the current tracking session.
        #[arg(long, short)]
        comment: Option<String>,
    },
    /// Summary of the daily tracking sessions
    Day {
        /// Timestamp in the format 2023-01-02T07:09.
        /// The day-component of the timestamp is used to filter the tracking sessions.
        /// Defaults to the current timestamp.
        #[arg(long, short, verbatim_doc_comment)]
        timestamp: Option<String>,
    },
    /// Summary of the monthly tracking sessions
    Month {
        /// Timestamp in the format 2023-01-02T07:09.
        /// The month-component of the timestamp is used to filter the tracking sessions.
        /// Defaults to the current timestamp.
        #[arg(long, short, verbatim_doc_comment)]
        timestamp: Option<String>,
    },
    /// Summary of the yearly tracking sessions
    Year {
        /// Timestamp in the format 2023-01-02T07:09.
        /// The year-component of the timestamp is used to filter the tracking sessions.
        /// Defaults to the current timestamp.
        #[arg(long, short, verbatim_doc_comment)]
        timestamp: Option<String>,
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
        },
        Commands::Day { timestamp } => {
            day::day_command(time_repo, timestamp)
        },
        Commands::Month { timestamp } => {
            month::month_command(time_repo, timestamp)
        },
        Commands::Year { timestamp } => {
            year::year_command(time_repo, timestamp)
        }
    }
}
