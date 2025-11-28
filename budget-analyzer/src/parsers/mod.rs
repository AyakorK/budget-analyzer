use anyhow::{anyhow, Result};
use std::path::Path;
use tree_sitter::{Language, Parser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedLanguage {
    TypeScript,
    JavaScript,
    Python,
    Ruby,
    Go,
    Rust,
}

impl SupportedLanguage {
    pub fn from_extension(ext: &str) -> Result<Self> {
        match ext {
            "ts" | "tsx" => Ok(Self::TypeScript),
            "js" | "jsx" => Ok(Self::JavaScript),
            "py" => Ok(Self::Python),
            "rb" => Ok(Self::Ruby),
            "go" => Ok(Self::Go),
            "rs" => Ok(Self::Rust),
            _ => Err(anyhow!("Unsupported file extension: .{}", ext)),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("No file extension found"))?;
        Self::from_extension(ext)
    }

    pub fn language(&self) -> Result<Language> {
        let lang = match self {
            Self::TypeScript => tree_sitter_typescript::language_typescript(),
            Self::JavaScript => tree_sitter_javascript::language(),
            Self::Python => tree_sitter_python::language(),
            Self::Ruby => tree_sitter_ruby::language(),
            Self::Go => tree_sitter_go::language(),
            Self::Rust => tree_sitter_rust::language(),
        };
        Ok(lang)
    }

    pub fn create_parser(&self) -> Result<Parser> {
        let mut parser = Parser::new();
        let lang = self.language()?;
        parser.set_language(&lang)?;
        Ok(parser)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Python => "Python",
            Self::Ruby => "Ruby",
            Self::Go => "Go",
            Self::Rust => "Rust",
        }
    }
}