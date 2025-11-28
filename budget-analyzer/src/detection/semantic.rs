use super::{ConstructType, Detector};
use tree_sitter::Node;

pub struct SemanticDetector;

impl SemanticDetector {
    pub fn new() -> Self {
        Self
    }
    fn is_in_string_or_comment(&self, node: &Node) -> bool {
        let kind = node.kind();

        if kind.contains("string")
            || kind.contains("comment")
            || kind.contains("template")
            || kind == "string_literal"
            || kind == "template_string"
            || kind == "raw_string_literal"
            || kind == "interpreted_string_literal" {
            return true;
        }

        let mut current = node.parent();
        while let Some(parent) = current {
            let parent_kind = parent.kind();
            if parent_kind.contains("string")
                || parent_kind.contains("comment")
                || parent_kind.contains("template")
                || parent_kind == "string_literal"
                || parent_kind == "template_string" {
                return true;
            }
            current = parent.parent();
        }

            false
        }

    fn is_keyword_in_context(&self, node: &Node, code: &str, keyword: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
        let trimmed = text.trim();

        if !trimmed.starts_with(keyword) {
            return false;
        }

        if let Some(rest) = trimmed.strip_prefix(keyword) {
            if rest.is_empty() {
                return true;
            }

            let next_char = rest.chars().next();
            matches!(next_char, Some(' ' | '\t' | '\n' | '(' | '{'))
        } else {
            false
        }
    }


    fn is_variable(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
    return false;
    }

    if node.kind() == "short_var_declaration" {
    return true;
    }

    self.has_identifier(node)
    && self.has_assignment_operator(node, code)
    && self.is_statement_level(node)
}

fn is_function(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
        return false;
    }

    let kind = node.kind();

    if kind.contains("function")
        || kind.contains("method")
        || kind == "function_definition"
        || kind == "function_declaration" {
        return true;
    }

    let has_structure = self.has_identifier(node)
        && self.has_parameters(node)
        && self.has_body_block(node);

    let has_keyword = self.is_keyword_in_context(node, code, "def")
        || self.is_keyword_in_context(node, code, "function")
        || self.is_keyword_in_context(node, code, "func")
        || self.is_keyword_in_context(node, code, "fn");

    has_structure || has_keyword
}

fn is_if_statement(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
        return false;
    }

    let kind = node.kind();

    if kind == "if_statement" || kind == "if_expression" {
        return true;
    }

    let has_structure = self.has_condition(node) && self.has_consequent(node);
    let has_keyword = self.is_keyword_in_context(node, code, "if");

    has_structure || has_keyword
}

fn is_while_loop(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
        return false;
    }

    let kind = node.kind();

    if kind == "while_statement" || kind == "while_expression" {
        return true;
    }

    let has_structure = self.has_condition(node) && self.has_body_block(node);
    let has_keyword = self.is_keyword_in_context(node, code, "while")
        || self.is_keyword_in_context(node, code, "until");

    has_structure && has_keyword
}

fn is_for_loop(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
        return false;
    }

    let kind = node.kind();

    if kind == "for_statement"
        || kind == "for_in_statement"
        || kind == "for_expression" {
        return true;
    }

    let has_structure = self.has_iterator(node) && self.has_body_block(node);
    let has_keyword = self.is_keyword_in_context(node, code, "for");

    has_structure || has_keyword
}

fn is_class(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
        return false;
    }

    let kind = node.kind();

    if kind == "class_declaration"
        || kind == "class_definition"
        || kind == "class" {
        return true;
    }

    if self.has_identifier(node) && self.has_methods_or_properties(node) {
        if self.count_methods(node) >= 2 {
            return true;
        }
    }

    if self.is_statement_level(node) {
        let has_keyword = self.is_keyword_in_context(node, code, "class");
        if has_keyword && self.has_identifier(node) {
            return true;
        }
    }

    false
}

fn is_ternary(&self, node: &Node, code: &str) -> bool {
    if self.is_in_string_or_comment(node) {
        return false;
    }

    let kind = node.kind();

    if kind == "ternary_expression"
        || kind == "conditional_expression"
        || kind == "ternary" {
        return true;
    }

    if kind.contains("expression") || kind.contains("statement") {
        let text = node.utf8_text(code.as_bytes()).unwrap_or("");
        let trimmed = text.trim();

        if trimmed.len() > 200 || trimmed.contains('\n') {
            return false;
        }

        let has_question = trimmed.contains('?');
        let has_colon = trimmed.contains(':');

        let has_python_style = trimmed.contains(" if ") && trimmed.contains(" else ");

        return (has_question && has_colon) || has_python_style;
    }

    false
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
