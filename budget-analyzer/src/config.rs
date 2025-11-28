use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetConfig {
    #[serde(default = "default_profiles")]
    pub profiles: HashMap<String, ProfileConfig>,

    #[serde(default, alias = "filePatterns")]
    pub file_patterns: HashMap<String, String>,

    #[serde(default)]
    pub files: HashMap<String, String>,

    #[serde(default = "default_true", alias = "autoDetect")]
    pub auto_detect: bool,

    #[serde(default = "default_rules")]
    pub rules: BudgetRules,

    #[serde(default)]
    pub bonuses: HashMap<String, i32>,

    #[serde(default)]
    pub maluses: HashMap<String, i32>,

    #[serde(default, alias = "enforceAtCompile")]
    pub enforce_at_compile: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    #[serde(alias = "maxBudget")]
    pub max_budget: i32,

    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetRules {
    #[serde(default = "default_variable_cost")]
    pub variable: i32,

    #[serde(default = "default_if_cost")]
    pub r#if: i32,

    #[serde(default = "default_while_cost")]
    pub r#while: i32,

    #[serde(default = "default_for_cost")]
    pub r#for: i32,

    #[serde(default = "default_function_cost")]
    pub function: i32,

    #[serde(default = "default_class_cost")]
    pub class: i32,
}

fn default_true() -> bool { true }
fn default_variable_cost() -> i32 { 2 }
fn default_if_cost() -> i32 { 5 }
fn default_while_cost() -> i32 { 15 }
fn default_for_cost() -> i32 { 10 }
fn default_function_cost() -> i32 { 8 }
fn default_class_cost() -> i32 { 15 }

fn default_rules() -> BudgetRules {
    BudgetRules {
        variable: 2,
        r#if: 5,
        r#while: 15,
        r#for: 10,
        function: 8,
        class: 15,
    }
}

fn default_profiles() -> HashMap<String, ProfileConfig> {
    HashMap::from([
        ("strict".to_string(), ProfileConfig {
            max_budget: 80,
            description: "Simple utility functions".to_string(),
        }),
        ("class".to_string(), ProfileConfig {
            max_budget: 300,
            description: "Classes with methods".to_string(),
        }),
        ("service".to_string(), ProfileConfig {
            max_budget: 250,
            description: "Business logic and services".to_string(),
        }),
        ("test".to_string(), ProfileConfig {
            max_budget: 500,
            description: "Test files".to_string(),
        }),
    ])
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            profiles: default_profiles(),
            file_patterns: HashMap::new(),
            files: HashMap::new(),
            auto_detect: true,
            rules: default_rules(),
            bonuses: HashMap::from([
                ("ternary".to_string(), -5),
                ("tail_recursion".to_string(), -10),
                ("pattern_matching".to_string(), -3),
            ]),
            maluses: HashMap::new(),
            enforce_at_compile: false,
        }
    }
}

impl BudgetConfig {
    pub fn load() -> anyhow::Result<Self> {
        Self::load_from(".budgetrc.json")
    }

    pub fn load_from<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        let mut config: BudgetConfig = serde_json::from_str(&content)?;

        config.merge_defaults();

        Ok(config)
    }

    pub fn get_profile(&self, name: &str) -> Option<&ProfileConfig> {
        self.profiles.get(name)
    }

    pub fn get_bonus(&self, construct_name: &str) -> i32 {
        self.bonuses.get(construct_name).copied().unwrap_or(0)
    }

    pub fn get_malus(&self, construct_name: &str) -> i32 {
        self.maluses.get(construct_name).copied().unwrap_or(0)
    }

    fn merge_defaults(&mut self) {
        let defaults = Self::default();

        for (key, value) in defaults.profiles {
            self.profiles.entry(key).or_insert(value);
        }

        for (key, value) in defaults.bonuses {
            self.bonuses.entry(key).or_insert(value);
        }
    }
}
