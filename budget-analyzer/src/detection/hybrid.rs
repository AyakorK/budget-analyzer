use super::{ConstructType, Detector};
use super::semantic::SemanticDetector;
use super::patterns::PatternDetector;
use tree_sitter::Node;

// Tries semantic detection first,
// then falls back to pattern-based detection.
pub struct HybridDetector {
    semantic: SemanticDetector,
    pattern: PatternDetector,
}

impl HybridDetector {
    pub fn new() -> Self {
        Self {
            semantic: SemanticDetector::new(),
            pattern: PatternDetector::new(),
        }
    }
}

impl Detector for HybridDetector {
    fn detect(&self, node: &Node, code: &str) -> Option<ConstructType> {
        if let Some(construct) = self.semantic.detect(node, code) {
            return Some(construct);
        }

        self.pattern.detect(node, code)
    }
}
