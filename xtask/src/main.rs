use clap::{Parser, Subcommand};
use java::PolymathJavaCommand;

mod java;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(subcommand)]
    PolymathJava(PolymathJavaCommand),
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Command::PolymathJava(java_cmd) => java::exec(java_cmd),
    }
}
