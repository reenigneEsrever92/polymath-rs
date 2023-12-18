use clap::{Parser, Subcommand};
use java::JavaCommand;

mod java;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(subcommand)]
    Java(JavaCommand),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Command::Java(java_cmd) => java::exec(java_cmd),
    }
}
