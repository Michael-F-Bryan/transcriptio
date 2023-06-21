use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Error};
use once_cell::sync::Lazy;
use zip::{write::FileOptions, ZipWriter};

const BINARIES: &[&str] = &["transcriptio"];
const STATIC_FILES: &[&str] = &["README.md", "LICENSE_MIT.md", "LICENSE_APACHE.md"];

#[derive(clap::Parser, Debug)]
pub struct Dist {
    /// The drectory to save release artifacts to.
    #[clap(long, default_value = DIST_DIR.as_os_str())]
    output_dir: PathBuf,
}

impl Dist {
    pub(crate) fn execute(self) -> Result<(), Error> {
        let _ = std::fs::remove_dir_all(&self.output_dir);
        std::fs::create_dir_all(&self.output_dir)
            .with_context(|| format!("Unable to create \"{}\"", self.output_dir.display()))?;

        self.binary()?;
        self.files()?;
        self.bundle()?;

        Ok(())
    }

    fn bundle(&self) -> Result<(), Error> {
        let parent_dir = self.output_dir.parent().unwrap();
        let triple = host_triple()?;
        let dest = parent_dir.join(format!("transcriptio.{triple}.zip"));

        let f = File::create(&dest)
            .with_context(|| format!("Unable to create \"{}\"", dest.display()))?;

        let mut writer = ZipWriter::new(BufWriter::new(f));
        self.bundle_dir(&self.output_dir, &mut writer)?;
        writer.finish()?.flush()?;

        Ok(())
    }

    fn bundle_dir(&self, dir: &Path, writer: &mut ZipWriter<BufWriter<File>>) -> Result<(), Error> {
        for entry in dir.read_dir()? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();
            let stripped = path
                .strip_prefix(&self.output_dir)
                .unwrap()
                .display()
                .to_string()
                .replace('\\', "/");

            if file_type.is_file() {
                writer.start_file(stripped, FileOptions::default())?;
                let f = File::open(&path)?;
                let mut reader = BufReader::new(f);
                std::io::copy(&mut reader, writer)?;
            } else if file_type.is_dir() {
                writer.add_directory(stripped, FileOptions::default())?;
                self.bundle_dir(&path, writer)?;
            }
        }

        Ok(())
    }

    fn binary(&self) -> Result<(), Error> {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let status = Command::new(cargo)
            .current_dir(crate::project_root())
            .args(["build", "--release", "--workspace", "--locked"])
            .status()?;

        anyhow::ensure!(status.success(), "Cargo build failed");

        let target_dir = crate::project_root().join("target").join("release");

        let can_strip = Command::new("strip")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok();

        for binary in BINARIES {
            let mut src = target_dir.join(binary);
            if cfg!(windows) {
                src.set_extension("exe");
            }

            let mut dest = self.output_dir.join(binary);
            if cfg!(windows) {
                dest.set_extension("exe");
            }

            crate::copy(&src, &dest)?;

            if can_strip {
                let status = Command::new("strip").arg(&dest).status()?;
                anyhow::ensure!(status.success(), "Strip failed for \"{}\"", dest.display());
            }
        }

        Ok(())
    }

    fn files(&self) -> Result<(), Error> {
        let project_root = crate::project_root();

        for filename in STATIC_FILES {
            let src = project_root.join(filename);
            let dest = self.output_dir.join(filename);
            crate::copy(src, dest)?;
        }

        Ok(())
    }
}

static DIST_DIR: Lazy<PathBuf> = Lazy::new(|| crate::project_root().join("target/dist"));

fn host_triple() -> Result<String, Error> {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let output = Command::new(rustc)
        .arg("--version")
        .arg("--verbose")
        .stdout(Stdio::piped())
        .output()?;

    anyhow::ensure!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;

    for line in stdout.lines() {
        if let Some(triple) = line.strip_prefix("host: ") {
            return Ok(triple.trim().to_string());
        }
    }

    Err(Error::msg("Unable to determine the host target triple"))
}
