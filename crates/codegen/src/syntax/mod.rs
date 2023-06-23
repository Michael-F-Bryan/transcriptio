mod ast_nodes;
mod syntax_kind;
mod tokens;

pub use self::{ast_nodes::ast_nodes, syntax_kind::SyntaxKind};

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use ungrammar::{Grammar, Rule};

    #[test]
    fn all_rules_are_used() {
        let grammar: Grammar = include_str!("../../../../webvtt.ungram")
            .replace("\r\n", "\n")
            .parse()
            .unwrap();
        let mut rules: HashSet<_> = grammar.iter().map(|n| grammar[n].name.as_str()).collect();

        fn mark(rule: &Rule, rules: &mut HashSet<&str>, grammar: &Grammar) {
            match rule {
                Rule::Labeled { rule, .. } => mark(rule, rules, grammar),
                Rule::Node(n) => {
                    rules.remove(grammar[*n].name.as_str());
                }
                Rule::Token(_) => {}
                Rule::Seq(r) | Rule::Alt(r) => {
                    for rule in r {
                        mark(rule, rules, grammar);
                    }
                }
                Rule::Opt(rule) | Rule::Rep(rule) => mark(rule, rules, grammar),
            }
        }

        for n in grammar.iter() {
            mark(&grammar[n].rule, &mut rules, &grammar);
        }

        // the root node is always used
        let root_node = grammar.iter().map(|n| &grammar[n].name).next().unwrap();
        rules.remove(root_node.as_str());

        if !rules.is_empty() {
            panic!("Unused rules: {rules:?}");
        }
    }
}
