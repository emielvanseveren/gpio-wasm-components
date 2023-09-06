use clap::{error::ErrorKind, Parser};
use commands::{RunCommand, VersionCommand};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

mod bindings;
mod commands;

#[derive(Parser)]
#[clap(
    version,
    after_help = "If no subcommand is provided, the `run` subcommand will be used."
)]

enum Command {
    Version(VersionCommand),
    Run(RunCommand),
}

impl Command {
    pub fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Version(c) => c.execute(),
            Self::Run(mut c) => c.execute(),
        }
    }
}

fn setup_global_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::ERROR)
        .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
        .finish();

    // Set the default subscriber for the application
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set default subscriber");
}

// my cpu has 4 cores with 2 threads each.
fn main() -> wasmtime::Result<()> {
    setup_global_subscriber();

    Command::try_parse()
        .unwrap_or_else(|e: clap::Error| match e.kind() {
            ErrorKind::InvalidSubcommand | ErrorKind::UnknownArgument => {
                Command::Run(RunCommand::parse())
            }
            _ => e.exit(),
        })
        .execute()
}
