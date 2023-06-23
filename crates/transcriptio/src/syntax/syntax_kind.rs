/// The different types of terminals and non-terminals in the
/// WAI language grammar.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(nonstandard_style)]
#[non_exhaustive]
pub enum SyntaxKind {
    ///The `-->` symbol.
    ARROW = 0u16,
    ///A comment.
    COMMENT = 1u16,
    ///End of input.
    EOF = 2u16,
    ///A lexer error.
    ERROR = 3u16,
    ///An identifier.
    IDENT = 4u16,
    ///The `identifier` keyword.
    IDENTIFIER_KW = 5u16,
    ///The `NOTE` keyword.
    NOTE_KW = 6u16,
    ///The `payload` keyword.
    PAYLOAD_KW = 7u16,
    ///The `REGION` keyword.
    REGION_KW = 8u16,
    ///The `STYLE` keyword.
    STYLE_KW = 9u16,
    ///A timestamp literal.
    TIMESTAMP = 10u16,
    ///The `WEBVTT` keyword.
    WEBVTT_KW = 11u16,
    ///One or more whitespace characters (spaces, tabs, newlines, etc.).
    WHITESPACE = 12u16,
    SOURCE_FILE = 13u16,
    MAGIC = 14u16,
    REGION_DEFINITION = 15u16,
    STYLE = 16u16,
    COMMENT_BLOCK = 17u16,
    CUE = 18u16,
    TIMINGS = 19u16,
    CUE_PAYLOAD = 20u16,
}
impl SyntaxKind {
    /// All the possible [`SyntaxKind`] variants.
    pub const fn all() -> [SyntaxKind; 21usize] {
        [
            SyntaxKind::ARROW,
            SyntaxKind::COMMENT,
            SyntaxKind::EOF,
            SyntaxKind::ERROR,
            SyntaxKind::IDENT,
            SyntaxKind::IDENTIFIER_KW,
            SyntaxKind::NOTE_KW,
            SyntaxKind::PAYLOAD_KW,
            SyntaxKind::REGION_KW,
            SyntaxKind::STYLE_KW,
            SyntaxKind::TIMESTAMP,
            SyntaxKind::WEBVTT_KW,
            SyntaxKind::WHITESPACE,
            SyntaxKind::SOURCE_FILE,
            SyntaxKind::MAGIC,
            SyntaxKind::REGION_DEFINITION,
            SyntaxKind::STYLE,
            SyntaxKind::COMMENT_BLOCK,
            SyntaxKind::CUE,
            SyntaxKind::TIMINGS,
            SyntaxKind::CUE_PAYLOAD,
        ]
    }
    /// Is this [`SyntaxKind`] a piece of punctuation?
    ///
    /// ```rust
    /// # use transcriptio::T;
    /// let kind = T![-->];
    /// assert!(kind.is_punctuation())
    /// ```
    pub const fn is_punctuation(self) -> bool {
        matches!(self, SyntaxKind::ARROW)
    }
    /// Is this [`SyntaxKind`] a keyword?
    ///
    /// ```rust
    /// # use transcriptio::T;
    /// let kind = T![WEBVTT];
    /// assert!(kind.is_keyword())
    /// ```
    pub const fn is_keyword(self) -> bool {
        matches!(
            self, SyntaxKind::IDENTIFIER_KW | SyntaxKind::NOTE_KW |
            SyntaxKind::PAYLOAD_KW | SyntaxKind::REGION_KW | SyntaxKind::STYLE_KW |
            SyntaxKind::WEBVTT_KW
        )
    }
    /// Given a textual symbol try to get the associated
    /// [`SyntaxKind`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use transcriptio::syntax::SyntaxKind;
    /// let kind = SyntaxKind::from_symbol("-->").unwrap();
    /// assert_eq!(kind, SyntaxKind::ARROW);
    /// ```
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol {
            "-->" => Some(SyntaxKind::ARROW),
            _ => None,
        }
    }
    /// get the [`SyntaxKind`] that corresponds to a particular
    /// variant.
    ///
    /// ```rust
    /// # use transcriptio::syntax::SyntaxKind;
    /// let kind = SyntaxKind::IDENT;
    /// let code: u16 = kind.into();
    ///
    /// let round_tripped = SyntaxKind::from_code(code).unwrap();
    ///
    /// assert_eq!(round_tripped, kind);
    /// ```
    pub fn from_code(n: u16) -> Option<Self> {
        match n {
            0u16 => Some(SyntaxKind::ARROW),
            1u16 => Some(SyntaxKind::COMMENT),
            2u16 => Some(SyntaxKind::EOF),
            3u16 => Some(SyntaxKind::ERROR),
            4u16 => Some(SyntaxKind::IDENT),
            5u16 => Some(SyntaxKind::IDENTIFIER_KW),
            6u16 => Some(SyntaxKind::NOTE_KW),
            7u16 => Some(SyntaxKind::PAYLOAD_KW),
            8u16 => Some(SyntaxKind::REGION_KW),
            9u16 => Some(SyntaxKind::STYLE_KW),
            10u16 => Some(SyntaxKind::TIMESTAMP),
            11u16 => Some(SyntaxKind::WEBVTT_KW),
            12u16 => Some(SyntaxKind::WHITESPACE),
            13u16 => Some(SyntaxKind::SOURCE_FILE),
            14u16 => Some(SyntaxKind::MAGIC),
            15u16 => Some(SyntaxKind::REGION_DEFINITION),
            16u16 => Some(SyntaxKind::STYLE),
            17u16 => Some(SyntaxKind::COMMENT_BLOCK),
            18u16 => Some(SyntaxKind::CUE),
            19u16 => Some(SyntaxKind::TIMINGS),
            20u16 => Some(SyntaxKind::CUE_PAYLOAD),
            _ => None,
        }
    }
}
impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(k: rowan::SyntaxKind) -> Self {
        SyntaxKind::from_code(k.0).unwrap()
    }
}
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(k: SyntaxKind) -> Self {
        rowan::SyntaxKind(k.into())
    }
}
impl From<SyntaxKind> for u16 {
    fn from(k: SyntaxKind) -> Self {
        k as u16
    }
}
impl From<SyntaxKind> for m_lexer::TokenKind {
    fn from(k: SyntaxKind) -> m_lexer::TokenKind {
        m_lexer::TokenKind(k.into())
    }
}
/// A helper macro for getting the [`SyntaxKind`] that corresponds
/// to a particular token.
#[macro_export]
macro_rules! T {
    (-->) => {
        $crate ::syntax::SyntaxKind::ARROW
    };
    (comment) => {
        $crate ::syntax::SyntaxKind::COMMENT
    };
    (identifier) => {
        $crate ::syntax::SyntaxKind::IDENTIFIER_KW
    };
    (NOTE) => {
        $crate ::syntax::SyntaxKind::NOTE_KW
    };
    (payload) => {
        $crate ::syntax::SyntaxKind::PAYLOAD_KW
    };
    (REGION) => {
        $crate ::syntax::SyntaxKind::REGION_KW
    };
    (STYLE) => {
        $crate ::syntax::SyntaxKind::STYLE_KW
    };
    (timestamp) => {
        $crate ::syntax::SyntaxKind::TIMESTAMP
    };
    (WEBVTT) => {
        $crate ::syntax::SyntaxKind::WEBVTT_KW
    };
}
