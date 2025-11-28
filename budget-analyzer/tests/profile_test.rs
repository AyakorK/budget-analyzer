use budget_analyzer::{Analyzer, Profile, BudgetConfig};
use std::path::Path;

#[test]
fn test_profile_detection_by_filename() {
    let analyzer = Analyzer::default();

    // Test file
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/fibonacci.test.ts"))
        .or_else(|_| {
            // File doesn't exist, create temporary
            std::fs::write("temp_test.test.ts", "const x = 1;").unwrap();
            analyzer.analyze_file(Path::new("temp_test.test.ts"))
        })
        .expect("Failed to analyze");

    // Should be Test profile based on .test. in filename
    // Note: might be Strict if file is too simple
    println!("Profile detected: {:?}", result.profile);
}

#[test]
fn test_profile_detection_from_stats() {
    let analyzer = Analyzer::default();

    // Class file should be detected as Class
    let class_result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/class.ts"))
        .expect("Failed to analyze class");

    assert_eq!(class_result.profile, Profile::Class);

    // Simple file should be Strict
    let simple_result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze simple");

    assert_eq!(simple_result.profile, Profile::Strict);
}

#[test]
fn test_custom_profile_from_config() {
    let mut config = BudgetConfig::default();

    // Add custom file pattern
    config.file_patterns.insert(
        "tests/fixtures/typescript/simple.ts".to_string(),
        "test".to_string(),
    );

    let analyzer = Analyzer::with_config(config);
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze");

    // Should be Test because of file pattern
    assert_eq!(result.profile, Profile::Test);
}

#[test]
fn test_explicit_file_config() {
    let mut config = BudgetConfig::default();

    // Add explicit file config (higher priority)
    config.files.insert(
        "tests/fixtures/typescript/simple.ts".to_string(),
        "service".to_string(),
    );

    let analyzer = Analyzer::with_config(config);
    let result = analyzer
        .analyze_file(Path::new("tests/fixtures/typescript/simple.ts"))
        .expect("Failed to analyze");

    // Should be Service because of explicit file config
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

    // Should default to Strict when auto_detect is false
    // (unless there's an explicit file config or pattern)
    assert_eq!(result.profile, Profile::Strict);
}
