//! Run basic parsing and type checking over a large number of known `*.vtt`
//! files.

use std::path::Path;

use anyhow::Error;
use libtest_mimic::Arguments;

fn main() -> Result<(), Error> {
    let arguments = Arguments::from_args();

    let corpus_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
    let test_cases = transcriptio_integration_tests::discover_tests(&corpus_dir)?;

    libtest_mimic::run(&arguments, test_cases).exit();
}
