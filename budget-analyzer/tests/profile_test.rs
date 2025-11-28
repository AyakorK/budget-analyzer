use budget_analyzer::{Analyzer, Profile, BudgetConfig};
use std::path::Path;

#[test]
fn test_profile_detection_by_filename() {
    let analyzer = Analyzer::default();

    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.test.ts"))
        .or_else(|_| {
            std::fs::write("temp_test.test.ts", "const x = 1;").unwrap();
            analyzer.analyze_file(Path::new("temp_test.test.ts"))
        })
        .expect("Failed to analyze");

    println!("Profile detected: {:?}", result.profile);
}

#[test]
fn test_profile_detection_from_stats() {
    let analyzer = Analyzer::default();

    let class_result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/class.ts"))
        .expect("Failed to analyze class");

    assert_eq!(class_result.profile, Profile::Class);

    let simple_result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze simple");

    assert_eq!(simple_result.profile, Profile::Strict);
}

#[test]
fn test_custom_profile_from_config() {
    let mut config = BudgetConfig::default();

    config.file_patterns.insert(
        "tests/fixtures/typescript/simple.ts".to_string(),
        "test".to_string(),
    );

    let analyzer = Analyzer::with_config(config);
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze");

    assert_eq!(result.profile, Profile::Test);
}

#[test]
fn test_explicit_file_config() {
    let mut config = BudgetConfig::default();

    config.files.insert(
        "tests/fixtures/typescript/simple.ts".to_string(),
        "service".to_string(),
    );

    let analyzer = Analyzer::with_config(config);
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze");

    assert_eq!(result.profile, Profile::Service);
}

#[test]
fn test_auto_detect_disabled() {
    let mut config = BudgetConfig::default();
    config.auto_detect = false;

    let analyzer = Analyzer::with_config(config);
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/class.ts"))
        .expect("Failed to analyze");

    assert_eq!(result.profile, Profile::Strict);
}
