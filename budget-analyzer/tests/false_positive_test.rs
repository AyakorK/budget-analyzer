use budget_analyzer::Analyzer;
use std::path::Path;
use std::fs;

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

    let result = analyzer
        .analyze_file(Path::new("temp_strings.ts"))
        .expect("Failed to analyze");

    println!("Breakdown:");
    for item in &result.calculation.breakdown {
        println!("  {} at line {}: {} pts", item.kind, item.line, item.cost);
    }

    let has_for = result.calculation.breakdown.iter().any(|item| item.kind == "for");
    let has_if = result.calculation.breakdown.iter().any(|item| item.kind == "if");
    let has_while = result.calculation.breakdown.iter().any(|item| item.kind == "while");
    let has_class = result.calculation.breakdown.iter().any(|item| item.kind == "class");

    assert!(!has_for, "Should NOT detect 'for' in string");
    assert!(!has_if, "Should NOT detect 'if' in string");
    assert!(!has_while, "Should NOT detect 'while' in string");
    assert!(!has_class, "Should NOT detect 'class' in string");

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
        .expect("Failed to analyze");

    println!("Breakdown:");
    for item in &result.calculation.breakdown {
        println!("  {} at line {}: {} pts", item.kind, item.line, item.cost);
    }

    let has_for = result.calculation.breakdown.iter().any(|item| item.kind == "for");
    let has_while = result.calculation.breakdown.iter().any(|item| item.kind == "while");
    let has_if = result.calculation.breakdown.iter().any(|item| item.kind == "if");
    let has_class = result.calculation.breakdown.iter().any(|item| item.kind == "class");

    assert!(!has_for, "Should NOT detect 'for' in comment");
    assert!(!has_while, "Should NOT detect 'while' in comment");
    assert!(!has_if, "Should NOT detect 'if' in comment");
    assert!(!has_class, "Should NOT detect 'class' in comment");

    fs::remove_file("temp_comments.ts").ok();
}

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

    let result = analyzer
        .analyze_file(Path::new("temp_real.ts"))
        .expect("Failed to analyze");

    println!("\nBreakdown:");
    for item in &result.calculation.breakdown {
        println!("  {} at line {}: {} pts", item.kind, item.line, item.cost);
    }
    println!("Total: {} pts\n", result.calculation.total);

    let has_function = result.calculation.breakdown.iter().any(|item| item.kind == "function");
    let has_for = result.calculation.breakdown.iter().any(|item| item.kind == "for");
    let has_if = result.calculation.breakdown.iter().any(|item| item.kind == "if");
    let has_class = result.calculation.breakdown.iter().any(|item| item.kind == "class");
    let has_while = result.calculation.breakdown.iter().any(|item| item.kind == "while");

    assert!(has_function, "Should detect real function");
    assert!(has_for, "Should detect real for loop");
    assert!(has_if, "Should detect real if statement");
    assert!(has_class, "Should detect real class");
    assert!(has_while, "Should detect real while loop");

    fs::remove_file("temp_real.ts").ok();
}

#[test]
fn test_template_strings() {
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
        .expect("Failed to analyze");

    let has_for = result.calculation.breakdown.iter().any(|item| item.kind == "for");
    let has_if = result.calculation.breakdown.iter().any(|item| item.kind == "if");
    let has_function = result.calculation.breakdown.iter().any(|item| item.kind == "function");
    let has_while = result.calculation.breakdown.iter().any(|item| item.kind == "while");

    assert!(!has_for, "Should NOT detect 'for' in template string");
    assert!(!has_if, "Should NOT detect 'if' in template string");
    assert!(!has_function, "Should NOT detect 'function' in template string");
    assert!(!has_while, "Should NOT detect 'while' in template string");

    fs::remove_file("temp_template.ts").ok();
}

#[test]
fn test_python_strings() {
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
        .expect("Failed to analyze");

    let has_for = result.calculation.breakdown.iter().any(|item| item.kind == "for");
    let has_if = result.calculation.breakdown.iter().any(|item| item.kind == "if");
    let has_class = result.calculation.breakdown.iter().any(|item| item.kind == "class");
    let has_while = result.calculation.breakdown.iter().any(|item| item.kind == "while");

    assert!(!has_for, "Should NOT detect 'for' in Python string");
    assert!(!has_if, "Should NOT detect 'if' in Python string");
    assert!(!has_class, "Should NOT detect 'class' in Python string");
    assert!(!has_while, "Should NOT detect 'while' in Python string");

    fs::remove_file("temp_py_strings.py").ok();
}
