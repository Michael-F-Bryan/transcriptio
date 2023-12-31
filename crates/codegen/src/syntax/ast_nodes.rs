use heck::{ToPascalCase, ToShoutySnekCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use ungrammar::{Grammar, Node, NodeData, Rule};

use crate::syntax::{tokens::Token, SyntaxKind};

pub fn ast_nodes(grammar: &Grammar, syntax_kind: &SyntaxKind) -> impl ToTokens {
    let mut nodes = Vec::new();

    for node in grammar.iter() {
        let NodeData { name, rule } = &grammar[node];

        let kind = top_level(grammar, rule, name, syntax_kind);
        nodes.push(AstNode {
            formatted_rule: format_rule(grammar, node),
            type_name: ast_node_name(name),
            kind: deduplicate_fields(kind),
        });
    }

    let tokens = syntax_kind
        .tokens
        .iter()
        .map(|t| TokenNode {
            syntax_kind: t.syntax_kind.clone(),
            token: t.token.clone(),
        })
        .collect();

    AstNodes { nodes, tokens }
}

fn ast_node_name(name: &str) -> Ident {
    let mut name = name.to_pascal_case();

    let blacklist = ["Option"];

    if blacklist.contains(&name.as_str()) {
        name.push('_');
    }

    format_ident!("{}", name)
}

fn deduplicate_fields(mut kind: AstNodeKind) -> AstNodeKind {
    let fields = match &mut kind {
        AstNodeKind::EnumNode { variants } => variants,
        AstNodeKind::StructNode { fields, .. } => fields,
    };

    let mut deduplicated: Vec<Field> = Vec::new();

    for field in fields.drain(..) {
        match deduplicated
            .iter_mut()
            .find(|f| f.syntax_kind == field.syntax_kind)
        {
            Some(original) => {
                original.multiplicity = Multiplicity::Multiple;
            }
            None => {
                deduplicated.push(field);
            }
        }
    }

    *fields = deduplicated;

    kind
}

fn top_level(grammar: &Grammar, rule: &Rule, name: &str, syntax_kind: &SyntaxKind) -> AstNodeKind {
    match rule {
        Rule::Alt(rules) => AstNodeKind::EnumNode {
            variants: rules
                .iter()
                .flat_map(|r| top_level_fields(grammar, r, syntax_kind))
                .collect(),
        },
        Rule::Node(n) => AstNodeKind::StructNode {
            syntax_kind: format_ident!("{}", name.TO_SHOUTY_SNEK_CASE()),
            fields: vec![node_field(grammar, *n)],
        },
        Rule::Token(t) => AstNodeKind::StructNode {
            syntax_kind: format_ident!("{}", name.TO_SHOUTY_SNEK_CASE()),
            fields: vec![token_field(syntax_kind, grammar, *t)],
        },
        Rule::Rep(rule) => {
            let mut fields = top_level_fields(grammar, rule, syntax_kind);
            for field in &mut fields {
                field.multiplicity = Multiplicity::Multiple;
            }
            AstNodeKind::StructNode {
                syntax_kind: format_ident!("{}", name.TO_SHOUTY_SNEK_CASE()),
                fields,
            }
        }
        Rule::Opt(rule) => {
            let mut fields = top_level_fields(grammar, rule, syntax_kind);
            for field in &mut fields {
                field.multiplicity = Multiplicity::Optional;
            }
            AstNodeKind::StructNode {
                syntax_kind: format_ident!("{}", name.TO_SHOUTY_SNEK_CASE()),
                fields,
            }
        }
        Rule::Seq(rules) => AstNodeKind::StructNode {
            syntax_kind: format_ident!("{}", name.TO_SHOUTY_SNEK_CASE()),
            fields: rules
                .iter()
                .flat_map(|r| top_level_fields(grammar, r, syntax_kind))
                .collect(),
        },
        Rule::Labeled { label, rule } => {
            let mut fields = top_level_fields(grammar, rule, syntax_kind);
            for field in &mut fields {
                field.name = format_ident!("{label}");
            }
            AstNodeKind::StructNode {
                syntax_kind: format_ident!("{}", name.TO_SHOUTY_SNEK_CASE()),
                fields,
            }
        }
    }
}

fn token_field(syntax_kind: &SyntaxKind, grammar: &Grammar, t: ungrammar::Token) -> Field {
    let Token { syntax_kind, .. } = syntax_kind.get_token(&grammar[t].name).unwrap();

    Field {
        name: format_ident!("{}", syntax_kind.to_string().to_pascal_case()),
        syntax_kind: syntax_kind.clone(),
        multiplicity: Multiplicity::One,
        kind: VariantKind::Token,
    }
}

fn top_level_fields(grammar: &Grammar, rule: &Rule, syntax_kind: &SyntaxKind) -> Vec<Field> {
    match rule {
        Rule::Labeled { rule, .. } => top_level_fields(grammar, rule, syntax_kind),
        Rule::Node(n) => vec![node_field(grammar, *n)],
        Rule::Token(t) => vec![token_field(syntax_kind, grammar, *t)],
        Rule::Alt(rules) | Rule::Seq(rules) => rules
            .iter()
            .flat_map(|r| top_level_fields(grammar, r, syntax_kind))
            .collect(),
        Rule::Opt(rule) => {
            let mut fields = top_level_fields(grammar, rule, syntax_kind);

            for field in &mut fields {
                field.multiplicity = Multiplicity::Optional;
            }
            fields
        }
        Rule::Rep(rule) => {
            let mut fields = top_level_fields(grammar, rule, syntax_kind);

            for field in &mut fields {
                field.multiplicity = Multiplicity::Multiple;
            }
            fields
        }
    }
}

fn node_field(grammar: &Grammar, node: Node) -> Field {
    let NodeData { name, .. } = &grammar[node];
    let ident = ast_node_name(name);

    Field {
        name: ident.clone(),
        syntax_kind: ident.clone(),
        multiplicity: Multiplicity::One,
        kind: VariantKind::AstNode(ident),
    }
}

#[derive(Debug, Clone)]
pub struct AstNodes {
    nodes: Vec<AstNode>,
    tokens: Vec<TokenNode>,
}

impl ToTokens for AstNodes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            //! Automatically generated, strongly-typed [`AstNode`]s.
            #![allow(clippy::redundant_clone, unreachable_patterns)]
            use rowan::{ast::AstNode, TextRange};
            use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxToken};
        });

        for node in &self.nodes {
            node.to_tokens(tokens);
        }

        for node in &self.tokens {
            node.to_tokens(tokens);
        }
    }
}

#[derive(Debug, Clone)]
struct TokenNode {
    syntax_kind: Ident,
    token: Option<String>,
}

impl ToTokens for TokenNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TokenNode { syntax_kind, token } = self;

        let type_name = format_ident!("{}Token", syntax_kind.to_string().to_pascal_case());
        let docs = match token {
            Some(symbol) => format!("The [`SyntaxKind::{syntax_kind}`] token (`{symbol}`)."),
            None => format!("The [`SyntaxKind::{syntax_kind}`] token."),
        };

        tokens.extend(quote! {
            #[doc = #docs]
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #type_name(SyntaxNode);

            impl #type_name {
                /// Get the underlying [`SyntaxToken`].
                pub fn token(&self) -> SyntaxToken {
                    self.0.children_with_tokens()
                        .filter_map(|element| element.into_token())
                        .find(|token| token.kind() == SyntaxKind::#syntax_kind)
                        .unwrap()
                }
            }

            impl AstNode for #type_name {
                type Language = crate::syntax::WebVTT;

                fn can_cast(kind: SyntaxKind) -> bool
                where
                    Self: Sized
                {
                    kind == SyntaxKind::#syntax_kind
                }

                fn cast(node: SyntaxNode) -> Option<Self>
                where
                    Self: Sized
                {
                    if #type_name::can_cast(node.kind()) {
                        Some(#type_name(node))
                    } else {
                        None
                    }
                }

                fn syntax(&self) -> &SyntaxNode {
                    &self.0
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
struct AstNode {
    formatted_rule: String,
    type_name: Ident,
    kind: AstNodeKind,
}

impl AstNode {
    fn definition(&self) -> TokenStream {
        let AstNode {
            formatted_rule,
            type_name,
            kind,
        } = self;

        let formatted_rule = quote! {
           #[doc = ""]
           #[doc = "Grammar:"]
           #[doc = "```text"]
           #[doc = #formatted_rule]
           #[doc = "```"]
        };

        match kind {
            AstNodeKind::StructNode { syntax_kind, .. } => {
                let docs =
                    format!("A strongly typed wrapper around a [`{syntax_kind}`][SyntaxKind::{syntax_kind}] node.");
                quote! {
                    #[doc = #docs]
                    #formatted_rule
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub struct #type_name(SyntaxNode);
                }
            }
            AstNodeKind::EnumNode { variants } => {
                let variants = variants.iter().map(|Field { name, kind, .. }| {
                    let ty = match kind {
                        VariantKind::AstNode(n) => n.clone(),
                        VariantKind::Token => format_ident!("{name}Token"),
                    };

                    quote!(#name(#ty))
                });
                let docs = format!("A strongly typed `{type_name}` node.");

                quote! {
                    #[doc = #docs]
                    #formatted_rule
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub enum #type_name {
                        #(#variants,)*
                    }
                }
            }
        }
    }

    fn ast_node_impl(&self) -> TokenStream {
        let AstNode {
            type_name, kind, ..
        } = self;

        match kind {
            AstNodeKind::StructNode { syntax_kind, .. } => quote! {
                impl AstNode for #type_name {
                    type Language = crate::syntax::WebVTT;

                    fn can_cast(kind: SyntaxKind) -> bool
                    where
                        Self: Sized
                    {
                        kind == SyntaxKind::#syntax_kind
                    }

                    fn cast(node: SyntaxNode) -> Option<Self>
                    where
                        Self: Sized
                    {
                        if #type_name::can_cast(node.kind()) {
                            Some(#type_name(node))
                        } else {
                            None
                        }
                    }

                    fn syntax(&self) -> &SyntaxNode {
                        &self.0
                    }
                }
            },
            AstNodeKind::EnumNode { variants } => {
                let checks: Vec<_> = variants
                    .iter()
                    .map(|v| match &v.kind {
                        VariantKind::AstNode(name) => quote!(#name::can_cast(kind)),
                        VariantKind::Token => {
                            let syntax_kind = &v.syntax_kind;
                            quote!(kind == SyntaxKind::#syntax_kind)
                        }
                    })
                    .collect();
                let casts = variants.iter().map(|v| {
                    let name = &v.name;

                    match &v.kind {
                        VariantKind::AstNode(ty) => {
                            quote! {
                                if let Some(variant) = #ty::cast(node.clone()) {
                                    return Some(#type_name::#name(variant));
                                }
                            }
                        }
                        VariantKind::Token => {
                            let token_type = format_ident!("{name}Token");
                            quote! {
                                if let Some(token) = #token_type::cast(node.clone()) {
                                    return Some(#type_name::#name(token));
                                }
                            }
                        }
                    }
                });
                let syntax = variants
                    .iter()
                    .map(|Field { name, .. }| quote!(#type_name::#name(node) => node.syntax()));

                quote! {
                    impl AstNode for #type_name {
                        type Language = crate::syntax::WebVTT;

                        fn can_cast(kind: SyntaxKind) -> bool
                        where
                            Self: Sized
                        {
                            #(#checks)||*
                        }

                        fn cast(node: SyntaxNode) -> Option<Self>
                        where
                            Self: Sized
                        {
                            #( #casts )*
                            None
                        }

                        fn syntax(&self) -> &SyntaxNode {
                            match self {
                                #(#syntax,)*
                            }
                        }
                    }
                }
            }
        }
    }

    fn methods(&self) -> TokenStream {
        let AstNode {
            type_name, kind, ..
        } = self;

        let methods: Vec<TokenStream> = match kind {
            AstNodeKind::StructNode { fields, .. } => fields.iter().map(accessor).collect(),
            AstNodeKind::EnumNode { variants } => variants.iter().map(enum_getter).collect(),
        };

        quote! {
            impl #type_name {
                #( #methods )*

                pub fn span(&self) -> TextRange {
                    self.syntax().text_range()
                }
            }
        }
    }
}

impl ToTokens for AstNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.definition());
        tokens.extend(self.ast_node_impl());
        tokens.extend(self.methods());
    }
}

fn accessor(field: &Field) -> TokenStream {
    let Field {
        name,
        multiplicity,
        kind,
        ..
    } = field;

    let method_name = accessor_name(name, multiplicity);

    let node_ty = match kind {
        VariantKind::AstNode(node_ty) => node_ty.clone(),
        VariantKind::Token => format_ident!("{name}Token"),
    };

    match multiplicity {
        Multiplicity::Multiple => quote! {
            pub fn #method_name(&self) -> impl Iterator<Item = #node_ty> {
                self.0.children().filter_map(#node_ty::cast)
            }
        },
        Multiplicity::One | Multiplicity::Optional => quote! {
                pub fn #method_name(&self) -> Option<#node_ty> {
                    self.0.children().find_map(#node_ty::cast)
                }
        },
    }
}

fn accessor_name(name: &Ident, multiplicity: &Multiplicity) -> Ident {
    let mut method_name = name.to_string().to_snake_case();
    if matches!(multiplicity, Multiplicity::Multiple) && !method_name.ends_with('s') {
        method_name.push('s');
    }
    if matches!(multiplicity, Multiplicity::Optional) {
        method_name.push_str("_opt");
    }

    let method_name = format_ident!("{method_name}");
    method_name
}

fn enum_getter(field: &Field) -> TokenStream {
    let Field { name, kind, .. } = field;

    let is_method_name = format_ident!("is_{}", name.to_string().to_snake_case());
    let as_method_name = format_ident!("as_{}", name.to_string().to_snake_case());

    let getter = match kind {
        VariantKind::AstNode(n) => enum_node_getters(name, n, &as_method_name),
        VariantKind::Token => enum_token_getter(name, &as_method_name),
    };

    quote! {
        #getter

        pub fn #is_method_name(&self) -> bool {
            self.#as_method_name().is_some()
        }
    }
}

fn enum_token_getter(variant_name: &Ident, method_name: &Ident) -> TokenStream {
    quote! {
        pub fn #method_name(&self) -> Option<SyntaxToken> {
            match self {
                Self::#variant_name(node) => Some(node.token()),
                _ => None,
            }
        }
    }
}

fn enum_node_getters(variant_name: &Ident, type_name: &Ident, method_name: &Ident) -> TokenStream {
    quote! {
        pub fn #method_name(&self) -> Option<#type_name> {
            match self {
                Self::#variant_name(node) => Some(node.clone()),
                _ => None,
            }
        }
    }
}

#[derive(Debug, Clone)]
enum AstNodeKind {
    StructNode {
        syntax_kind: Ident,
        fields: Vec<Field>,
    },
    EnumNode {
        variants: Vec<Field>,
    },
}

#[derive(Debug, Clone)]
struct Field {
    syntax_kind: Ident,
    name: Ident,
    multiplicity: Multiplicity,
    kind: VariantKind,
}

#[derive(Debug, Clone)]
enum VariantKind {
    AstNode(Ident),
    Token,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Multiplicity {
    Optional,
    One,
    Multiple,
}

fn format_rule(grammar: &Grammar, node: ungrammar::Node) -> String {
    let mut buffer = String::new();
    format::node(grammar, node, &mut buffer).unwrap();
    buffer
}

mod format {
    use std::fmt::Write;
    use ungrammar::{Grammar, Node, NodeData, Rule, TokenData};

    pub(crate) fn node(grammar: &Grammar, node: Node, buffer: &mut String) -> std::fmt::Result {
        let NodeData { name, rule } = &grammar[node];

        write!(buffer, "{name} = ")?;
        fmt_rule(grammar, rule, 0, buffer)?;

        Ok(())
    }

    fn fmt_rule(
        grammar: &Grammar,
        rule: &Rule,
        depth: usize,
        buffer: &mut String,
    ) -> std::fmt::Result {
        match rule {
            Rule::Labeled { label, rule } => {
                write!(buffer, "{label}:")?;
                fmt_rule(grammar, rule, depth, buffer)?;

                Ok(())
            }
            Rule::Node(n) => {
                let NodeData { name, .. } = &grammar[*n];
                write!(buffer, "{name}")?;

                Ok(())
            }
            Rule::Token(t) => {
                let TokenData { name } = &grammar[*t];
                write!(buffer, "'{name}'")?;
                Ok(())
            }
            Rule::Seq(rules) => {
                if depth > 0 {
                    write!(buffer, "(")?;
                }
                for (i, rule) in rules.iter().enumerate() {
                    if i > 0 {
                        write!(buffer, " ")?;
                    }
                    fmt_rule(grammar, rule, depth + 1, buffer)?;
                }
                if depth > 0 {
                    write!(buffer, ")")?;
                }

                Ok(())
            }
            Rule::Alt(rules) => {
                if depth > 1 {
                    writeln!(buffer)?;
                    for _ in 0..depth {
                        write!(buffer, "  ")?;
                    }
                }
                if depth > 0 {
                    write!(buffer, "(")?;
                }
                for (i, rule) in rules.iter().enumerate() {
                    if i > 0 {
                        write!(buffer, " | ")?;
                    }
                    fmt_rule(grammar, rule, depth + 1, buffer)?;
                }
                if depth > 0 {
                    write!(buffer, ")")?;
                }
                if depth > 1 {
                    writeln!(buffer)?;
                }

                Ok(())
            }
            Rule::Opt(rule) => {
                fmt_rule(grammar, rule, depth + 1, buffer)?;
                write!(buffer, "?")?;
                Ok(())
            }
            Rule::Rep(rule) => {
                fmt_rule(grammar, rule, depth + 1, buffer)?;
                write!(buffer, "*")?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_formatting_works() {
        let example_grammar = r#"
            SourceFile = Item*

            Item = UseItem
                | ResourceItem

            UseItem = 'use' UseNames 'from' 'path'
            UseNames = '*' | UseNamesList
            UseNamesList = '{' (UseNamesItem ',')* '}'
            UseNamesItem = name:'ident' ('as' rename:'ident')?

            ResourceItem = Attributes 'resource' name:'ident' ( '{' ResourceDefs  '}' )?
            ResourceDefs = ResourceDef*
            ResourceDef = Attributes 'static'? Func

            Attributes = Attribute*
            Attribute = 'doc-comment'

            Func = name:'ident' ':' 'func' '(' FuncArgs ')' ('->' Ty)?
            FuncArgs = (FuncArg ',')*
            FuncArg = name:'ident' ':' Ty

            Ty = 'ident'
        "#;
        let grammar: Grammar = example_grammar.parse().unwrap();

        let formatted = grammar
            .iter()
            .map(|node| format_rule(&grammar, node))
            .collect::<Vec<_>>()
            .join("\n");

        insta::assert_display_snapshot!(&formatted);
    }
}
