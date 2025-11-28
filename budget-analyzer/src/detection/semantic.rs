use super::{ConstructType, Detector};
use tree_sitter::Node;

pub struct SemanticDetector;

impl SemanticDetector {
    pub fn new() -> Self {
        Self
    }

    fn is_variable(&self, node: &Node, code: &str) -> bool {
        // Go-specific: short variable declaration
        if node.kind() == "short_var_declaration" {
            return true;
        }

        self.has_identifier(node)
            && self.has_assignment_operator(node, code)
            && self.is_statement_level(node)
    }

    fn is_function(&self, node: &Node, code: &str) -> bool {
        let kind = node.kind();

        // Check node kind first (most reliable)
        if kind.contains("function")
            || kind.contains("method")
            || kind == "function_definition"
            || kind == "function_declaration" {
            return true;
        }

        // Structural check
        let has_structure = self.has_identifier(node)
            && self.has_parameters(node)
            && self.has_body_block(node);

        // Keyword check
        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
        let has_keyword = text.trim_start().starts_with("def ")
            || text.trim_start().starts_with("function ")
            || text.trim_start().starts_with("func ")
            || text.trim_start().starts_with("fn ");

        has_structure || has_keyword
    }

    fn is_if_statement(&self, node: &Node, code: &str) -> bool {
        let has_structure = self.has_condition(node) && self.has_consequent(node);

        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
        let has_keyword = text.trim_start().starts_with("if");

        has_structure || has_keyword
    }

    fn is_while_loop(&self, node: &Node, code: &str) -> bool {
        let has_structure = self.has_condition(node) && self.has_body_block(node);

        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
        let has_keyword = text.trim_start().starts_with("while")
            || text.trim_start().starts_with("until");

        has_structure && has_keyword
    }

    fn is_for_loop(&self, node: &Node, code: &str) -> bool {
        let has_structure = self.has_iterator(node) && self.has_body_block(node);

        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
        let has_keyword = text.trim_start().starts_with("for");

        has_structure || has_keyword
    }

    fn is_class(&self, node: &Node, code: &str) -> bool {
        let kind = node.kind();

        // Check node kind first
        if kind == "class_declaration"
            || kind == "class_definition"
            || kind == "class" {
            return true;
        }

        // Structural check: must have name AND multiple methods
        if self.has_identifier(node) && self.has_methods_or_properties(node) {
            if self.count_methods(node) >= 2 {
                return true;
            }
        }

        // Keyword check (last resort, very strict)
        if self.is_statement_level(node) {
            let text = node.utf8_text(code.as_bytes()).unwrap_or("");
            let parts: Vec<&str> = text.trim_start().split_whitespace().collect();

            if parts.first() == Some(&"class") && parts.get(1).is_some() {
                return true;
            }
        }

        false
    }

    fn is_ternary(&self, node: &Node, code: &str) -> bool {
        let text = node.utf8_text(code.as_bytes()).unwrap_or("");

        // JavaScript/TypeScript: condition ? true : false
        let has_ternary_op = text.contains('?') && text.contains(':');

        // Python: true if condition else false (single line)
        let has_python_ternary = text.contains(" if ")
            && text.contains(" else ")
            && !text.contains('\n');

        has_ternary_op || has_python_ternary
    }

    // === HELPERS ===

    fn has_identifier(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("identifier") || kind.contains("name")
            })
    }

    fn has_assignment_operator(&self, node: &Node, code: &str) -> bool {
        // Check if node itself is a short_var_declaration
        if node.kind() == "short_var_declaration" {
            return true;
        }

        node.children(&mut node.walk())
            .any(|child| {
                let text = child.utf8_text(code.as_bytes()).unwrap_or("");
                let kind = child.kind();

                text == "=" || text == ":=" || text == "<-" || text == "←"
                    || kind == ":=" || kind == "short_var_declaration"
            })
    }

    fn has_parameters(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("parameter") || kind.contains("argument") || kind == "("
            })
    }

    fn has_body_block(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("block") || kind.contains("body") || kind == "{"
            })
    }

    fn has_condition(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("condition") || kind.contains("test")
            })
    }

    fn has_consequent(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("consequent") || kind.contains("then") || kind.contains("block")
            })
    }

    fn has_iterator(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("iterator") || kind.contains("variable") || kind.contains("range")
            })
    }

    fn has_methods_or_properties(&self, node: &Node) -> bool {
        node.children(&mut node.walk())
            .any(|child| {
                let kind = child.kind();
                kind.contains("method") || kind.contains("property") || kind.contains("field")
            })
    }

    fn count_methods(&self, node: &Node) -> usize {
        node.children(&mut node.walk())
            .filter(|child| {
                let kind = child.kind();
                kind.contains("method") || kind.contains("function") || kind == "function_definition"
            })
            .count()
    }

    fn is_statement_level(&self, node: &Node) -> bool {
        node.parent()
            .map(|parent| {
                let kind = parent.kind();
                kind.contains("program")
                    || kind.contains("block")
                    || kind.contains("body")
                    || kind == "source_file"
            })
            .unwrap_or(false)
    }
}

impl Detector for SemanticDetector {
    fn detect(&self, node: &Node, code: &str) -> Option<ConstructType> {
        // Order matters: most specific first
        if self.is_ternary(node, code) {
            return Some(ConstructType::Ternary);
        }

        if self.is_class(node, code) {
            return Some(ConstructType::Class);
        }

        if self.is_function(node, code) {
            return Some(ConstructType::Function);
        }

        if self.is_for_loop(node, code) {
            return Some(ConstructType::For);
        }

        if self.is_while_loop(node, code) {
            return Some(ConstructType::While);
        }

        if self.is_if_statement(node, code) {
            return Some(ConstructType::If);
        }

        if self.is_variable(node, code) {
            return Some(ConstructType::Variable);
        }

        None
    }
}
