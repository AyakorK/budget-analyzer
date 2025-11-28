use budget_analyzer::{Analyzer, BudgetConfig};
use std::path::Path;
use std::fs;

#[test]
fn test_ternary_bonus() {
    let analyzer = Analyzer::default();

    // Create file with ternary
    let ternary_code = r#"
const result = condition ? value1 : value2;
const another = x > 0 ? "positive" : "negative";
"#;

    fs::write("temp_ternary.ts", ternary_code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_ternary.ts"))
        .expect("Failed to analyze");

    // Check if ternary bonus is applied
    let ternary_items: Vec<_> = result.calculation.breakdown
        .iter()
        .filter(|item| item.kind == "ternary")
        .collect();

    assert!(!ternary_items.is_empty(), "Should detect ternary operators");

    for item in ternary_items {
        assert_eq!(item.cost, -5, "Ternary should have -5 bonus");
    }

    fs::remove_file("temp_ternary.ts").ok();
}

#[test]
fn test_custom_bonus() {
    use std::collections::HashMap;

    let mut config = BudgetConfig::default();

    // Add custom bonus
    config.bonuses.insert("ternary".to_string(), -20);

    let analyzer = Analyzer::with_config(config);

    let ternary_code = "const x = a ? b : c;";
    fs::write("temp_custom_bonus.ts", ternary_code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_custom_bonus.ts"))
        .expect("Failed to analyze");

    let ternary_item = result.calculation.breakdown
        .iter()
        .find(|item| item.kind == "ternary");

    if let Some(item) = ternary_item {
        assert_eq!(item.cost, -20, "Custom ternary bonus should be -20");
    }

    fs::remove_file("temp_custom_bonus.ts").ok();
}

#[test]
fn test_malus_application() {
    use std::collections::HashMap;

    let mut config = BudgetConfig::default();

    // Add malus for if statements
    config.maluses.insert("if".to_string(), 10);

    let analyzer = Analyzer::with_config(config);

    let if_code = r#"
if (condition) {
    doSomething();
}
"#;
    fs::write("temp_malus.ts", if_code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_malus.ts"))
        .expect("Failed to analyze");

    let if_item = result.calculation.breakdown
        .iter()
        .find(|item| item.kind == "if");

    if let Some(item) = if_item {
        // Base cost (5) + malus (10) = 15
        assert_eq!(item.cost, 15, "If with malus should be 15");
    }

    fs::remove_file("temp_malus.ts").ok();
}
