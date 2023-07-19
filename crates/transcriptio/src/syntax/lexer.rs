use crate::syntax::SyntaxKind;

/// Split the input text into its tokens.
///
/// Unknown input will be preserved as an [`ERROR`] token.
pub fn tokenize(_input: &str) -> impl Iterator<Item = (SyntaxKind, &'_ str)> {
    if false {
        return Vec::new().into_iter();
    }
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxKind::*;

    #[test]
    #[ignore = "TODO: Implement the lexer"]
    fn tokenize_the_kitchen_sink() {
        let src = "";

        let tokens: Vec<_> = tokenize(src).collect();

        let round_tripped: String = tokens.iter().map(|(_, text)| *text).collect();
        assert_eq!(round_tripped, src);
        tokens
            .iter()
            .copied()
            .for_each(|(kind, text)| assert_ne!(kind, ERROR, "Invalid token: {text:?}"));
        insta::assert_debug_snapshot!(tokens);
    }
}
