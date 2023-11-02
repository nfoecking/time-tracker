use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "time-tracker")]
#[command(about = "CLI application to track working times", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Start {
        #[arg(long, short)]
        timestamp: String,
    },
    End {
        #[arg(long, short)]
        timestamp: String,
    }
}


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Start { timestamp } => {
            println!("'myapp start' was used, ts is: {timestamp:?}")
        },
        Commands::End { timestamp } => {
            println!("'myapp start' was used, ts is: {timestamp:?}")
        }
    }
}
