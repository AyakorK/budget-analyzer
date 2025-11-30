use super::{ConstructType, Detector};
use std::collections::HashMap;
use tree_sitter::Node;

pub struct PatternDetector {
    pub patterns: HashMap<String, ConstructType>,
}

impl PatternDetector {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        Self::add_typescript_patterns(&mut patterns);
        Self::add_python_patterns(&mut patterns);
        Self::add_ruby_patterns(&mut patterns);
        Self::add_go_patterns(&mut patterns);
        Self::add_rust_patterns(&mut patterns);
        Self::add_blablalang_patterns(&mut patterns);

        Self { patterns }
    }

    fn add_typescript_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("variable_declaration", ConstructType::Variable),
            ("lexical_declaration", ConstructType::Variable),
            ("function_declaration", ConstructType::Function),
            ("arrow_function", ConstructType::Function),
            ("method_definition", ConstructType::Function),
            ("function_expression", ConstructType::Function),
            ("if_statement", ConstructType::If),
            ("ternary_expression", ConstructType::Ternary),
            ("while_statement", ConstructType::While),
            ("for_statement", ConstructType::For),
            ("for_in_statement", ConstructType::For),
            ("for_of_statement", ConstructType::For),
            ("class_declaration", ConstructType::Class),
        ];

        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }

    fn add_python_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("assignment", ConstructType::Variable),
            ("function_definition", ConstructType::Function),
            ("class_definition", ConstructType::Class),
            ("conditional_expression", ConstructType::Ternary),
        ];

        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }

    fn add_ruby_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("method", ConstructType::Function),
            ("singleton_method", ConstructType::Function),
            ("if", ConstructType::If),
            ("unless", ConstructType::If),
            ("conditional", ConstructType::Ternary),
            ("while", ConstructType::While),
            ("until", ConstructType::While),
            ("for", ConstructType::For),
            ("class", ConstructType::Class),
        ];

        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }

    fn add_go_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("short_var_declaration", ConstructType::Variable),
            ("var_declaration", ConstructType::Variable),
            ("function_declaration", ConstructType::Function),
            ("method_declaration", ConstructType::Function),
            ("if_statement", ConstructType::If),
            ("for_statement", ConstructType::For),
            ("type_declaration", ConstructType::Class),
        ];

        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }

    fn add_rust_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("let_declaration", ConstructType::Variable),
            ("function_item", ConstructType::Function),
            ("if_expression", ConstructType::If),
            ("while_expression", ConstructType::While),
            ("loop_expression", ConstructType::While),
            ("for_expression", ConstructType::For),
            ("struct_item", ConstructType::Class),
            ("impl_item", ConstructType::Class),
        ];

        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }

    fn add_blablalang_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("var_declaration", ConstructType::Variable),
            ("func_declaration", ConstructType::Function),
            ("if_expr", ConstructType::If),
            ("while_expr", ConstructType::While),
            ("for_expr", ConstructType::For),
            ("class_declaration", ConstructType::Class),
        ];

        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }

    pub fn add_pattern(&mut self, pattern: String, construct: ConstructType) {
        self.patterns.insert(pattern, construct);
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> Option<ConstructType> {
        self.patterns.remove(pattern)
    }

    pub fn has_pattern(&self, pattern: &str) -> bool {
        self.patterns.contains_key(pattern)
    }

    pub fn get_patterns_for_construct(&self, construct: ConstructType) -> Vec<&String> {
        self.patterns
            .iter()
            .filter(|(_, &ct)| ct == construct)
            .map(|(pattern, _)| pattern)
            .collect()
    }
}

impl Detector for PatternDetector {
    fn detect(&self, node: &Node, _code: &str) -> Option<ConstructType> {
        let node_kind = node.kind();

        if is_string_or_comment_kind(node_kind) {
            return None;
        }

        if is_in_macro(node) {
            return None;
        }

        if node_kind != "impl_item" && is_in_impl_signature(node) {
            return None;
        }

        self.patterns.get(node_kind).copied()
    }
}

fn is_in_macro(node: &Node) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        let kind = parent.kind();
        if kind == "macro_invocation" || kind.contains("macro") {
            return true;
        }
        current = parent.parent();
    }
    false
}

fn is_in_impl_signature(node: &Node) -> bool {
    let mut current = node.parent();

    while let Some(parent) = current {
        let kind = parent.kind();

        if (kind == "impl_item" || kind == "trait_impl") && node.kind() != "impl_item" {
            if let Some(body) = parent
                .children(&mut parent.walk())
                .find(|child| child.kind() == "declaration_list" || child.kind() == "block")
            {
                if node.start_byte() < body.start_byte() {
                    return true;
                }
            } else {
                return true;
            }
        }

        current = parent.parent();
    }

    false
}

fn is_string_or_comment_kind(kind: &str) -> bool {
    kind.contains("string")
        || kind.contains("comment")
        || kind.contains("template")
        || kind == "regex"
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new()
    }
}
