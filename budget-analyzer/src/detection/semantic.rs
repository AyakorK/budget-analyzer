use super::{ConstructType, Detector};
use tree_sitter::Node;

/// Semantic detector that analyzes code structure and context
/// to identify programming constructs while filtering out false positives
/// from strings, comments, and templates.
pub struct SemanticDetector;

impl SemanticDetector {
    pub fn new() -> Self {
        Self
    }

    // ========================================================================
    // STRING AND COMMENT FILTERING
    // ========================================================================

    /// Checks if a node is inside a string, comment, or template literal
    /// This is crucial to avoid false positives
    fn is_in_string_or_comment(&self, node: &Node) -> bool {
        let kind = node.kind();

        if self.is_string_or_comment_kind(kind) {
            return true;
        }

        if self.is_in_macro_invocation(node) {
            return true;
        }

        let mut current = node.parent();
        while let Some(parent) = current {
            if self.is_string_or_comment_kind(parent.kind()) {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    /// Helper to check if a kind represents a string or comment
    #[inline]
    fn is_string_or_comment_kind(&self, kind: &str) -> bool {
        kind.contains("string")
            || kind.contains("comment")
            || kind.contains("template")
            || matches!(
                kind,
                "string_literal"
                    | "template_string"
                    | "raw_string_literal"
                    | "interpreted_string_literal"
            )
    }

    // ========================================================================
    // KEYWORD DETECTION WITH CONTEXT
    // ========================================================================

    /// Checks if a keyword appears in the correct syntactic context
    /// (not in strings/comments and properly delimited)
    fn is_keyword_in_context(&self, node: &Node, code: &str, keyword: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let text = match node.utf8_text(code.as_bytes()) {
            Ok(t) => t.trim(),
            Err(_) => return false,
        };

        if !text.starts_with(keyword) {
            return false;
        }

        // Check that keyword is properly delimited
        match text.strip_prefix(keyword) {
            Some(rest) if rest.is_empty() => true,
            Some(rest) => {
                let next_char = rest.chars().next();
                matches!(next_char, Some(' ' | '\t' | '\n' | '(' | '{'))
            }
            None => false,
        }
    }

    // ========================================================================
    // CONSTRUCT DETECTORS
    // ========================================================================

    /// Detects variable declarations and assignments
    fn is_variable(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        // Go short variable declaration
        if node.kind() == "short_var_declaration" {
            return true;
        }

        // Standard variable detection
        self.has_identifier(node)
            && self.has_assignment_operator(node, code)
            && self.is_statement_level(node)
    }

    /// Detects function declarations and definitions
    fn is_function(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let kind = node.kind();

        // Direct function node types
        if kind.contains("function")
            || kind.contains("method")
            || matches!(kind, "function_definition" | "function_declaration")
        {
            return true;
        }

        // Structural detection (has name, params, and body)
        let has_structure =
            self.has_identifier(node) && self.has_parameters(node) && self.has_body_block(node);

        // Keyword-based detection
        let has_keyword = self.is_keyword_in_context(node, code, "def")
            || self.is_keyword_in_context(node, code, "function")
            || self.is_keyword_in_context(node, code, "func")
            || self.is_keyword_in_context(node, code, "fn");

        has_structure || has_keyword
    }

    /// Detects if statements and conditionals
    fn is_if_statement(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let kind = node.kind();

        // Direct if node types
        if matches!(kind, "if_statement" | "if_expression") {
            return true;
        }

        // Structural + keyword detection
        let has_structure = self.has_condition(node) && self.has_consequent(node);
        let has_keyword = self.is_keyword_in_context(node, code, "if");

        has_structure || has_keyword
    }

    /// Detects while loops
    fn is_while_loop(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let kind = node.kind();

        // Direct while node types
        if matches!(kind, "while_statement" | "while_expression") {
            return true;
        }

        // Structural + keyword detection
        let has_structure = self.has_condition(node) && self.has_body_block(node);
        let has_keyword = self.is_keyword_in_context(node, code, "while")
            || self.is_keyword_in_context(node, code, "until");

        has_structure && has_keyword
    }

    /// Detects for loops (including for-in, for-of)
    fn is_for_loop(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let kind = node.kind();

        // Direct for node types (most reliable)
        if matches!(
            kind,
            "for_statement" | "for_in_statement" | "for_expression"
        ) {
            return true;
        }

        // CRITICAL: Avoid false positives in macros like format!()
        // Check if we're inside a macro invocation
        if self.is_in_macro_invocation(node) {
            return false;
        }

        // Additional check: if node text contains "format!" or other macros, skip
        if let Ok(text) = node.utf8_text(code.as_bytes()) {
            if text.contains("format!") || text.contains("println!") || text.contains("vec!") {
                return false;
            }
        }

        // Structural + keyword detection
        let has_structure = self.has_iterator(node) && self.has_body_block(node);
        let has_keyword = self.is_keyword_in_context(node, code, "for");

        // MUST have BOTH structure AND keyword to avoid false positives
        has_structure && has_keyword
    }

    /// Detects class declarations and definitions
    fn is_class(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let kind = node.kind();

        // Direct class node types
        if matches!(kind, "class_declaration" | "class_definition" | "class") {
            return true;
        }

        // Complex structural detection for classes
        if self.has_identifier(node) && self.has_methods_or_properties(node) {
            // A class should have at least 2 methods to be considered a real class
            if self.count_methods(node) >= 2 {
                return true;
            }
        }

        // Statement-level class keyword detection
        if self.is_statement_level(node) {
            let has_keyword = self.is_keyword_in_context(node, code, "class");
            if has_keyword && self.has_identifier(node) {
                return true;
            }
        }

        false
    }

    /// Detects ternary operators (?: or Python's if-else)
    fn is_ternary(&self, node: &Node, code: &str) -> bool {
        if self.is_in_string_or_comment(node) {
            return false;
        }

        let kind = node.kind();

        // Direct ternary node types
        if matches!(
            kind,
            "ternary_expression" | "conditional_expression" | "ternary"
        ) {
            return true;
        }

        // Pattern-based detection for expressions
        if kind.contains("expression") || kind.contains("statement") {
            let text = match node.utf8_text(code.as_bytes()) {
                Ok(t) => t.trim(),
                Err(_) => return false,
            };

            // Avoid false positives on multi-line code or very long expressions
            if text.len() > 200 || text.contains('\n') {
                return false;
            }

            // JavaScript/TypeScript style: condition ? true : false
            let has_js_ternary = text.contains('?') && text.contains(':');

            // Python style: value if condition else other_value
            let has_python_ternary = text.contains(" if ") && text.contains(" else ");

            return has_js_ternary || has_python_ternary;
        }

        false
    }

    // ========================================================================
    // STRUCTURAL HELPERS
    // ========================================================================

    /// Checks if node has an identifier child
    fn has_identifier(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("identifier") || kind.contains("name")
        })
    }

    /// Checks if node has an assignment operator
    fn has_assignment_operator(&self, node: &Node, code: &str) -> bool {
        // Go short variable declaration
        if node.kind() == "short_var_declaration" {
            return true;
        }

        node.children(&mut node.walk()).any(|child| {
            let text = child.utf8_text(code.as_bytes()).unwrap_or("");
            let kind = child.kind();

            text == "="
                || text == ":="
                || text == "<-"
                || text == "←"
                || kind == ":="
                || kind == "short_var_declaration"
        })
    }

    /// Checks if node has function parameters
    fn has_parameters(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("parameter") || kind.contains("argument") || kind == "("
        })
    }

    /// Checks if node has a body block
    fn has_body_block(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("block") || kind.contains("body") || kind == "{"
        })
    }

    /// Checks if node has a condition
    fn has_condition(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("condition") || kind.contains("test")
        })
    }

    /// Checks if node has a consequent (then branch)
    fn has_consequent(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("consequent") || kind.contains("then") || kind.contains("block")
        })
    }

    /// Checks if node has an iterator (for loop variable)
    fn has_iterator(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("iterator") || kind.contains("variable") || kind.contains("range")
        })
    }

    /// Checks if node has methods or properties (class members)
    fn has_methods_or_properties(&self, node: &Node) -> bool {
        node.children(&mut node.walk()).any(|child| {
            let kind = child.kind();
            kind.contains("method") || kind.contains("property") || kind.contains("field")
        })
    }

    /// Counts methods in a node (for class detection)
    fn count_methods(&self, node: &Node) -> usize {
        node.children(&mut node.walk())
            .filter(|child| {
                let kind = child.kind();
                kind.contains("method")
                    || kind.contains("function")
                    || kind == "function_definition"
            })
            .count()
    }

    /// Checks if node is at statement level (not nested in expression)
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

    /// Checks if node is inside a macro invocation (Rust-specific)
    /// This prevents false positives like detecting "for" in format!()
    fn is_in_macro_invocation(&self, node: &Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            let kind = parent.kind();
            // Rust macro invocations
            if kind == "macro_invocation" || kind.contains("macro") {
                return true;
            }
            current = parent.parent();
        }
        false
    }
}

// ========================================================================
// DETECTOR TRAIT IMPLEMENTATION
// ========================================================================

impl Detector for SemanticDetector {
    fn detect(&self, node: &Node, code: &str) -> Option<ConstructType> {
        // Priority order matters for accurate detection

        // 1. Ternary (highest priority to avoid confusion with conditionals)
        if self.is_ternary(node, code) {
            return Some(ConstructType::Ternary);
        }

        // 2. Class (before function to avoid detecting class methods as standalone functions)
        if self.is_class(node, code) {
            return Some(ConstructType::Class);
        }

        // 3. Function
        if self.is_function(node, code) {
            return Some(ConstructType::Function);
        }

        // 4. Loops
        if self.is_for_loop(node, code) {
            return Some(ConstructType::For);
        }

        if self.is_while_loop(node, code) {
            return Some(ConstructType::While);
        }

        // 5. Conditionals
        if self.is_if_statement(node, code) {
            return Some(ConstructType::If);
        }

        // 6. Variables (lowest priority)
        if self.is_variable(node, code) {
            return Some(ConstructType::Variable);
        }

        None
    }
}

impl Default for SemanticDetector {
    fn default() -> Self {
        Self::new()
    }
}
