use std::{
    io::Stdout,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use clap::Subcommand;
use color_eyre::Result;

#[derive(Subcommand, Debug)]
pub enum JavaCommand {
    Version,
    Release,
}

pub fn exec(cmd: JavaCommand) -> Result<()> {
    match cmd {
        JavaCommand::Version => {
            println!("{}", get_java_version()?);
            Ok(())
        }
        JavaCommand::Release => release(),
    }
}

fn get_java_version() -> Result<String> {
    let out = Command::new("mvn")
        .current_dir("polymath-java")
        .args([
            "help:evaluate",
            "-Dexpression=project.version",
            "-q",
            "-DforceStdout",
        ])
        .output()?;

    Ok(String::from_utf8(out.stdout)?)
}

fn release() -> Result<()> {
    todo!()
}
