use super::{ConstructType, Detector};
use std::collections::HashMap;
use tree_sitter::Node;

pub struct PatternDetector {
    patterns: HashMap<String, ConstructType>,
}

impl PatternDetector {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // TypeScript/JavaScript
        patterns.insert("variable_declaration".to_string(), ConstructType::Variable);
        patterns.insert("lexical_declaration".to_string(), ConstructType::Variable);
        patterns.insert("function_declaration".to_string(), ConstructType::Function);
        patterns.insert("arrow_function".to_string(), ConstructType::Function);
        patterns.insert("method_definition".to_string(), ConstructType::Function);
        patterns.insert("if_statement".to_string(), ConstructType::If);
        patterns.insert("while_statement".to_string(), ConstructType::While);
        patterns.insert("for_statement".to_string(), ConstructType::For);
        patterns.insert("for_in_statement".to_string(), ConstructType::For);
        patterns.insert("class_declaration".to_string(), ConstructType::Class);
        patterns.insert("ternary_expression".to_string(), ConstructType::Ternary);

        // Python
        patterns.insert("assignment".to_string(), ConstructType::Variable);
        patterns.insert("function_definition".to_string(), ConstructType::Function);
        patterns.insert("class_definition".to_string(), ConstructType::Class);
        patterns.insert("conditional_expression".to_string(), ConstructType::Ternary);

        // Ruby
        patterns.insert("method".to_string(), ConstructType::Function);
        patterns.insert("singleton_method".to_string(), ConstructType::Function);
        patterns.insert("if".to_string(), ConstructType::If);
        patterns.insert("unless".to_string(), ConstructType::If);
        patterns.insert("while".to_string(), ConstructType::While);
        patterns.insert("until".to_string(), ConstructType::While);
        patterns.insert("for".to_string(), ConstructType::For);
        patterns.insert("class".to_string(), ConstructType::Class);
        patterns.insert("conditional".to_string(), ConstructType::Ternary);

        Self { patterns }
    }

    pub fn add_pattern(&mut self, pattern: String, construct: ConstructType) {
        self.patterns.insert(pattern, construct);
    }
}

impl Detector for PatternDetector {
    fn detect(&self, node: &Node, _code: &str) -> Option<ConstructType> {
        self.patterns.get(node.kind()).copied()
    }
}
