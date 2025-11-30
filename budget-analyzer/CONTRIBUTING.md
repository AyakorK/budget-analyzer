# Contributing to Budget Analyzer

Thank you for your interest in contributing to Budget Analyzer! This document provides guidelines for adding new languages and features.

## Adding a New Language

### 1. Add Tree-sitter Parser

Add the language parser to `budget-analyzer/Cargo.toml`:
```toml
[dependencies]
tree-sitter-java = "0.x.x"
```

### 2. Register the Language

In `budget-analyzer/src/parsers.rs`:
```rust
pub enum SupportedLanguage {
    TypeScript,
    Python,
    Ruby,
    Go,
    Rust,
    Java,
    // Add here
}

impl SupportedLanguage {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "ts" => Some(Self::TypeScript),
            "py" => Some(Self::Python),
            "rb" => Some(Self::Ruby),
            "go" => Some(Self::Go),
            "rs" => Some(Self::Rust),
            "java" => Some(Self::Java),
            // Add here
            _ => None,
        }
    }

    pub fn create_parser(&self) -> Result<Parser> {
        let mut parser = Parser::new();
        let language = match self {
            Self::TypeScript => tree_sitter_typescript::language_typescript(),
            Self::Python => tree_sitter_python::language(),
            Self::Ruby => tree_sitter_ruby::language(),
            Self::Go => tree_sitter_go::language(),
            Self::Rust => tree_sitter_rust::language(),
            Self::Java => tree_sitter_java::language(),
            // Add here
        };
        parser.set_language(language)?;
        Ok(parser)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::TypeScript => "TypeScript",
            Self::Python => "Python",
            Self::Ruby => "Ruby",
            Self::Go => "Go",
            Self::Rust => "Rust",
            Self::Java => "Java",
            // Add here
        }
    }
}
```

### 3. Add Syntax Patterns

In `budget-analyzer/src/detection/patterns.rs`:
```rust
impl PatternDetector {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        Self::add_typescript_patterns(&mut patterns);
        Self::add_python_patterns(&mut patterns);
        Self::add_ruby_patterns(&mut patterns);
        Self::add_go_patterns(&mut patterns);
        Self::add_rust_patterns(&mut patterns);
        Self::add_java_patterns(&mut patterns);

        // Add here

        Self { patterns }
    }

    // Add this function adapted to your language
    fn add_java_patterns(patterns: &mut HashMap<String, ConstructType>) {
        let mappings = [
            ("local_variable_declaration", ConstructType::Variable),
            ("method_declaration", ConstructType::Function),
            ("if_statement", ConstructType::If),
            ("while_statement", ConstructType::While),
            ("for_statement", ConstructType::For),
            ("enhanced_for_statement", ConstructType::For),
            ("class_declaration", ConstructType::Class),
        ];
        
        for (pattern, construct) in mappings {
            patterns.insert(pattern.to_string(), construct);
        }
    }
}
```

### 4. Add Quality Patterns

In `budget-analyzer/src/detection/quality.rs`:
```rust
lazy_static::lazy_static! {
    static ref QUALITY_CONFIGS: HashMap<&'static str, LanguageQualityConfig> = {
        let mut map = HashMap::new();
        
        // ... existing languages
        
        // Add Java
        map.insert("Java", LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "method_invocation",
                        field: "name",
                        keywords: vec!["println", "print"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "method_invocation",
                        field: "object",
                        keywords: vec!["logger.error", "LOG.error"],
                    },
                ],
            },
            comment_node: "comment",
            try_node: Some("try_statement"),
            doc_comment_prefix: "/**",
        });
        
        map
    };
}
```

### 5. Enable in VSCode Extension

In `vscode-budget/src/extension.ts`:
```typescript
const clientOptions: LanguageClientOptions = {
    documentSelector: [
        { scheme: 'file', language: 'typescript' },
        { scheme: 'file', language: 'javascript' },
        { scheme: 'file', language: 'python' },
        { scheme: 'file', language: 'ruby' },
        { scheme: 'file', language: 'go' },
        { scheme: 'file', language: 'rust' },
        { scheme: 'file', language: 'java' },  // Add here
    ],
};
```

### 6. Build and Test
```bash
# Build analyzer
cd budget-analyzer
cargo build --release

# Test on sample file
cargo run -- analyze test.java

# Build LSP
cd ../budget-lsp
cargo build --release

# Build extension
cd ../vscode-budget
npm run compile

# Test in VSCode (Press F5)
```

## Adding Custom Bonus/Malus

Edit `.budgetrc.json` in your project:
```json
{
  "bonuses": {
    "custom_pattern": -5
  },
  "maluses": {
    "bad_practice": 10
  }
}
```

## Code Style

- Use meaningful variable names
- Keep functions focused and small
- Add comments only when necessary
- Follow Rust conventions for Rust code
- Follow TypeScript conventions for extension code

## Testing

Before submitting a PR:

1. Run tests: `cargo test`
2. Check formatting: `cargo fmt --check`
3. Run clippy: `cargo clippy`
4. Test on sample files for your language
5. Verify LSP integration in VSCode

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-language`)
3. Commit your changes (`git commit -m 'Add support for Java'`)
4. Push to the branch (`git push origin feature/new-language`)
5. Open a Pull Request with a clear description

## Questions?

Open an issue or reach out to the maintainers.