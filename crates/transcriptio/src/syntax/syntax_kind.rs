/// The different types of terminals and non-terminals in the
/// WAI language grammar.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(nonstandard_style)]
#[non_exhaustive]
pub enum SyntaxKind {
    ///The `-->` symbol.
    ARROW = 0u16,
    ///The `:` symbol.
    COLON = 1u16,
    ///The `.` symbol.
    DOT = 2u16,
    ///End of input.
    EOF = 3u16,
    ///A lexer error.
    ERROR = 4u16,
    ///An identifier.
    IDENT = 5u16,
    ///The `identifier` keyword.
    IDENTIFIER_KW = 6u16,
    ///A line of text.
    LINE = 7u16,
    ///The newline character
    NEWLINE = 8u16,
    ///The `NOTE` keyword.
    NOTE_KW = 9u16,
    ///The `number` keyword.
    NUMBER_KW = 10u16,
    ///The `REGION` keyword.
    REGION_KW = 11u16,
    ///The `STYLE` keyword.
    STYLE_KW = 12u16,
    ///The `WEBVTT` keyword.
    WEBVTT_KW = 13u16,
    ///One or more whitespace characters (spaces, tabs, etc.).
    WHITESPACE = 14u16,
    SOURCE_FILE = 15u16,
    MAGIC = 16u16,
    REGION_DEFINITION = 17u16,
    STYLE = 18u16,
    COMMENT_BLOCK = 19u16,
    CUE = 20u16,
    TIMINGS = 21u16,
    CUE_PAYLOAD = 22u16,
    TIMESTAMP = 23u16,
}
impl SyntaxKind {
    /// All the possible [`SyntaxKind`] variants.
    pub const fn all() -> [SyntaxKind; 24usize] {
        [
            SyntaxKind::ARROW,
            SyntaxKind::COLON,
            SyntaxKind::DOT,
            SyntaxKind::EOF,
            SyntaxKind::ERROR,
            SyntaxKind::IDENT,
            SyntaxKind::IDENTIFIER_KW,
            SyntaxKind::LINE,
            SyntaxKind::NEWLINE,
            SyntaxKind::NOTE_KW,
            SyntaxKind::NUMBER_KW,
            SyntaxKind::REGION_KW,
            SyntaxKind::STYLE_KW,
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
            SyntaxKind::TIMESTAMP,
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
        matches!(self, SyntaxKind::ARROW | SyntaxKind::COLON | SyntaxKind::DOT)
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
            self, SyntaxKind::IDENTIFIER_KW | SyntaxKind::NOTE_KW | SyntaxKind::NUMBER_KW
            | SyntaxKind::REGION_KW | SyntaxKind::STYLE_KW | SyntaxKind::WEBVTT_KW
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
            ":" => Some(SyntaxKind::COLON),
            "." => Some(SyntaxKind::DOT),
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
            1u16 => Some(SyntaxKind::COLON),
            2u16 => Some(SyntaxKind::DOT),
            3u16 => Some(SyntaxKind::EOF),
            4u16 => Some(SyntaxKind::ERROR),
            5u16 => Some(SyntaxKind::IDENT),
            6u16 => Some(SyntaxKind::IDENTIFIER_KW),
            7u16 => Some(SyntaxKind::LINE),
            8u16 => Some(SyntaxKind::NEWLINE),
            9u16 => Some(SyntaxKind::NOTE_KW),
            10u16 => Some(SyntaxKind::NUMBER_KW),
            11u16 => Some(SyntaxKind::REGION_KW),
            12u16 => Some(SyntaxKind::STYLE_KW),
            13u16 => Some(SyntaxKind::WEBVTT_KW),
            14u16 => Some(SyntaxKind::WHITESPACE),
            15u16 => Some(SyntaxKind::SOURCE_FILE),
            16u16 => Some(SyntaxKind::MAGIC),
            17u16 => Some(SyntaxKind::REGION_DEFINITION),
            18u16 => Some(SyntaxKind::STYLE),
            19u16 => Some(SyntaxKind::COMMENT_BLOCK),
            20u16 => Some(SyntaxKind::CUE),
            21u16 => Some(SyntaxKind::TIMINGS),
            22u16 => Some(SyntaxKind::CUE_PAYLOAD),
            23u16 => Some(SyntaxKind::TIMESTAMP),
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
/// A helper macro for getting the [`SyntaxKind`] that corresponds
/// to a particular token.
#[macro_export]
macro_rules! T {
    (-->) => {
        $crate ::syntax::SyntaxKind::ARROW
    };
    (:) => {
        $crate ::syntax::SyntaxKind::COLON
    };
    (.) => {
        $crate ::syntax::SyntaxKind::DOT
    };
    (identifier) => {
        $crate ::syntax::SyntaxKind::IDENTIFIER_KW
    };
    (line) => {
        $crate ::syntax::SyntaxKind::LINE
    };
    (NOTE) => {
        $crate ::syntax::SyntaxKind::NOTE_KW
    };
    (number) => {
        $crate ::syntax::SyntaxKind::NUMBER_KW
    };
    (REGION) => {
        $crate ::syntax::SyntaxKind::REGION_KW
    };
    (STYLE) => {
        $crate ::syntax::SyntaxKind::STYLE_KW
    };
    (WEBVTT) => {
        $crate ::syntax::SyntaxKind::WEBVTT_KW
    };
}
