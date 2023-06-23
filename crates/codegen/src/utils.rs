use quote::ToTokens;
use std::path::Path;

/// Get the root directory for this repository.
pub fn project_root() -> &'static Path {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap();
    assert!(root_dir.join(".git").exists());

    root_dir
}

/// Format some Rust tokens.
///
/// # Panics
///
/// It is assumed that the tokens would parse as a Rust file.
pub fn format_rust(contents: impl ToTokens) -> String {
    let contents =
        syn::parse2(contents.to_token_stream()).expect("Unable to parse the tokens as a syn::File");
    prettyplease::unparse(&contents)
}

/// Check that a particular file has the desired contents.
///
/// If the file is missing or outdated, this function will update the file and
/// trigger a panic to fail any test this is called from.
pub fn ensure_file_contents(path: impl AsRef<Path>, contents: impl AsRef<str>) {
    let path = path.as_ref();
    let contents = normalize_newlines(contents.as_ref());

    if let Ok(old_contents) = std::fs::read_to_string(path) {
        if contents == normalize_newlines(&old_contents) {
            // File is already up to date
            return;
        }
    }

    let display_path = path.strip_prefix(project_root()).unwrap_or(path);

    eprintln!("{} was not up-to-date, updating...", display_path.display());

    if std::env::var("CI").is_ok() {
        eprintln!("Note: run `cargo test` locally and commit the updated files");
    }

    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, contents).unwrap();
    panic!("some file was not up to date and has been updated. Please re-run the tests.");
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}
