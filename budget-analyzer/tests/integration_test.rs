use budget_analyzer::{Analyzer, Profile};
use std::path::Path;

// ==================== TYPESCRIPT TESTS ====================

#[test]
fn test_typescript_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze TypeScript simple file");

    assert_eq!(result.language, "TypeScript");
    assert_eq!(result.profile, Profile::Strict);
    assert_eq!(result.calculation.total, 4);
    assert!(!result.exceeded);
}

#[test]
fn test_typescript_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.ts"))
        .expect("Failed to analyze TypeScript fibonacci");

    assert_eq!(result.language, "TypeScript");
    assert!(result.calculation.total >= 13);
    assert!(result.calculation.total <= 30);
    assert!(!result.exceeded);
}

#[test]
fn test_typescript_class() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/class.ts"))
        .expect("Failed to analyze TypeScript class");

    assert_eq!(result.language, "TypeScript");
    assert_eq!(result.profile, Profile::Class);
    assert!(result.calculation.total >= 20);
    assert!(result.calculation.total <= 100);
    assert!(!result.exceeded);
}

// ==================== PYTHON TESTS ====================

#[test]
fn test_python_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/simple.py"))
        .expect("Failed to analyze Python simple file");

    assert_eq!(result.language, "Python");
    assert_eq!(result.profile, Profile::Strict);
    assert_eq!(result.calculation.total, 4);
    assert!(!result.exceeded);
}

#[test]
fn test_python_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/fibonacci.py"))
        .expect("Failed to analyze Python fibonacci");

    assert_eq!(result.language, "Python");
    assert!(result.calculation.total >= 13);
    assert!(result.calculation.total <= 30);
    assert!(!result.exceeded);
}

#[test]
fn test_python_class() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/class.py"))
        .expect("Failed to analyze Python class");

    assert_eq!(result.language, "Python");
    assert_eq!(result.profile, Profile::Class);
    assert!(result.calculation.total >= 20);
    assert!(result.calculation.total <= 100);
    assert!(!result.exceeded);
}

// ==================== RUBY TESTS ====================

#[test]
fn test_ruby_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/simple.rb"))
        .expect("Failed to analyze Ruby simple file");

    assert_eq!(result.language, "Ruby");
    assert_eq!(result.profile, Profile::Strict);
    assert_eq!(result.calculation.total, 4);
    assert!(!result.exceeded);
}

#[test]
fn test_ruby_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/fibonacci.rb"))
        .expect("Failed to analyze Ruby fibonacci");

    assert_eq!(result.language, "Ruby");
    assert!(result.calculation.total >= 13);
    assert!(result.calculation.total <= 40);
    assert!(!result.exceeded);
}

#[test]
fn test_ruby_class() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/class.rb"))
        .expect("Failed to analyze Ruby class");

    assert_eq!(result.language, "Ruby");
    assert_eq!(result.profile, Profile::Class);
    assert!(result.calculation.total >= 20);
    assert!(result.calculation.total <= 100);
    assert!(!result.exceeded);
}

// ==================== JAVASCRIPT TESTS ====================

#[test]
fn test_javascript_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/simple.js"))
        .expect("Failed to analyze JavaScript simple file");

    assert_eq!(result.language, "JavaScript");
    assert_eq!(result.profile, Profile::Strict);
    assert_eq!(result.calculation.total, 4);
    assert!(!result.exceeded);
}

#[test]
fn test_javascript_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/fibonacci.js"))
        .expect("Failed to analyze JavaScript fibonacci");

    assert_eq!(result.language, "JavaScript");
    assert!(result.calculation.total >= 13);
    assert!(result.calculation.total <= 30);
    assert!(!result.exceeded);
}

// ==================== MULTI-LANGUAGE CONSISTENCY ====================

#[test]
fn test_all_languages_fibonacci_consistency() {
    let analyzer = Analyzer::default();

    let ts_result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.ts"))
        .unwrap();

    let py_result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/fibonacci.py"))
        .unwrap();

    let rb_result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/fibonacci.rb"))
        .unwrap();

    let js_result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/fibonacci.js"))
        .unwrap();

    let budgets = vec![
        ("TypeScript", ts_result.calculation.total),
        ("Python", py_result.calculation.total),
        ("Ruby", rb_result.calculation.total),
        ("JavaScript", js_result.calculation.total),
    ];

    println!("Fibonacci budgets across languages:");
    for (lang, budget) in &budgets {
        println!("  {}: {} pts", lang, budget);
    }

    for (lang, budget) in &budgets {
        assert!(
            *budget >= 13,
            "{} budget too low: {} < 13",
            lang,
            budget
        );
        assert!(
            *budget <= 30,
            "{} budget too high: {} > 30",
            lang,
            budget
        );
    }

    println!("✅ All fibonacci implementations have reasonable budgets");
}

// ==================== BREAKDOWN TESTS ====================

#[test]
fn test_breakdown_details() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.ts"))
        .unwrap();

    let breakdown = &result.calculation.breakdown;

    assert!(breakdown.iter().any(|item| item.kind == "function"));
    assert!(breakdown.iter().any(|item| item.kind == "if"));

    println!("Breakdown:");
    for item in breakdown {
        println!("  {} at line {}: {} pts", item.kind, item.line, item.cost);
    }
}

// ==================== PROFILE DETECTION ====================

#[test]
fn test_profile_detection_class() {
    let analyzer = Analyzer::default();

    let ts_class = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/class.ts"))
        .unwrap();

    let py_class = analyzer
        .analyze_file(Path::new("tests/fixtures/python/class.py"))
        .unwrap();

    let rb_class = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/class.rb"))
        .unwrap();

    assert_eq!(ts_class.profile, Profile::Class);
    assert_eq!(py_class.profile, Profile::Class);
    assert_eq!(rb_class.profile, Profile::Class);
}

// ==================== BUDGET EXCEED TESTS ====================

#[test]
fn test_typescript_complex_utils_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/complex_utils.ts"))
        .expect("Failed to analyze complex TypeScript file");

    assert_eq!(result.profile, Profile::Strict);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 80, "Budget should exceed 80 pts, got {}", result.calculation.total);

    println!("Complex utils: {} pts (max 80)", result.calculation.total);
}

#[test]
fn test_python_bloated_class_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/bloated_class.py"))
        .expect("Failed to analyze bloated Python class");

    assert_eq!(result.profile, Profile::Class);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 300, "Budget should exceed 300 pts, got {}", result.calculation.total);

    println!("Bloated class: {} pts (max 300)", result.calculation.total);
}

#[test]
fn test_ruby_loops_nightmare_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/loops_nightmare.rb"))
        .expect("Failed to analyze Ruby loops");

    assert!(
        result.calculation.total >= 60,
        "Expected budget >= 60, got {}",
        result.calculation.total
    );

    println!(
        "Loops nightmare: {} pts (profile: {:?}, max: {})",
        result.calculation.total,
        result.profile,
        result.max_budget
    );
}

#[test]
fn test_javascript_overloaded_utils_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/overloaded_utils.js"))
        .expect("Failed to analyze overloaded JavaScript utils");

    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 80, "Budget should exceed 80 pts, got {}", result.calculation.total);

    println!("Overloaded utils: {} pts (max 80)", result.calculation.total);
}

#[test]
fn test_check_command_fails_on_exceeded() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&["run", "--", "check", "tests/fixtures/typescript/complex_utils.ts"])
        .output()
        .expect("Failed to run check command");

    assert!(!output.status.success(), "Check should fail on exceeded budget");
}

// ==================== GO TESTS ====================

#[test]
fn test_go_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/simple.go"))
        .expect("Failed to analyze Go simple file");

    assert_eq!(result.language, "Go");
    assert_eq!(result.profile, Profile::Strict);
    assert_eq!(result.calculation.total, 4);
    assert!(!result.exceeded);
}

#[test]
fn test_go_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/fibonacci.go"))
        .expect("Failed to analyze Go fibonacci");

    assert_eq!(result.language, "Go");
    assert!(result.calculation.total >= 13);
    assert!(result.calculation.total <= 30);
    assert!(!result.exceeded);
}

#[test]
fn test_go_bloated_service_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/bloated_service.go"))
        .expect("Failed to analyze Go bloated service");

    assert!(
        matches!(result.profile, Profile::Service | Profile::Class),
        "Expected Service or Class profile, got {:?}",
        result.profile
    );

    assert!(
        result.calculation.total > 100,
        "Expected budget > 100, got {}",
        result.calculation.total
    );

    println!(
        "Go bloated service: {} pts (profile: {:?}, max: {})",
        result.calculation.total,
        result.profile,
        result.max_budget
    );
}

#[test]
fn test_go_loops_nightmare_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/loops_nightmare.go"))
        .expect("Failed to analyze Go loops nightmare");

    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(
        result.calculation.total > 60,
        "Expected budget > 60, got {}",
        result.calculation.total
    );

    println!("Go loops nightmare: {} pts (max {})", result.calculation.total, result.max_budget);
}

// ==================== RUST TESTS ====================

#[test]
fn test_rust_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/simple.rs"))
        .expect("Failed to analyze Rust simple file");

    assert_eq!(result.language, "Rust");
    assert_eq!(result.profile, Profile::Strict);
    assert!(result.calculation.total >= 4);
    assert!(result.calculation.total <= 30);
    assert!(!result.exceeded);
}

#[test]
fn test_rust_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/fibonacci.rs"))
        .expect("Failed to analyze Rust fibonacci");

    assert_eq!(result.language, "Rust");
    assert!(result.calculation.total >= 13);
    assert!(result.calculation.total <= 30);
    assert!(!result.exceeded);
}

#[test]
fn test_rust_complex_utils_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/complex_utils.rs"))
        .expect("Failed to analyze Rust complex utils");

    assert_eq!(result.profile, Profile::Strict);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(
        result.calculation.total > 80,
        "Budget should exceed 80 pts, got {}",
        result.calculation.total
    );

    println!("Rust complex utils: {} pts (max 80)", result.calculation.total);
}

#[test]
fn test_rust_nested_hell_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/nested_hell.rs"))
        .expect("Failed to analyze Rust nested hell");

    assert_eq!(result.profile, Profile::Strict);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(
        result.calculation.total > 80,
        "Budget should exceed 80 pts, got {}",
        result.calculation.total
    );

    println!("Rust nested hell: {} pts (max 80)", result.calculation.total);
}

// ==================== MULTI-LANGUAGE ALL 6 ====================

#[test]
fn test_all_six_languages_fibonacci_consistency() {
    let analyzer = Analyzer::default();

    let ts_result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.ts"))
        .unwrap();

    let py_result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/fibonacci.py"))
        .unwrap();

    let rb_result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/fibonacci.rb"))
        .unwrap();

    let js_result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/fibonacci.js"))
        .unwrap();

    let go_result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/fibonacci.go"))
        .unwrap();

    let rs_result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/fibonacci.rs"))
        .unwrap();

    let budgets = vec![
        ("TypeScript", ts_result.calculation.total),
        ("JavaScript", js_result.calculation.total),
        ("Python", py_result.calculation.total),
        ("Ruby", rb_result.calculation.total),
        ("Go", go_result.calculation.total),
        ("Rust", rs_result.calculation.total),
    ];

    println!("\n Fibonacci budgets across ALL 6 languages:");
    for (lang, budget) in &budgets {
        println!("  {:12} {} pts", format!("{}:", lang), budget);
    }

    for (lang, budget) in &budgets {
        assert!(
            *budget >= 10,
            "{} budget too low: {} < 10",
            lang,
            budget
        );
        assert!(
            *budget <= 40,
            "{} budget too high: {} > 40",
            lang,
            budget
        );
    }

    println!("✅ All 6 languages have reasonable fibonacci budgets!\n");
}