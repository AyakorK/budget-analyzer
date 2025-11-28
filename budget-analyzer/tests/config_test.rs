use budget_analyzer::BudgetConfig;
use std::fs;

#[test]
fn test_default_config() {
    let config = BudgetConfig::default();

    assert_eq!(config.profiles.len(), 4);
    assert!(config.profiles.contains_key("strict"));
    assert!(config.profiles.contains_key("class"));
    assert!(config.profiles.contains_key("service"));
    assert!(config.profiles.contains_key("test"));

    assert_eq!(config.rules.variable, 2);
    assert_eq!(config.rules.r#if, 5);
    assert_eq!(config.rules.r#while, 15);
    assert_eq!(config.rules.r#for, 10);
    assert_eq!(config.rules.function, 8);
    assert_eq!(config.rules.class, 15);

    assert_eq!(config.get_bonus("ternary"), -5);
    assert_eq!(config.get_bonus("tail_recursion"), -10);

    assert!(config.auto_detect);
}

#[test]
fn test_custom_config_loading() {
    let config_content = r#"{
        "profiles": {
            "my_profile": {
                "max_budget": 200,
                "description": "Custom test profile"
            }
        },
        "rules": {
            "variable": 3,
            "if": 7,
            "function": 12
        },
        "bonuses": {
            "ternary": -8,
            "custom_bonus": -15
        },
        "maluses": {
            "god_function": 100
        },
        "filePatterns": {
            "**/*.custom.ts": "my_profile"
        },
        "autoDetect": false
    }"#;

    let temp_path = "test_config_temp.json";
    fs::write(temp_path, config_content).expect("Failed to write temp config");

    let config = BudgetConfig::load_from(temp_path).expect("Failed to load custom config");

    assert!(config.profiles.contains_key("my_profile"));
    assert_eq!(config.get_profile("my_profile").unwrap().max_budget, 200);

    assert_eq!(config.rules.variable, 3);
    assert_eq!(config.rules.r#if, 7);
    assert_eq!(config.rules.function, 12);

    assert_eq!(config.rules.r#while, 15); // Default
    assert_eq!(config.rules.r#for, 10);    // Default

    assert_eq!(config.get_bonus("ternary"), -8);
    assert_eq!(config.get_bonus("custom_bonus"), -15);

    assert_eq!(config.get_malus("god_function"), 100);

    assert!(config.file_patterns.contains_key("**/*.custom.ts"));

    assert!(!config.auto_detect);

    fs::remove_file(temp_path).ok();
}

#[test]
fn test_config_merge_with_defaults() {
    let config_content = r#"{
        "rules": {
            "variable": 5
        }
    }"#;

    let temp_path = "test_merge_config.json";
    fs::write(temp_path, config_content).expect("Failed to write temp config");

    let config = BudgetConfig::load_from(temp_path).expect("Failed to load config");

    assert_eq!(config.rules.variable, 5);

    assert_eq!(config.rules.r#if, 5);
    assert_eq!(config.profiles.len(), 4);
    assert!(config.profiles.contains_key("strict"));

    fs::remove_file(temp_path).ok();
}

#[test]
fn test_get_profile() {
    let config = BudgetConfig::default();

    let strict = config.get_profile("strict").unwrap();
    assert_eq!(strict.max_budget, 80);

    let class = config.get_profile("class").unwrap();
    assert_eq!(class.max_budget, 300);

    let nonexistent = config.get_profile("nonexistent");
    assert!(nonexistent.is_none());
}

#[test]
fn test_bonuses_and_maluses() {
    let config = BudgetConfig::default();

    assert_eq!(config.get_bonus("ternary"), -5);

    assert_eq!(config.get_bonus("nonexistent"), 0);

    assert_eq!(config.get_malus("nonexistent"), 0);
}
