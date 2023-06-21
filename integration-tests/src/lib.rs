use std::{fmt::Write, path::Path};

use anyhow::{Context, Error};
use libtest_mimic::{Failed, Trial};
use walkdir::WalkDir;

#[tracing::instrument(skip_all)]
pub fn discover_tests(corpus_dir: &Path) -> Result<Vec<Trial>, Error> {
    let corpus_dir = corpus_dir
        .canonicalize()
        .with_context(|| format!("Unable to canonicalize \"{}\"", corpus_dir.display()))?;

    let mut test_cases = Vec::new();

    let vtt_files = WalkDir::new(&corpus_dir).into_iter().filter_entry(|entry| {
        entry.file_type().is_dir() || entry.path().extension() == Some("vtt".as_ref())
    });

    for entry in vtt_files {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let test_case = infer_test_case(&corpus_dir, entry.path())?;
        test_cases.push(test_case);
    }

    Ok(test_cases)
}

#[tracing::instrument(level = "debug", skip_all, fields(vtt_file=%vtt_file.display()))]
fn infer_test_case(corpus_dir: &Path, vtt_file: &Path) -> Result<Trial, Error> {
    let vtt_file = vtt_file.to_path_buf();
    let relative_path = vtt_file.strip_prefix(corpus_dir).unwrap();

    let (name_segments, ignored) = test_name(relative_path);

    let trial =
        Trial::test(name_segments.join("::"), move || parse(&vtt_file)).with_ignored_flag(ignored);

    Ok(trial)
}

#[tracing::instrument(level = "debug", skip_all, fields(vtt_file=%vtt_file.display()))]
fn parse(vtt_file: &Path) -> Result<(), Failed> {
    let src = std::fs::read_to_string(vtt_file)
        .with_context(|| format!("Unable to read \"{}\"", vtt_file.display()))
        .map_err(|e| format!("{e:?}"))?;

    let (ast, errors) = transcriptio::vtt::parse(&src);

    if errors.is_empty() {
        return Ok(());
    }

    let mut error_message = format!("Parsing \"{}\" failed with errors", vtt_file.display());
    writeln!(error_message, "==== AST ====").unwrap();
    writeln!(error_message, "{ast:#?}").unwrap();
    writeln!(error_message, "==== Errors ====").unwrap();
    writeln!(error_message, "{errors:#?}").unwrap();

    Err(Failed::from(error_message))
}

fn test_name(path: &Path) -> (Vec<String>, bool) {
    let mut segments = Vec::new();
    let mut ignored = false;

    for component in path.iter() {
        let component = component.to_string_lossy();
        let component = component.trim_end_matches(".vtt");

        match component.strip_prefix('_') {
            Some(name) => {
                ignored = true;
                segments.push(name.to_string());
            }
            None => {
                segments.push(component.to_string());
            }
        }
    }

    (segments, ignored)
}
