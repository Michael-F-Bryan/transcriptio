mod dist;

use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use clap::Parser;

fn main() -> Result<(), Error> {
    let Args { cmd } = Args::parse();

    match cmd {
        Command::Dist(d) => d.execute(),
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Generate release artifacts.
    Dist(dist::Dist),
}

pub(crate) fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}

pub(crate) fn copy(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), Error> {
    let src = src.as_ref();
    let dest = dest.as_ref();

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Unable to create \"{}\"", parent.display()))?;
    }

    std::fs::copy(src, dest).with_context(|| {
        format!(
            "Unable to copy \"{}\" to \"{}\"",
            src.display(),
            dest.display(),
        )
    })?;

    Ok(())
}
