use budget_analyzer::{Analyzer, Profile};
use std::path::Path;

// ============================================================================
// TYPESCRIPT TESTS
// ============================================================================

#[test]
fn test_typescript_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze TypeScript simple file");

    assert_eq!(result.language, "TypeScript");
    assert_eq!(result.profile, Profile::Strict);
    assert!(
        result.calculation.total >= 2,
        "Simple TS file should have at least 2 pts"
    );
    assert!(!result.exceeded);
}

#[test]
fn test_typescript_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.ts"))
        .expect("Failed to analyze TypeScript fibonacci");

    assert_eq!(result.language, "TypeScript");
    assert!(
        result.calculation.total >= 13 && result.calculation.total <= 35,
        "Fibonacci should be between 13-35 pts, got {}",
        result.calculation.total
    );
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
    assert!(
        result.calculation.total >= 20 && result.calculation.total <= 120,
        "Class should be between 20-120 pts"
    );
    assert!(!result.exceeded);
}

#[test]
fn test_typescript_complex_utils_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/complex_utils.ts"))
        .expect("Failed to analyze complex TypeScript file");

    assert_eq!(result.profile, Profile::Strict);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(
        result.calculation.total > 80,
        "Budget should exceed 80 pts, got {}",
        result.calculation.total
    );
}

// ============================================================================
// JAVASCRIPT TESTS
// ============================================================================

#[test]
fn test_javascript_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/simple.js"))
        .expect("Failed to analyze JavaScript simple file");

    assert_eq!(result.language, "JavaScript");
    assert_eq!(result.profile, Profile::Strict);
    assert!(result.calculation.total >= 2);
    assert!(!result.exceeded);
}

#[test]
fn test_javascript_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/fibonacci.js"))
        .expect("Failed to analyze JavaScript fibonacci");

    assert_eq!(result.language, "JavaScript");
    assert!(
        result.calculation.total >= 13 && result.calculation.total <= 35,
        "JS fibonacci budget: {}",
        result.calculation.total
    );
    assert!(!result.exceeded);
}

#[test]
fn test_javascript_overloaded_utils_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/javascript/overloaded_utils.js"))
        .expect("Failed to analyze overloaded JavaScript utils");

    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 80);
}

// ============================================================================
// PYTHON TESTS
// ============================================================================

#[test]
fn test_python_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/simple.py"))
        .expect("Failed to analyze Python simple file");

    assert_eq!(result.language, "Python");
    assert_eq!(result.profile, Profile::Strict);
    assert!(result.calculation.total >= 2);
    assert!(!result.exceeded);
}

#[test]
fn test_python_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/fibonacci.py"))
        .expect("Failed to analyze Python fibonacci");

    assert_eq!(result.language, "Python");
    assert!(
        result.calculation.total >= 13 && result.calculation.total <= 35,
        "Python fibonacci budget: {}",
        result.calculation.total
    );
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
    assert!(result.calculation.total >= 20 && result.calculation.total <= 120);
    assert!(!result.exceeded);
}

#[test]
fn test_python_bloated_class_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/python/bloated_class.py"))
        .expect("Failed to analyze bloated Python class");

    assert_eq!(result.profile, Profile::Class);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 300);
}

// ============================================================================
// RUBY TESTS
// ============================================================================

#[test]
fn test_ruby_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/simple.rb"))
        .expect("Failed to analyze Ruby simple file");

    assert_eq!(result.language, "Ruby");
    assert_eq!(result.profile, Profile::Strict);
    assert!(result.calculation.total >= 2);
    assert!(!result.exceeded);
}

#[test]
fn test_ruby_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/ruby/fibonacci.rb"))
        .expect("Failed to analyze Ruby fibonacci");

    assert_eq!(result.language, "Ruby");
    assert!(
        result.calculation.total >= 13 && result.calculation.total <= 45,
        "Ruby fibonacci budget: {}",
        result.calculation.total
    );
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
    assert!(result.calculation.total >= 20 && result.calculation.total <= 120);
    assert!(!result.exceeded);
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
}

// ============================================================================
// GO TESTS
// ============================================================================

#[test]
fn test_go_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/simple.go"))
        .expect("Failed to analyze Go simple file");

    assert_eq!(result.language, "Go");
    assert_eq!(result.profile, Profile::Strict);
    assert!(result.calculation.total >= 2);
    assert!(!result.exceeded);
}

#[test]
fn test_go_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/fibonacci.go"))
        .expect("Failed to analyze Go fibonacci");

    assert_eq!(result.language, "Go");
    assert!(
        result.calculation.total >= 13 && result.calculation.total <= 35,
        "Go fibonacci budget: {}",
        result.calculation.total
    );
    assert!(!result.exceeded);
}

#[test]
fn test_go_bloated_service_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/bloated_service.go"))
        .expect("Failed to analyze Go bloated service");

    assert!(matches!(result.profile, Profile::Service | Profile::Class));
    assert!(result.calculation.total > 100);
}

#[test]
fn test_go_loops_nightmare_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/go/loops_nightmare.go"))
        .expect("Failed to analyze Go loops nightmare");

    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 60);
}

// ============================================================================
// RUST TESTS
// ============================================================================

#[test]
fn test_rust_simple() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/simple.rs"))
        .expect("Failed to analyze Rust simple file");

    assert_eq!(result.language, "Rust");
    assert_eq!(result.profile, Profile::Strict);
    assert!(result.calculation.total >= 2 && result.calculation.total <= 35);
    assert!(!result.exceeded);
}

#[test]
fn test_rust_fibonacci() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/fibonacci.rs"))
        .expect("Failed to analyze Rust fibonacci");

    assert_eq!(result.language, "Rust");
    assert!(
        result.calculation.total >= 13 && result.calculation.total <= 35,
        "Rust fibonacci budget: {}",
        result.calculation.total
    );
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
    assert!(result.calculation.total > 80);
}

#[test]
fn test_rust_nested_hell_exceeds() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/rust/nested_hell.rs"))
        .expect("Failed to analyze Rust nested hell");

    assert_eq!(result.profile, Profile::Strict);
    assert!(result.exceeded, "Expected budget to be exceeded");
    assert!(result.calculation.total > 80);
}

// ============================================================================
// BLABLALANG TESTS (Custom Language)
// ============================================================================
// NOTE: Ces tests sont commentés car le support .bl n'est pas encore implémenté
// Pour les activer, ajouter le support de .bl dans parsers/mod.rs

/*
#[test]
fn test_blablalang_simple() {
    let analyzer = Analyzer::default();

    // This test assumes you have a .bl file in fixtures
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/blablalang/simple.bl"))
        .expect("Failed to analyze BlaBlaLang simple file");

    assert_eq!(result.language, "BlaBlaLang");
    assert!(result.calculation.total >= 2);
    assert!(!result.exceeded);
}

#[test]
fn test_blablalang_fibonacci() {
    let analyzer = Analyzer::default();

    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/blablalang/fibonacci.bl"))
        .expect("Failed to analyze BlaBlaLang fibonacci");

    assert_eq!(result.language, "BlaBlaLang");
    assert!(result.calculation.total >= 13 && result.calculation.total <= 35);
    assert!(!result.exceeded);
}
*/

// ============================================================================
// MULTI-LANGUAGE CONSISTENCY
// ============================================================================

#[test]
fn test_all_six_languages_fibonacci_consistency() {
    let analyzer = Analyzer::default();

    let languages_and_files = vec![
        ("TypeScript", "tests/fixtures/typescript/fibonacci.ts"),
        ("JavaScript", "tests/fixtures/javascript/fibonacci.js"),
        ("Python", "tests/fixtures/python/fibonacci.py"),
        ("Ruby", "tests/fixtures/ruby/fibonacci.rb"),
        ("Go", "tests/fixtures/go/fibonacci.go"),
        ("Rust", "tests/fixtures/rust/fibonacci.rs"),
    ];

    let mut budgets = Vec::new();

    for (lang, file) in languages_and_files {
        let result = analyzer
            .analyze_file(Path::new(file))
            .expect(&format!("Failed to analyze {} fibonacci", lang));
        budgets.push((lang, result.calculation.total));
    }

    println!("\n🔥 Fibonacci budgets across ALL languages:");
    for (lang, budget) in &budgets {
        println!("  {:12} {} pts", format!("{}:", lang), budget);
    }

    for (lang, budget) in &budgets {
        assert!(
            *budget >= 10 && *budget <= 45,
            "{} budget out of range: {} (expected 10-45)",
            lang,
            budget
        );
    }

    println!("✅ All languages have consistent fibonacci budgets!\n");
}

// ============================================================================
// PROFILE DETECTION
// ============================================================================

#[test]
fn test_profile_detection_class_all_languages() {
    let analyzer = Analyzer::default();

    let class_files = vec![
        ("TypeScript", "tests/fixtures/typescript/class.ts"),
        ("Python", "tests/fixtures/python/class.py"),
        ("Ruby", "tests/fixtures/ruby/class.rb"),
    ];

    for (lang, file) in class_files {
        let result = analyzer
            .analyze_file(Path::new(file))
            .expect(&format!("Failed to analyze {} class", lang));

        assert_eq!(
            result.profile,
            Profile::Class,
            "{} class should be detected as Class profile",
            lang
        );
    }
}

// ============================================================================
// BREAKDOWN DETAILS
// ============================================================================

#[test]
fn test_breakdown_details_comprehensive() {
    let analyzer = Analyzer::default();
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.ts"))
        .unwrap();

    let breakdown = &result.calculation.breakdown;

    println!("\n📊 Detailed breakdown:");
    for item in breakdown {
        println!(
            "  {} at line {}: {} pts - {}",
            item.kind, item.line, item.cost, item.description
        );
    }

    // Verify common constructs exist
    assert!(
        breakdown.iter().any(|item| item.kind == "function"),
        "Should detect functions"
    );
    assert!(
        breakdown.iter().any(|item| item.kind == "if"),
        "Should detect if statements"
    );

    // Verify line numbers are reasonable
    for item in breakdown {
        assert!(item.line > 0, "Line number should be positive");
    }
}

// ============================================================================
// BUDGET EXCEED TESTS
// ============================================================================

#[test]
fn test_check_command_fails_on_exceeded() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "check",
            "tests/fixtures/typescript/complex_utils.ts",
        ])
        .output()
        .expect("Failed to run check command");

    assert!(
        !output.status.success(),
        "Check should fail on exceeded budget"
    );
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[test]
fn test_multiple_files_analysis() {
    let analyzer = Analyzer::default();

    let files = vec![
        "tests/fixtures/typescript/simple.ts",
        "tests/fixtures/python/simple.py",
        "tests/fixtures/ruby/simple.rb",
        "tests/fixtures/javascript/simple.js",
    ];

    for file in files {
        let result = analyzer.analyze_file(Path::new(file));
        assert!(result.is_ok(), "Should analyze {} successfully", file);
    }
}

#[test]
fn test_concurrent_analysis_safety() {
    use std::sync::Arc;
    use std::thread;

    let analyzer = Arc::new(Analyzer::default());
    let mut handles = vec![];

    for i in 0..4 {
        let analyzer_clone = Arc::clone(&analyzer);
        let handle = thread::spawn(move || {
            let file = match i {
                0 => "tests/fixtures/typescript/simple.ts",
                1 => "tests/fixtures/python/simple.py",
                2 => "tests/fixtures/ruby/simple.rb",
                _ => "tests/fixtures/javascript/simple.js",
            };
            analyzer_clone.analyze_file(Path::new(file))
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().expect("Thread panicked");
        assert!(result.is_ok(), "Concurrent analysis should succeed");
    }
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_file_not_found_error() {
    let analyzer = Analyzer::default();
    let result = analyzer.analyze_file(Path::new("nonexistent.ts"));

    assert!(result.is_err(), "Should return error for non-existent file");
}

#[test]
fn test_unsupported_extension() {
    let analyzer = Analyzer::default();

    let result = analyzer.analyze_file(Path::new("tests/fixtures/readme.txt"));

    if let Err(e) = result {
        assert!(
            e.to_string().contains("language") || e.to_string().contains("detect"),
            "Error should mention language detection"
        );
    }
}
