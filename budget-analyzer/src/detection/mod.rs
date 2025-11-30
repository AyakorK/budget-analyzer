pub mod hybrid;
pub mod patterns;
pub mod quality;
pub mod semantic;

pub use hybrid::HybridDetector;
pub use patterns::PatternDetector;
pub use quality::{detect_quality_patterns, BudgetItem};
pub use semantic::SemanticDetector;
use tree_sitter::Node;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructType {
    Variable,
    Function,
    If,
    While,
    For,
    Class,
    Ternary,
}

impl ConstructType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Variable => "variable",
            Self::Function => "function",
            Self::If => "if",
            Self::While => "while",
            Self::For => "for",
            Self::Class => "class",
            Self::Ternary => "ternary",
        }
    }
}

pub trait Detector {
    fn detect(&self, node: &Node, code: &str) -> Option<ConstructType>;
}
