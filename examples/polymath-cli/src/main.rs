use std::{env::temp_dir, fs::File, io::Write};

use clap::{Parser, Subcommand};
use rand::Rng;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    ToMathMl {
        asciimath: String,
        #[arg(short, long)]
        open: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::ToMathMl { asciimath, open } => {
            let mathml = format!("<math>{}</math>", polymath_rs::to_math_ml(&asciimath));

            if open {
                let mut dir = temp_dir();
                let file_name = format!("{}.html", rand::thread_rng().gen::<i64>());

                dir.push(file_name);

                let mut file = File::create(dir.clone()).unwrap();

                file.write_all(mathml.as_bytes()).unwrap();

                open::that(format!(
                    "file://{}",
                    dir.canonicalize().unwrap().as_path().display()
                ))
                .unwrap();
            } else {
                println!("{mathml}");
            }
        }
    }
}
