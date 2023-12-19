use clap::Subcommand;
use color_eyre::Result;
use duct::cmd;

#[derive(Subcommand, Debug)]
pub enum PolymathJavaCommand {
    Version,
    SyncVersion,
}

pub fn exec(cmd: PolymathJavaCommand) -> Result<()> {
    match cmd {
        PolymathJavaCommand::Version => {
            println!("{}", get_java_version()?);
            Ok(())
        }
        PolymathJavaCommand::SyncVersion => sync_version(),
    }
}

fn get_java_version() -> Result<String> {
    Ok(cmd!(
        "mvn",
        "help:evaluate",
        "-Dexpression=project.version",
        "-q",
        "-DforceStdout"
    )
    .dir("polymath-java")
    .read()?)
}

fn sync_version() -> Result<()> {
    let crate_version = get_crate_version();
    set_version(&crate_version)?;
    Ok(())
}

fn set_version(version: &str) -> Result<String> {
    let current_version = get_java_version();
    cmd!("mvn", "versions:set", format!("-DnewVersion={version}"))
        .dir("polymath-java")
        .run()?;
    current_version
}

fn get_crate_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
