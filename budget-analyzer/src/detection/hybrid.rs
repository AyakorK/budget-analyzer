use super::patterns::PatternDetector;
use super::semantic::SemanticDetector;
use super::{ConstructType, Detector};
use tree_sitter::Node;

/// Hybrid detector that combines semantic and pattern-based detection.
///
/// # Detection Strategy
///
/// 1. **Semantic Detection (Priority)**: Analyzes code structure and context
///    - Filters out strings, comments, templates
///    - Verifies keyword context and delimiters
///    - Checks structural requirements (params, body, etc.)
///
/// 2. **Pattern Detection (Fallback)**: Matches AST node types
///    - Simple mapping from tree-sitter node kinds
///    - Fast and reliable for well-formed code
///    - Language-specific patterns
///
/// This two-phase approach provides:
/// - High accuracy (semantic filtering)
/// - Good coverage (pattern fallback)
/// - Minimal false positives
pub struct HybridDetector {
    semantic: SemanticDetector,
    pattern: PatternDetector,
}

impl HybridDetector {
    /// Creates a new hybrid detector with default semantic and pattern detectors
    pub fn new() -> Self {
        Self {
            semantic: SemanticDetector::new(),
            pattern: PatternDetector::new(),
        }
    }

    /// Creates a hybrid detector with a custom pattern detector
    ///
    /// Useful for adding language-specific patterns or customizing detection rules
    pub fn with_pattern_detector(pattern: PatternDetector) -> Self {
        Self {
            semantic: SemanticDetector::new(),
            pattern,
        }
    }

    /// Gets a reference to the semantic detector
    ///
    /// Useful for testing or introspection
    pub fn semantic(&self) -> &SemanticDetector {
        &self.semantic
    }

    /// Gets a reference to the pattern detector
    ///
    /// Useful for testing or adding custom patterns
    pub fn pattern(&self) -> &PatternDetector {
        &self.pattern
    }

    /// Gets a mutable reference to the pattern detector
    ///
    /// Allows runtime modification of pattern rules
    pub fn pattern_mut(&mut self) -> &mut PatternDetector {
        &mut self.pattern
    }
}

impl Detector for HybridDetector {
    /// Detects programming constructs using hybrid approach
    ///
    /// # Algorithm
    ///
    /// 1. Try semantic detection first (more accurate)
    /// 2. Fall back to pattern detection if semantic fails
    /// 3. Return None if both fail
    ///
    /// # Example
    ///
    /// ```ignore
    /// let detector = HybridDetector::new();
    /// let construct = detector.detect(&node, source_code);
    ///
    /// match construct {
    ///     Some(ConstructType::Function) => println!("Found function"),
    ///     Some(ConstructType::Class) => println!("Found class"),
    ///     None => println!("No construct detected"),
    ///     _ => {}
    /// }
    /// ```
    fn detect(&self, node: &Node, code: &str) -> Option<ConstructType> {
        // Phase 1: Semantic detection (context-aware, filters false positives)
        if let Some(construct) = self.semantic.detect(node, code) {
            return Some(construct);
        }

        // Phase 2: Pattern detection (fast fallback for well-formed code)
        self.pattern.detect(node, code)
    }
}

impl Default for HybridDetector {
    fn default() -> Self {
        Self::new()
    }
}
