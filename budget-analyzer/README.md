# 🎯 Budget Analyzer - Universal Code Complexity Analyzer

> **Semantic detection** for budget analysis across **any programming language** - optional configuration if you want to make your own rules !

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)]()

## ✨ Features

-  **6 Languages Supported** - TypeScript, JavaScript, Python, Ruby, Go, Rust
-  **Semantic Detection** - No manual mapping per language
-  **Zero Configuration** - Works out of the box
-  **Blazing Fast** - Built in Rust with Tree-sitter
-  **Smart Profiling** - Auto-detects strict/class/service/test
-  **Beautiful CLI** - Colored output with detailed breakdowns
-  **LSP Support** - Real-time budget analysis in VSCode
-  **Fully Customizable** - Profiles, rules, bonuses, maluses

---

## Quick Start

### Installation
```bash
cd budget-analyzer
cargo build --release
cargo install --path .
```

### Basic Usage
```bash
# Analyze a single file
budget analyze src/fibonacci.ts

# Analyze entire directory
budget analyze src/

# Check budget (CI/CD friendly - exits with code 1 if exceeded)
budget check src/

# Show current configuration
budget config

# JSON output
budget analyze src/ --json
```

---

## Example Output
```
Budget Analysis Results

✅ src/utils.ts [TypeScript] (strict) 45/80 pts (56.3%)
✅ src/Point.py [Python] (class) 230/300 pts (76.7%)
❌ src/complex.rb [Ruby] (service) 280/250 pts (112.0%)

Summary: 2 OK, 1 EXCEEDED
```

---

## How It Works

### Semantic Detection

Instead of hardcoding language-specific patterns, we use **semantic analysis**:
```rust
fn is_function(node: &Node, code: &str) -> bool {
    has_identifier(node) &&
    has_parameters(node) &&
    has_body_block(node)
}
```

**This works for:**
- TypeScript: `function fib(n: number) { }`
- Python: `def fib(n):`
- Ruby: `def fib(n); end`
- Go: `func fib(n int) int { }`
- Rust: `fn fib(n: i32) -> i32 { }`

**And many more!**

---

## Default Budget Rules

| Construct | Cost          | Description                    |
|-----------|---------------|--------------------------------|
| Variable  | 2 pts         | Variable declaration           |
| If        | 5 pts         | Conditional statement          |
| For       | 10 pts        | For loop                       |
| While     | 15 pts        | While loop                     |
| Function  | 8 pts         | Function/method definition     |
| Class     | 15 pts        | Class definition               |
| Ternary   | **-5 pts**   | Ternary operator (bonus!)      |

*All values are customizable via `.budgetrc.json`*

---

## Profiles

Profiles are **auto-detected** based on filename and code structure:

| Profile | Budget | Auto-Detection Rules                           |
|---------|--------|------------------------------------------------|
| `strict`  | 80 pts | Utils, helpers, simple functions              |
| `class`   | 300 pts| Files with class definitions                  |
| `service` | 250 pts| Business logic with multiple functions        |
| `test`    | 500 pts| Files containing `.test.` or `.spec.`         |

---

## Configuration

### Create `.budgetrc.json`:

You can check the .budgetrc.example.json for reference

### Configuration Priority

1. **Explicit file config** (`files`)
2. **File patterns** (`filePatterns`)
3. **Auto-detection** (if `autoDetect: true`)
4. **Default** (`strict` profile)

---

## Supported Languages

| Language   | Status | Notes                    |
|------------|--------|--------------------------|
| TypeScript | ✅     | Full support             |
| JavaScript | ✅     | Full support             |
| Python     | ✅     | Full support             |
| Ruby       | ✅     | Full support             |
| Go         | ✅     | Full support             |
| Rust       | ✅     | Full support             |
| BlablaLang | 🔜     | Coming soon              |
| Java       | 🔜     | Coming soon              |
| C++        | 🔜     | Coming soon              |
| C#         | 🔜     | Coming soon              |

---

## Testing
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test config_test
cargo test integration_test

# Run with output
cargo test -- --nocapture

```

---
## Architecture
```
budget-analyzer/
├── src/
│   ├── detection/
│   │   ├── semantic.rs       # Semantic detection engine
│   │   ├── patterns.rs       # Fallback pattern matching
│   │   └── hybrid.rs         # Combines both approaches
│   ├── parsers/
│   │   └── mod.rs            # Tree-sitter language integration
│   ├── analyzer.rs           # Main analysis logic
│   ├── config.rs             # Configuration management
│   ├── profiles.rs           # Profile detection
│   └── main.rs               # CLI
├── tests/
│   ├── fixtures/             # Multi-language test files
│   ├── integration_test.rs   # Language tests
│   ├── config_test.rs        # Config tests
│   ├── profile_test.rs       # Profile tests
│   └── bonus_test.rs         # Bonus/malus tests
└── .budgetrc.example.json    # Example configuration
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed instructions on:
- Adding a new language
- Improving semantic detection
- Adding new bonus/malus rules
- Testing guidelines

---

##  LSP Support (VSCode)

Real-time budget analysis in your editor!

**Features:**
- 🟢 Inline hints showing budget at top of file
- 💡 Hover tooltips with detailed breakdown
- ⚠️ Warnings for high-cost constructs
- ❌ Errors when budget is exceeded

See [vscode-budget/README.md](../vscode-budget/README.md) for setup.

---

## 📝 Why This Approach?

### Traditional Approaches

- Manual mapping for each language (100+ files to maintain)
- External tools required (Node.js, Python interpreters)
- Slow parsing and analysis
- Hard to maintain and extend

### Our Approach

- **Semantic detection** - One implementation for all languages
- **Tree-sitter** - Fast, incremental parsing
- **Rust** - Native performance, single binary
- **Extensible** - Add new languages in minutes

---

## Acknowledgments

- [Tree-sitter](https://tree-sitter.github.io/) for universal parsing

---

## Links

- [Contributing Guide](CONTRIBUTING.md)
- [Examples](.budgetrc.example.json)
- [LSP Extension](../vscode-budget/)

---

**Made with ❤️ by the BlaBlaLang Team**