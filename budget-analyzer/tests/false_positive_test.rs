use budget_analyzer::Analyzer;
use std::fs;
use std::path::Path;

// ============================================================================
// EDGE CASES - STRINGS & COMMENTS
// ============================================================================

#[test]
fn test_no_false_positive_in_strings() {
    let analyzer = Analyzer::default();

    let code = r#"
const message = "for loop example";
const info = "if statement here";
const text = "while processing";
const log = "class definition";
console.log("function call");
"#;

    fs::write("temp_strings.ts", code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_strings.ts")).unwrap();

    // Should NOT detect keywords inside strings
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "for"),
        "Should NOT detect 'for' in string"
    );
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "if"),
        "Should NOT detect 'if' in string"
    );
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "while"),
        "Should NOT detect 'while' in string"
    );
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "class"),
        "Should NOT detect 'class' in string"
    );

    fs::remove_file("temp_strings.ts").ok();
}

#[test]
fn test_no_false_positive_in_comments() {
    let analyzer = Analyzer::default();

    let code = r#"
// This is a for loop comment
/* while loop explanation */
const x = 5; // if statement note
// class implementation details
"#;

    fs::write("temp_comments.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_comments.ts"))
        .unwrap();

    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "for"),
        "Should NOT detect 'for' in comment"
    );
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "while"),
        "Should NOT detect 'while' in comment"
    );

    fs::remove_file("temp_comments.ts").ok();
}

#[test]
fn test_template_strings_no_false_positive() {
    let analyzer = Analyzer::default();

    let code = r#"
const msg = `This for loop ${x} and if statement`;
const template = `
    function example() {
        while processing
    }
`;
"#;

    fs::write("temp_template.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_template.ts"))
        .unwrap();

    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "for"),
        "Should NOT detect 'for' in template string"
    );
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "function"),
        "Should NOT detect 'function' in template string"
    );

    fs::remove_file("temp_template.ts").ok();
}

#[test]
fn test_python_strings_no_false_positive() {
    let analyzer = Analyzer::default();

    let code = r#"
message = "for loop in string"
info = 'if statement here'
doc = """
    This is a class example
    with while loop
"""
x = 5
"#;

    fs::write("temp_py_strings.py", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_py_strings.py"))
        .unwrap();

    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "for"),
        "Should NOT detect 'for' in Python string"
    );
    assert!(
        !result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "class"),
        "Should NOT detect 'class' in Python string"
    );

    fs::remove_file("temp_py_strings.py").ok();
}

// ============================================================================
// REAL CONSTRUCT DETECTION
// ============================================================================

#[test]
fn test_real_constructs_still_detected() {
    let analyzer = Analyzer::default();

    let code = r#"
// Real code with actual constructs
function test() {
    for (let i = 0; i < 10; i++) {
        if (i > 5) {
            console.log("for and if are real here");
        }
    }
}

class MyClass {
    method() {
        while (true) {
            break;
        }
    }
}
"#;

    fs::write("temp_real.ts", code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_real.ts")).unwrap();

    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "function"),
        "Should detect real function"
    );
    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "for"),
        "Should detect real for loop"
    );
    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "if"),
        "Should detect real if statement"
    );
    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "class"),
        "Should detect real class"
    );
    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "while"),
        "Should detect real while loop"
    );

    fs::remove_file("temp_real.ts").ok();
}

// ============================================================================
// NESTED STRUCTURES
// ============================================================================

#[test]
fn test_deeply_nested_structures() {
    let analyzer = Analyzer::default();

    let code = r#"
function outer() {
    if (true) {
        for (let i = 0; i < 10; i++) {
            while (i < 5) {
                if (i === 3) {
                    const x = 1;
                }
            }
        }
    }
}
"#;

    fs::write("temp_nested.ts", code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_nested.ts")).unwrap();

    let if_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "if")
        .count();
    let for_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();
    let while_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "while")
        .count();

    assert_eq!(if_count, 2, "Should detect 2 if statements");
    assert_eq!(for_count, 1, "Should detect 1 for loop");
    assert_eq!(while_count, 1, "Should detect 1 while loop");

    fs::remove_file("temp_nested.ts").ok();
}

// ============================================================================
// TERNARY OPERATORS
// ============================================================================

#[test]
fn test_ternary_detection_javascript() {
    let analyzer = Analyzer::default();

    let code = r#"
const result = condition ? value1 : value2;
const nested = a ? (b ? c : d) : e;
const complex = isValid ? process() : fallback();
"#;

    fs::write("temp_ternary_js.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_ternary_js.ts"))
        .unwrap();

    let ternary_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "ternary")
        .count();

    assert!(
        ternary_count >= 3,
        "Should detect at least 3 ternary operators, found {}",
        ternary_count
    );

    fs::remove_file("temp_ternary_js.ts").ok();
}

#[test]
fn test_ternary_detection_python() {
    let analyzer = Analyzer::default();

    let code = r#"
result = value1 if condition else value2
nested = a if x else (b if y else c)
"#;

    fs::write("temp_ternary_py.py", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_ternary_py.py"))
        .unwrap();

    let ternary_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "ternary")
        .count();

    assert!(
        ternary_count >= 2,
        "Should detect at least 2 ternary operators in Python"
    );

    fs::remove_file("temp_ternary_py.py").ok();
}

// ============================================================================
// MULTI-LANGUAGE CONSISTENCY
// ============================================================================

#[test]
fn test_simple_function_all_languages() {
    let analyzer = Analyzer::default();

    // TypeScript
    let ts_code = "function add(a: number, b: number): number { return a + b; }";
    fs::write("temp_simple.ts", ts_code).unwrap();
    let ts_result = analyzer.analyze_file(Path::new("temp_simple.ts")).unwrap();

    // Python
    let py_code = "def add(a, b):\n    return a + b";
    fs::write("temp_simple.py", py_code).unwrap();
    let py_result = analyzer.analyze_file(Path::new("temp_simple.py")).unwrap();

    // Ruby
    let rb_code = "def add(a, b)\n  a + b\nend";
    fs::write("temp_simple.rb", rb_code).unwrap();
    let rb_result = analyzer.analyze_file(Path::new("temp_simple.rb")).unwrap();

    // All should detect 1 function
    assert_eq!(
        ts_result
            .calculation
            .breakdown
            .iter()
            .filter(|item| item.kind == "function")
            .count(),
        1,
        "TypeScript should detect 1 function"
    );
    assert_eq!(
        py_result
            .calculation
            .breakdown
            .iter()
            .filter(|item| item.kind == "function")
            .count(),
        1,
        "Python should detect 1 function"
    );
    assert_eq!(
        rb_result
            .calculation
            .breakdown
            .iter()
            .filter(|item| item.kind == "function")
            .count(),
        1,
        "Ruby should detect 1 function"
    );

    fs::remove_file("temp_simple.ts").ok();
    fs::remove_file("temp_simple.py").ok();
    fs::remove_file("temp_simple.rb").ok();
}

// ============================================================================
// GO-SPECIFIC TESTS
// ============================================================================

#[test]
fn test_go_short_var_declaration() {
    let analyzer = Analyzer::default();

    let code = r#"
package main

func main() {
    x := 42
    y, z := 1, 2
    result := compute()
}
"#;

    fs::write("temp_go_vars.go", code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_go_vars.go")).unwrap();

    let var_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "variable")
        .count();

    assert!(
        var_count >= 3,
        "Should detect at least 3 short var declarations"
    );

    fs::remove_file("temp_go_vars.go").ok();
}

#[test]
fn test_go_functions() {
    let analyzer = Analyzer::default();

    let code = r#"
package main

func add(a int, b int) int {
    return a + b
}

func (s *Service) process() error {
    return nil
}
"#;

    fs::write("temp_go_funcs.go", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_go_funcs.go"))
        .unwrap();

    let func_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "function")
        .count();

    assert_eq!(
        func_count, 2,
        "Should detect 2 functions (regular + method)"
    );

    fs::remove_file("temp_go_funcs.go").ok();
}

// ============================================================================
// RUST-SPECIFIC TESTS
// ============================================================================

#[test]
fn test_rust_let_declarations() {
    let analyzer = Analyzer::default();

    let code = r#"
fn main() {
    let x = 5;
    let mut y = 10;
    let z: i32 = 42;
}
"#;

    fs::write("temp_rust_vars.rs", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_rust_vars.rs"))
        .unwrap();

    let var_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "variable")
        .count();

    assert!(var_count >= 3, "Should detect at least 3 let declarations");

    fs::remove_file("temp_rust_vars.rs").ok();
}

#[test]
fn test_rust_impl_blocks() {
    let analyzer = Analyzer::default();

    let code = r#"
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn distance(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}
"#;

    fs::write("temp_rust_impl.rs", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_rust_impl.rs"))
        .unwrap();

    let class_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "class")
        .count();

    assert!(class_count >= 1, "Should detect struct/impl as class");

    fs::remove_file("temp_rust_impl.rs").ok();
}

// ============================================================================
// COMPLEX REAL-WORLD SCENARIOS
// ============================================================================

#[test]
fn test_complex_class_with_methods() {
    let analyzer = Analyzer::default();

    let code = r#"
class UserService {
    private users: User[] = [];

    constructor(private db: Database) {}

    async createUser(data: UserData): Promise<User> {
        const user = new User(data);
        await this.db.save(user);
        this.users.push(user);
        return user;
    }

    findById(id: string): User | undefined {
        return this.users.find(u => u.id === id);
    }

    async deleteUser(id: string): Promise<boolean> {
        const index = this.users.findIndex(u => u.id === id);
        if (index !== -1) {
            this.users.splice(index, 1);
            return true;
        }
        return false;
    }
}
"#;

    fs::write("temp_complex_class.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_complex_class.ts"))
        .unwrap();

    let class_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "class")
        .count();
    let func_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "function")
        .count();

    assert_eq!(class_count, 1, "Should detect 1 class");
    assert!(func_count >= 3, "Should detect at least 3 methods");

    fs::remove_file("temp_complex_class.ts").ok();
}

#[test]
fn test_mixed_constructs() {
    let analyzer = Analyzer::default();

    let code = r#"
const config = { debug: true };

function processData(items: any[]) {
    for (const item of items) {
        if (item.valid) {
            const result = item.process ? item.process() : defaultProcess(item);
            while (result.pending) {
                await result.wait();
            }
        }
    }
}

class DataProcessor {
    process(data: any) {
        return data;
    }
}
"#;

    fs::write("temp_mixed.ts", code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_mixed.ts")).unwrap();

    println!("\nMixed constructs breakdown:");
    for item in &result.calculation.breakdown {
        println!("  {} at line {}: {} pts", item.kind, item.line, item.cost);
    }

    assert!(result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "variable"));
    assert!(result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "function"));
    assert!(result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "for"));
    assert!(result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "if"));
    assert!(result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "while"));
    assert!(result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "class"));

    fs::remove_file("temp_mixed.ts").ok();
}

// ============================================================================
// EDGE CASE: EMPTY FILES
// ============================================================================

#[test]
fn test_empty_file() {
    let analyzer = Analyzer::default();

    fs::write("temp_empty.ts", "").unwrap();
    let result = analyzer.analyze_file(Path::new("temp_empty.ts")).unwrap();

    assert_eq!(
        result.calculation.total, 0,
        "Empty file should have 0 budget"
    );
    assert_eq!(
        result.calculation.breakdown.len(),
        0,
        "Empty file should have no constructs"
    );

    fs::remove_file("temp_empty.ts").ok();
}

// ============================================================================
// EDGE CASE: COMMENTS ONLY
// ============================================================================

#[test]
fn test_comments_only_file() {
    let analyzer = Analyzer::default();

    let code = r#"
// This is a comment
/* This is a multi-line
   comment with keywords:
   function, class, if, for, while
*/
// Another comment
"#;

    fs::write("temp_comments_only.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_comments_only.ts"))
        .unwrap();

    assert_eq!(
        result.calculation.total, 0,
        "Comments-only file should have 0 budget"
    );

    fs::remove_file("temp_comments_only.ts").ok();
}

// ============================================================================
// PERFORMANCE TEST
// ============================================================================

#[test]
fn test_large_file_performance() {
    let analyzer = Analyzer::default();

    // Generate a large file with 100 functions
    let mut code = String::new();
    for i in 0..100 {
        code.push_str(&format!(
            "function func{}(x) {{ if (x > 0) {{ return x * 2; }} return 0; }}\n",
            i
        ));
    }

    fs::write("temp_large.ts", &code).unwrap();

    let start = std::time::Instant::now();
    let result = analyzer.analyze_file(Path::new("temp_large.ts")).unwrap();
    let duration = start.elapsed();

    println!("\nLarge file analysis:");
    println!("  Functions: 100");
    println!("  Total budget: {} pts", result.calculation.total);
    println!("  Time: {:?}", duration);

    assert!(
        duration.as_secs() < 5,
        "Should analyze large file in under 5 seconds"
    );

    fs::remove_file("temp_large.ts").ok();
}

#[test]
fn test_no_false_positive_in_format_macro() {
    let analyzer = Analyzer::default();

    // Rust code with format! macro containing "for"
    let rust_code = r#"
fn main() {
    let message = format!("test");
    let breakdown_text = format!("\n- **{}**: {}pts (×{})", "kind", 10, 2);
    let error = format!("❌ Analysis error: {}", "test");
}
"#;

    fs::write("temp_format_test.rs", rust_code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_format_test.rs"))
        .unwrap();

    // Should detect 1 function (main) but NO for loops
    let functions = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "function")
        .count();
    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();
    let variables = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "variable")
        .count();

    println!("\n🔍 Format macro test:");
    println!("  Functions: {}", functions);
    println!("  For loops: {} (should be 0!)", for_loops);
    println!("  Variables: {} (should be 3)", variables);

    assert_eq!(functions, 1, "Should detect 1 function (main)");
    assert_eq!(
        for_loops, 0,
        "Should NOT detect any for loops in format! macro"
    );
    assert_eq!(variables, 3, "Should detect 3 variables");

    fs::remove_file("temp_format_test.rs").ok();
}

#[test]
fn test_real_for_loop_still_detected() {
    let analyzer = Analyzer::default();

    // Rust code with REAL for loop
    let rust_code = r#"
fn main() {
    for i in 0..10 {
        println!("{}", i);
    }
}
"#;

    fs::write("temp_real_for.rs", rust_code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_real_for.rs"))
        .unwrap();

    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();

    println!("\n✅ Real for loop test:");
    println!("  For loops: {} (should be 1)", for_loops);

    assert_eq!(for_loops, 1, "Should detect 1 real for loop");

    fs::remove_file("temp_real_for.rs").ok();
}

#[test]
fn test_no_false_positive_in_other_macros() {
    let analyzer = Analyzer::default();

    // Rust code with various macros
    let rust_code = r#"
fn main() {
    println!("for loop example");
    vec![1, 2, 3];
    assert_eq!(1, 1);
    format_args!("test for formatting");
}
"#;

    fs::write("temp_macros.rs", rust_code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_macros.rs")).unwrap();

    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();

    println!("\n🔍 Macros test:");
    println!("  For loops: {} (should be 0)", for_loops);

    assert_eq!(for_loops, 0, "Should NOT detect for loops in any macros");

    fs::remove_file("temp_macros.rs").ok();
}

#[test]
fn test_format_in_method_call() {
    let analyzer = Analyzer::default();

    // Exact case from your code
    let rust_code = r#"
fn main() {
    log_message(MessageType::ERROR, format!("❌ Analysis error: {}", "test"));
}

fn log_message(msg_type: i32, message: String) {}
"#;

    fs::write("temp_method_format.rs", rust_code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_method_format.rs"))
        .unwrap();

    println!("\n🔍 Debug - All detections:");
    for item in &result.calculation.breakdown {
        println!(
            "  {} at line {}: {}",
            item.kind, item.line, item.description
        );
    }

    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();

    println!("\nFor loops detected: {} (should be 0)", for_loops);

    assert_eq!(
        for_loops, 0,
        "Should NOT detect for loops in format!() inside method call"
    );

    fs::remove_file("temp_method_format.rs").ok();
}

#[test]
fn test_no_false_positive_in_impl_for() {
    let analyzer = Analyzer::default();

    // Rust impl block with "for" keyword
    let rust_code = r#"
trait LanguageServer {}

struct Backend {
    name: String,
}

impl LanguageServer for Backend {
    fn process(&self) {
        println!("Processing");
    }
}
"#;

    fs::write("temp_impl_for.rs", rust_code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_impl_for.rs"))
        .unwrap();

    println!("\n🔍 Impl for test - Breakdown:");
    for item in &result.calculation.breakdown {
        println!(
            "  {} at line {}: {}",
            item.kind, item.line, item.description
        );
    }

    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();

    let classes = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "class")
        .count();

    println!("\nFor loops detected: {} (should be 0)", for_loops);
    println!("Classes detected: {} (should be 2: struct + impl)", classes);

    assert_eq!(
        for_loops, 0,
        "Should NOT detect 'for' in 'impl Trait for Type' as a loop"
    );
    assert_eq!(classes, 2, "Should detect struct and impl as classes");

    fs::remove_file("temp_impl_for.rs").ok();
}

#[test]
fn test_real_for_loop_in_impl_block() {
    let analyzer = Analyzer::default();

    // Real for loop INSIDE impl block
    let rust_code = r#"
trait LanguageServer {}

struct Backend {}

impl LanguageServer for Backend {
    fn process(&self) {
        for i in 0..10 {
            println!("{}", i);
        }
    }
}
"#;

    fs::write("temp_impl_real_for.rs", rust_code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_impl_real_for.rs"))
        .unwrap();

    println!("\n🔍 Real for in impl - Breakdown:");
    for item in &result.calculation.breakdown {
        println!(
            "  {} at line {}: {}",
            item.kind, item.line, item.description
        );
    }

    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();

    let classes = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "class")
        .count();

    println!("\nFor loops detected: {} (should be 1)", for_loops);
    println!("Classes detected: {} (should be 2)", classes);

    assert_eq!(
        for_loops, 1,
        "Should detect 1 real for loop inside impl block"
    );
    assert_eq!(classes, 2, "Should detect struct and impl as classes");

    fs::remove_file("temp_impl_real_for.rs").ok();
}

#[test]
fn test_multiple_impl_blocks() {
    let analyzer = Analyzer::default();

    let rust_code = r#"
trait A {}
trait B {}
struct S {}

impl A for S {}

impl B for S {
    fn method(&self) {
        for x in 0..5 {
            println!("{}", x);
        }
    }
}
"#;

    fs::write("temp_multiple_impl.rs", rust_code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_multiple_impl.rs"))
        .unwrap();

    let for_loops = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "for")
        .count();

    let classes = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "class")
        .count();

    // Should detect: 1 struct + 2 impl blocks = 3 classes
    // Should detect: 1 real for loop (not the "for" in impl signatures)
    assert_eq!(
        for_loops, 1,
        "Should detect 1 for loop, not the 'for' in impl signatures"
    );
    assert_eq!(
        classes, 3,
        "Should detect 1 struct + 2 impl blocks as classes"
    );

    fs::remove_file("temp_multiple_impl.rs").ok();
}
