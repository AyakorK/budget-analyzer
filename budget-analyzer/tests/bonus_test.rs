use budget_analyzer::{Analyzer, BudgetConfig};
use std::path::Path;
use std::fs;

#[test]
fn test_ternary_bonus() {
    let analyzer = Analyzer::default();

    let ternary_code = r#"const result = condition ? value1 : value2;"#;

    fs::write("temp_ternary.ts", ternary_code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_ternary.ts"))
        .expect("Failed to analyze");

    let ternary_items: Vec<_> = result.calculation.breakdown
        .iter()
        .filter(|item| item.kind == "ternary")
        .collect();

    assert!(!ternary_items.is_empty(), "Should detect ternary operator");

    println!("Ternary items detected: {}", ternary_items.len());
    for item in &ternary_items {
        println!("  Line {}: {} pts", item.line, item.cost);
    }

    let total_ternary_cost: i32 = ternary_items.iter().map(|item| item.cost).sum();
    assert!(total_ternary_cost < 0, "Ternary should have negative cost (bonus), got {}", total_ternary_cost);

    fs::remove_file("temp_ternary.ts").ok();
}

#[test]
fn test_custom_bonus() {
    let mut config = BudgetConfig::default();
    config.bonuses.insert("ternary".to_string(), -20);

    let analyzer = Analyzer::with_config(config);

    let ternary_code = "const x = a ? b : c;";
    fs::write("temp_custom_bonus.ts", ternary_code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_custom_bonus.ts"))
        .expect("Failed to analyze");

    let ternary_items: Vec<_> = result.calculation.breakdown
        .iter()
        .filter(|item| item.kind == "ternary")
        .collect();

    assert!(!ternary_items.is_empty(), "Should detect ternary");

    println!("Ternary items: {}", ternary_items.len());
    for item in &ternary_items {
        println!("  Cost: {} pts", item.cost);
    }

    let total_cost: i32 = ternary_items.iter().map(|item| item.cost).sum();

    assert!(total_cost < 0, "Custom bonus should make cost negative, got {}", total_cost);

    fs::remove_file("temp_custom_bonus.ts").ok();
}

#[test]
fn test_malus_application() {
    let mut config = BudgetConfig::default();
    config.maluses.insert("if".to_string(), 10);

    let analyzer = Analyzer::with_config(config);

    let if_code = r#"if (condition) { doSomething(); }"#;
    fs::write("temp_malus.ts", if_code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_malus.ts"))
        .expect("Failed to analyze");

    let if_items: Vec<_> = result.calculation.breakdown
        .iter()
        .filter(|item| item.kind == "if")
        .collect();

    assert!(!if_items.is_empty(), "Should detect if statement");

    let item = if_items.first().unwrap();
    assert_eq!(item.cost, 15, "If with malus should be 15, got {}", item.cost);

    fs::remove_file("temp_malus.ts").ok();
}

#[test]
fn test_bonus_integration() {
    let analyzer = Analyzer::default();

    let code = r#"
const x = condition ? 1 : 0;
if (x > 0) {
    console.log("positive");
}
"#;

    fs::write("temp_bonus_integration.ts", code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_bonus_integration.ts"))
        .expect("Failed to analyze");

    println!("\nBreakdown:");
    for item in &result.calculation.breakdown {
        println!("  {} at line {}: {} pts", item.kind, item.line, item.cost);
    }
    println!("Total: {} pts\n", result.calculation.total);

    let has_if = result.calculation.breakdown.iter().any(|item| item.kind == "if");
    let has_ternary = result.calculation.breakdown.iter().any(|item| item.kind == "ternary");

    assert!(has_if, "Should detect if statement");
    assert!(has_ternary, "Should detect ternary");

    let ternary_cost: i32 = result.calculation.breakdown
        .iter()
        .filter(|item| item.kind == "ternary")
        .map(|item| item.cost)
        .sum();

    assert!(ternary_cost < 0, "Ternary bonus should be negative");

    fs::remove_file("temp_bonus_integration.ts").ok();
}
