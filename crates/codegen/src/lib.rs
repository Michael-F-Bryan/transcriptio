//! An internal crate used for generating Rust code.
//!
//! If you need to do some code generation, you'll typically implement the
//! generation in this crate then add a test to this `lib.rs` file.
//!
//! It is important that the `codegen` crate doesn't depend on any other crates
//! in this repository. That way, even if some generated code is
//! outdated/missing/broken, you can still run the `codegen` crate's tests and
//! restore the world to a valid state.

pub mod syntax;
mod utils;

pub use utils::{ensure_file_contents, format_rust, project_root};

#[cfg(test)]
mod tests {
    use crate::syntax::{ast_nodes, SyntaxKind};

    use super::*;
    use ungrammar::Grammar;

    #[test]
    fn update_syntax_kind() {
        let grammar: Grammar = include_str!("../../../webvtt.ungram").parse().unwrap();

        let syntax_kind = SyntaxKind::new(&grammar);

        ensure_file_contents(
            project_root().join("crates/transcriptio/src/syntax/syntax_kind.rs"),
            format_rust(&syntax_kind),
        );

        ensure_file_contents(
            project_root().join("crates/transcriptio/src/syntax/ast.rs"),
            format_rust(ast_nodes(&grammar, &syntax_kind)),
        );
    }
}
