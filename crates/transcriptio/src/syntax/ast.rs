//! Automatically generated, strongly-typed [`AstNode`]s.
#![allow(clippy::redundant_clone, unreachable_patterns)]
use rowan::{ast::AstNode, TextRange};
use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
///A strongly typed wrapper around a [`SOURCE_FILE`][SyntaxKind::SOURCE_FILE] node.
///
///Grammar:
///```text
///SourceFile = Magic HeaderItems* Items*
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile(SyntaxNode);
impl AstNode for SourceFile {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::SOURCE_FILE
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if SourceFile::can_cast(node.kind()) { Some(SourceFile(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl SourceFile {
    pub fn magic(&self) -> Option<Magic> {
        self.0.children().find_map(Magic::cast)
    }
    pub fn header_items(&self) -> impl Iterator<Item = HeaderItems> {
        self.0.children().filter_map(HeaderItems::cast)
    }
    pub fn items(&self) -> impl Iterator<Item = Items> {
        self.0.children().filter_map(Items::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`MAGIC`][SyntaxKind::MAGIC] node.
///
///Grammar:
///```text
///Magic = 'WEBVTT'
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Magic(SyntaxNode);
impl AstNode for Magic {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::MAGIC
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Magic::can_cast(node.kind()) { Some(Magic(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Magic {
    pub fn webvtt_kw(&self) -> Option<WebvttKwToken> {
        self.0.children().find_map(WebvttKwToken::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed `HeaderItems` node.
///
///Grammar:
///```text
///HeaderItems = RegionDefinition | Style | CommentBlock
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HeaderItems {
    RegionDefinition(RegionDefinition),
    Style(Style),
    CommentBlock(CommentBlock),
}
impl AstNode for HeaderItems {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        RegionDefinition::can_cast(kind) || Style::can_cast(kind)
            || CommentBlock::can_cast(kind)
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(variant) = RegionDefinition::cast(node.clone()) {
            return Some(HeaderItems::RegionDefinition(variant));
        }
        if let Some(variant) = Style::cast(node.clone()) {
            return Some(HeaderItems::Style(variant));
        }
        if let Some(variant) = CommentBlock::cast(node.clone()) {
            return Some(HeaderItems::CommentBlock(variant));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            HeaderItems::RegionDefinition(node) => node.syntax(),
            HeaderItems::Style(node) => node.syntax(),
            HeaderItems::CommentBlock(node) => node.syntax(),
        }
    }
}
impl HeaderItems {
    pub fn as_region_definition(&self) -> Option<RegionDefinition> {
        match self {
            Self::RegionDefinition(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn is_region_definition(&self) -> bool {
        self.as_region_definition().is_some()
    }
    pub fn as_style(&self) -> Option<Style> {
        match self {
            Self::Style(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn is_style(&self) -> bool {
        self.as_style().is_some()
    }
    pub fn as_comment_block(&self) -> Option<CommentBlock> {
        match self {
            Self::CommentBlock(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn is_comment_block(&self) -> bool {
        self.as_comment_block().is_some()
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed `Items` node.
///
///Grammar:
///```text
///Items = Cue | CommentBlock
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Items {
    Cue(Cue),
    CommentBlock(CommentBlock),
}
impl AstNode for Items {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        Cue::can_cast(kind) || CommentBlock::can_cast(kind)
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(variant) = Cue::cast(node.clone()) {
            return Some(Items::Cue(variant));
        }
        if let Some(variant) = CommentBlock::cast(node.clone()) {
            return Some(Items::CommentBlock(variant));
        }
        None
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Items::Cue(node) => node.syntax(),
            Items::CommentBlock(node) => node.syntax(),
        }
    }
}
impl Items {
    pub fn as_cue(&self) -> Option<Cue> {
        match self {
            Self::Cue(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn is_cue(&self) -> bool {
        self.as_cue().is_some()
    }
    pub fn as_comment_block(&self) -> Option<CommentBlock> {
        match self {
            Self::CommentBlock(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn is_comment_block(&self) -> bool {
        self.as_comment_block().is_some()
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`REGION_DEFINITION`][SyntaxKind::REGION_DEFINITION] node.
///
///Grammar:
///```text
///RegionDefinition = 'REGION'
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegionDefinition(SyntaxNode);
impl AstNode for RegionDefinition {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::REGION_DEFINITION
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if RegionDefinition::can_cast(node.kind()) {
            Some(RegionDefinition(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl RegionDefinition {
    pub fn region_kw(&self) -> Option<RegionKwToken> {
        self.0.children().find_map(RegionKwToken::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`STYLE`][SyntaxKind::STYLE] node.
///
///Grammar:
///```text
///Style = 'STYLE'
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Style(SyntaxNode);
impl AstNode for Style {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::STYLE
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Style::can_cast(node.kind()) { Some(Style(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Style {
    pub fn style_kw(&self) -> Option<StyleKwToken> {
        self.0.children().find_map(StyleKwToken::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`COMMENT_BLOCK`][SyntaxKind::COMMENT_BLOCK] node.
///
///Grammar:
///```text
///CommentBlock = 'NOTE' 'comment'
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentBlock(SyntaxNode);
impl AstNode for CommentBlock {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::COMMENT_BLOCK
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if CommentBlock::can_cast(node.kind()) { Some(CommentBlock(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CommentBlock {
    pub fn note_kw(&self) -> Option<NoteKwToken> {
        self.0.children().find_map(NoteKwToken::cast)
    }
    pub fn comment(&self) -> Option<CommentToken> {
        self.0.children().find_map(CommentToken::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`CUE`][SyntaxKind::CUE] node.
///
///Grammar:
///```text
///Cue = 'identifier'? Timings CuePayload
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cue(SyntaxNode);
impl AstNode for Cue {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::CUE
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Cue::can_cast(node.kind()) { Some(Cue(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Cue {
    pub fn identifier_kw_opt(&self) -> Option<IdentifierKwToken> {
        self.0.children().find_map(IdentifierKwToken::cast)
    }
    pub fn timings(&self) -> Option<Timings> {
        self.0.children().find_map(Timings::cast)
    }
    pub fn cue_payload(&self) -> Option<CuePayload> {
        self.0.children().find_map(CuePayload::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`TIMINGS`][SyntaxKind::TIMINGS] node.
///
///Grammar:
///```text
///Timings = 'timestamp' '-->' 'timestamp'
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timings(SyntaxNode);
impl AstNode for Timings {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TIMINGS
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if Timings::can_cast(node.kind()) { Some(Timings(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl Timings {
    pub fn timestamps(&self) -> impl Iterator<Item = TimestampToken> {
        self.0.children().filter_map(TimestampToken::cast)
    }
    pub fn arrow(&self) -> Option<ArrowToken> {
        self.0.children().find_map(ArrowToken::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///A strongly typed wrapper around a [`CUE_PAYLOAD`][SyntaxKind::CUE_PAYLOAD] node.
///
///Grammar:
///```text
///CuePayload = 'payload'
///```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CuePayload(SyntaxNode);
impl AstNode for CuePayload {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::CUE_PAYLOAD
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if CuePayload::can_cast(node.kind()) { Some(CuePayload(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
impl CuePayload {
    pub fn payload_kw(&self) -> Option<PayloadKwToken> {
        self.0.children().find_map(PayloadKwToken::cast)
    }
    pub fn span(&self) -> TextRange {
        self.syntax().text_range()
    }
}
///The [`SyntaxKind::ARROW`] token (`-->`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrowToken(SyntaxNode);
impl ArrowToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::ARROW)
            .unwrap()
    }
}
impl AstNode for ArrowToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ARROW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if ArrowToken::can_cast(node.kind()) { Some(ArrowToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::COMMENT`] token (`comment`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentToken(SyntaxNode);
impl CommentToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::COMMENT)
            .unwrap()
    }
}
impl AstNode for CommentToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::COMMENT
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if CommentToken::can_cast(node.kind()) { Some(CommentToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::EOF`] token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EofToken(SyntaxNode);
impl EofToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::EOF)
            .unwrap()
    }
}
impl AstNode for EofToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::EOF
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if EofToken::can_cast(node.kind()) { Some(EofToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::ERROR`] token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorToken(SyntaxNode);
impl ErrorToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::ERROR)
            .unwrap()
    }
}
impl AstNode for ErrorToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::ERROR
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if ErrorToken::can_cast(node.kind()) { Some(ErrorToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::IDENT`] token (`ident`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentToken(SyntaxNode);
impl IdentToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::IDENT)
            .unwrap()
    }
}
impl AstNode for IdentToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::IDENT
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if IdentToken::can_cast(node.kind()) { Some(IdentToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::IDENTIFIER_KW`] token (`identifier`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentifierKwToken(SyntaxNode);
impl IdentifierKwToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::IDENTIFIER_KW)
            .unwrap()
    }
}
impl AstNode for IdentifierKwToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::IDENTIFIER_KW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if IdentifierKwToken::can_cast(node.kind()) {
            Some(IdentifierKwToken(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::NOTE_KW`] token (`NOTE`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoteKwToken(SyntaxNode);
impl NoteKwToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::NOTE_KW)
            .unwrap()
    }
}
impl AstNode for NoteKwToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::NOTE_KW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if NoteKwToken::can_cast(node.kind()) { Some(NoteKwToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::PAYLOAD_KW`] token (`payload`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PayloadKwToken(SyntaxNode);
impl PayloadKwToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::PAYLOAD_KW)
            .unwrap()
    }
}
impl AstNode for PayloadKwToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::PAYLOAD_KW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if PayloadKwToken::can_cast(node.kind()) {
            Some(PayloadKwToken(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::REGION_KW`] token (`REGION`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegionKwToken(SyntaxNode);
impl RegionKwToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::REGION_KW)
            .unwrap()
    }
}
impl AstNode for RegionKwToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::REGION_KW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if RegionKwToken::can_cast(node.kind()) {
            Some(RegionKwToken(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::STYLE_KW`] token (`STYLE`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StyleKwToken(SyntaxNode);
impl StyleKwToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::STYLE_KW)
            .unwrap()
    }
}
impl AstNode for StyleKwToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::STYLE_KW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if StyleKwToken::can_cast(node.kind()) { Some(StyleKwToken(node)) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::TIMESTAMP`] token (`timestamp`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TimestampToken(SyntaxNode);
impl TimestampToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::TIMESTAMP)
            .unwrap()
    }
}
impl AstNode for TimestampToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::TIMESTAMP
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if TimestampToken::can_cast(node.kind()) {
            Some(TimestampToken(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::WEBVTT_KW`] token (`WEBVTT`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WebvttKwToken(SyntaxNode);
impl WebvttKwToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::WEBVTT_KW)
            .unwrap()
    }
}
impl AstNode for WebvttKwToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::WEBVTT_KW
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if WebvttKwToken::can_cast(node.kind()) {
            Some(WebvttKwToken(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
///The [`SyntaxKind::WHITESPACE`] token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhitespaceToken(SyntaxNode);
impl WhitespaceToken {
    /// Get the underlying [`SyntaxToken`].
    pub fn token(&self) -> SyntaxToken {
        self.0
            .children_with_tokens()
            .filter_map(|element| element.into_token())
            .find(|token| token.kind() == SyntaxKind::WHITESPACE)
            .unwrap()
    }
}
impl AstNode for WhitespaceToken {
    type Language = crate::syntax::WebVTT;
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        kind == SyntaxKind::WHITESPACE
    }
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if WhitespaceToken::can_cast(node.kind()) {
            Some(WhitespaceToken(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}
