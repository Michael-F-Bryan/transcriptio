use anyhow::Error;
use clap::Parser;

fn main() -> Result<(), Error> {
    let Args {} = Args::parse();
    Ok(())
}

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {}
