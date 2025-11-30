// Unit tests for detection module components
// These test the internal behavior of PatternDetector and HybridDetector

use budget_analyzer::detection::{ConstructType, HybridDetector, PatternDetector};

// ============================================================================
// PATTERN DETECTOR TESTS
// ============================================================================

#[test]
fn test_pattern_detector_typescript_patterns() {
    let detector = PatternDetector::new();

    assert_eq!(
        detector.patterns.get("variable_declaration"),
        Some(&ConstructType::Variable)
    );
    assert_eq!(
        detector.patterns.get("function_declaration"),
        Some(&ConstructType::Function)
    );
    assert_eq!(
        detector.patterns.get("class_declaration"),
        Some(&ConstructType::Class)
    );
}

#[test]
fn test_pattern_detector_javascript_patterns() {
    let detector = PatternDetector::new();

    assert_eq!(
        detector.patterns.get("arrow_function"),
        Some(&ConstructType::Function)
    );
    assert_eq!(
        detector.patterns.get("for_in_statement"),
        Some(&ConstructType::For)
    );
}

#[test]
fn test_pattern_detector_python_patterns() {
    let detector = PatternDetector::new();

    assert_eq!(
        detector.patterns.get("function_definition"),
        Some(&ConstructType::Function)
    );
    assert_eq!(
        detector.patterns.get("class_definition"),
        Some(&ConstructType::Class)
    );
    assert_eq!(
        detector.patterns.get("assignment"),
        Some(&ConstructType::Variable)
    );
}

#[test]
fn test_pattern_detector_ruby_patterns() {
    let detector = PatternDetector::new();

    assert_eq!(
        detector.patterns.get("method"),
        Some(&ConstructType::Function)
    );
    assert_eq!(detector.patterns.get("unless"), Some(&ConstructType::If));
    assert_eq!(detector.patterns.get("until"), Some(&ConstructType::While));
}

#[test]
fn test_pattern_detector_go_patterns() {
    let detector = PatternDetector::new();

    assert_eq!(
        detector.patterns.get("short_var_declaration"),
        Some(&ConstructType::Variable)
    );
    assert_eq!(
        detector.patterns.get("method_declaration"),
        Some(&ConstructType::Function)
    );
}

#[test]
fn test_pattern_detector_rust_patterns() {
    let detector = PatternDetector::new();

    assert_eq!(
        detector.patterns.get("let_declaration"),
        Some(&ConstructType::Variable)
    );
    assert_eq!(
        detector.patterns.get("function_item"),
        Some(&ConstructType::Function)
    );
    assert_eq!(
        detector.patterns.get("impl_item"),
        Some(&ConstructType::Class)
    );
}

#[test]
fn test_pattern_detector_add_pattern() {
    let mut detector = PatternDetector::new();

    detector.add_pattern("custom_loop".to_string(), ConstructType::For);

    assert_eq!(
        detector.patterns.get("custom_loop"),
        Some(&ConstructType::For)
    );
}

#[test]
fn test_pattern_detector_remove_pattern() {
    let mut detector = PatternDetector::new();

    let removed = detector.remove_pattern("for_statement");

    assert_eq!(removed, Some(ConstructType::For));
    assert!(!detector.has_pattern("for_statement"));
}

#[test]
fn test_pattern_detector_has_pattern() {
    let detector = PatternDetector::new();

    assert!(detector.has_pattern("function_declaration"));
    assert!(detector.has_pattern("class_declaration"));
    assert!(!detector.has_pattern("non_existent_pattern"));
}

#[test]
fn test_pattern_detector_get_patterns_for_construct() {
    let detector = PatternDetector::new();

    let function_patterns = detector.get_patterns_for_construct(ConstructType::Function);
    let class_patterns = detector.get_patterns_for_construct(ConstructType::Class);

    // Should have multiple function patterns
    assert!(
        function_patterns.len() > 5,
        "Should have multiple function patterns, got {}",
        function_patterns.len()
    );

    // Should have multiple class patterns
    assert!(
        class_patterns.len() > 3,
        "Should have multiple class patterns, got {}",
        class_patterns.len()
    );
}

#[test]
fn test_pattern_detector_default() {
    let detector1 = PatternDetector::new();
    let detector2 = PatternDetector::default();

    assert_eq!(
        detector1.patterns.len(),
        detector2.patterns.len(),
        "Default and new should create same detector"
    );
}

// ============================================================================
// HYBRID DETECTOR TESTS
// ============================================================================

#[test]
fn test_hybrid_detector_creation() {
    let detector = HybridDetector::new();

    // Should have access to both detectors
    assert!(detector.pattern().patterns.len() > 0);
}

#[test]
fn test_hybrid_detector_default() {
    let detector1 = HybridDetector::new();
    let detector2 = HybridDetector::default();

    // Both should have same number of patterns
    assert_eq!(
        detector1.pattern().patterns.len(),
        detector2.pattern().patterns.len()
    );
}

#[test]
fn test_hybrid_detector_with_custom_pattern() {
    let mut custom_pattern = PatternDetector::new();
    custom_pattern.add_pattern("custom_node".to_string(), ConstructType::Function);

    let detector = HybridDetector::with_pattern_detector(custom_pattern);

    assert!(detector.pattern().has_pattern("custom_node"));
}

#[test]
fn test_hybrid_detector_pattern_mutation() {
    let mut detector = HybridDetector::new();

    detector
        .pattern_mut()
        .add_pattern("test_pattern".to_string(), ConstructType::Variable);

    assert!(detector.pattern().has_pattern("test_pattern"));
}

#[test]
fn test_hybrid_detector_semantic_reference() {
    let detector = HybridDetector::new();

    // Should be able to get reference to semantic detector
    let _semantic = detector.semantic();
    // Just verify we can access it without error
}

#[test]
fn test_hybrid_detector_pattern_reference() {
    let detector = HybridDetector::new();

    // Should be able to get reference to pattern detector
    let pattern = detector.pattern();
    assert!(pattern.patterns.len() > 0);
}

#[test]
fn test_construct_type_as_str() {
    assert_eq!(ConstructType::Variable.as_str(), "variable");
    assert_eq!(ConstructType::Function.as_str(), "function");
    assert_eq!(ConstructType::If.as_str(), "if");
    assert_eq!(ConstructType::While.as_str(), "while");
    assert_eq!(ConstructType::For.as_str(), "for");
    assert_eq!(ConstructType::Class.as_str(), "class");
    assert_eq!(ConstructType::Ternary.as_str(), "ternary");
}

#[test]
fn test_construct_type_equality() {
    assert_eq!(ConstructType::Variable, ConstructType::Variable);
    assert_ne!(ConstructType::Variable, ConstructType::Function);
}

#[test]
fn test_construct_type_clone() {
    let c1 = ConstructType::Function;
    let c2 = c1.clone();

    assert_eq!(c1, c2);
}

// ============================================================================
// INTEGRATION TESTS (Pattern + Hybrid)
// ============================================================================

#[test]
fn test_pattern_detector_with_all_construct_types() {
    let detector = PatternDetector::new();

    // Verify we have patterns for all construct types
    let has_variable = detector
        .get_patterns_for_construct(ConstructType::Variable)
        .len()
        > 0;
    let has_function = detector
        .get_patterns_for_construct(ConstructType::Function)
        .len()
        > 0;
    let has_if = detector.get_patterns_for_construct(ConstructType::If).len() > 0;
    let has_while = detector
        .get_patterns_for_construct(ConstructType::While)
        .len()
        > 0;
    let has_for = detector
        .get_patterns_for_construct(ConstructType::For)
        .len()
        > 0;
    let has_class = detector
        .get_patterns_for_construct(ConstructType::Class)
        .len()
        > 0;
    let has_ternary = detector
        .get_patterns_for_construct(ConstructType::Ternary)
        .len()
        > 0;

    assert!(has_variable, "Should have variable patterns");
    assert!(has_function, "Should have function patterns");
    assert!(has_if, "Should have if patterns");
    assert!(has_while, "Should have while patterns");
    assert!(has_for, "Should have for patterns");
    assert!(has_class, "Should have class patterns");
    assert!(has_ternary, "Should have ternary patterns");
}

#[test]
fn test_hybrid_detector_can_add_patterns_at_runtime() {
    let mut detector = HybridDetector::new();

    // Add custom patterns
    detector
        .pattern_mut()
        .add_pattern("my_custom_var".to_string(), ConstructType::Variable);
    detector
        .pattern_mut()
        .add_pattern("my_custom_func".to_string(), ConstructType::Function);

    assert!(detector.pattern().has_pattern("my_custom_var"));
    assert!(detector.pattern().has_pattern("my_custom_func"));
}

#[test]
fn test_multiple_pattern_detectors_independent() {
    let mut detector1 = PatternDetector::new();
    let mut detector2 = PatternDetector::new();

    detector1.add_pattern("custom1".to_string(), ConstructType::Variable);
    detector2.add_pattern("custom2".to_string(), ConstructType::Function);

    assert!(detector1.has_pattern("custom1"));
    assert!(!detector1.has_pattern("custom2"));

    assert!(!detector2.has_pattern("custom1"));
    assert!(detector2.has_pattern("custom2"));
}
