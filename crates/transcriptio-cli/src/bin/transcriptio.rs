use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Error};
use clap::Parser;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(tracing::level_filters::LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let Args { cmd } = Args::parse();

    match cmd {
        Cmd::Parse(p) => p.execute(),
    }
}

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Clone, Parser)]
enum Cmd {
    /// Parse a subtitle file.
    Parse(Parse),
}

#[derive(Debug, Clone, Parser)]
struct Parse {
    /// The file to parse
    input: Input,
}

impl Parse {
    fn execute(self) -> Result<(), Error> {
        let mut input = self.input.open()?;
        let mut src = String::new();
        input.read_to_string(&mut src)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Input {
    Stdin,
    File(PathBuf),
}

impl Input {
    fn open(&self) -> Result<Box<dyn BufRead>, Error> {
        match self {
            Input::Stdin => Ok(Box::new(BufReader::new(std::io::stdin()))),
            Input::File(path) => {
                let f = File::open(path).with_context(|| {
                    format!("Unable to open \"{}\" for reading", path.display())
                })?;
                Ok(Box::new(BufReader::new(f)))
            }
        }
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        if value == "-" {
            return Input::Stdin;
        }

        Input::File(value.into())
    }
}
