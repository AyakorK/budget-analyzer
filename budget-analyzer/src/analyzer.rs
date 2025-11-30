use crate::config::BudgetConfig;
use crate::detection::{detect_quality_patterns, ConstructType, Detector, HybridDetector};
use crate::parsers::SupportedLanguage;
use crate::profiles::{CodeStats, Profile, ProfileDetector};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub file_path: PathBuf,
    pub language: String,
    pub profile: Profile,
    pub calculation: BudgetCalculation,
    pub max_budget: i32,
    pub exceeded: bool,
}

#[derive(Debug, Clone)]
pub struct BudgetCalculation {
    pub total: i32,
    pub breakdown: Vec<CostItem>,
}

#[derive(Debug, Clone)]
pub struct CostItem {
    pub kind: String,
    pub cost: i32,
    pub line: usize,
    pub description: String,
}

impl AnalysisResult {
    pub fn status(&self) -> &str {
        if self.exceeded {
            "EXCEEDED"
        } else {
            "OK"
        }
    }

    pub fn percentage(&self) -> f32 {
        (self.calculation.total as f32 / self.max_budget as f32) * 100.0
    }
}

pub struct Analyzer {
    config: BudgetConfig,
    detector: HybridDetector,
}

impl Analyzer {
    pub fn new() -> Result<Self> {
        let config = BudgetConfig::load().unwrap_or_default();
        Ok(Self {
            config,
            detector: HybridDetector::new(),
        })
    }

    pub fn with_config(config: BudgetConfig) -> Self {
        Self {
            config,
            detector: HybridDetector::new(),
        }
    }

    pub fn analyze_file(&self, file_path: &Path) -> Result<AnalysisResult> {
        let language =
            SupportedLanguage::from_path(file_path).context("Failed to detect language")?;

        let code = fs::read_to_string(file_path).context("Failed to read file")?;

        let mut parser = language.create_parser()?;
        let tree = parser
            .parse(&code, None)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse file"))?;

        let root = tree.root_node();
        let mut breakdown = Vec::new();
        let mut stats = CodeStats::new();
        let mut seen: HashMap<(usize, String), usize> = HashMap::new();

        self.traverse_node(
            &root,
            &code,
            &mut breakdown,
            &mut stats,
            &mut seen,
            language.as_str(),
        );

        let total: i32 = breakdown.iter().map(|item| item.cost).sum();
        let calculation = BudgetCalculation { total, breakdown };

        let profile_detector = ProfileDetector::new(&self.config);
        let profile = profile_detector.detect(file_path, &stats);
        let max_budget = self.get_max_budget(&profile);
        let exceeded = calculation.total > max_budget;

        Ok(AnalysisResult {
            file_path: file_path.to_path_buf(),
            language: language.as_str().to_string(),
            profile,
            calculation,
            max_budget,
            exceeded,
        })
    }

    fn traverse_node(
        &self,
        node: &Node,
        code: &str,
        breakdown: &mut Vec<CostItem>,
        stats: &mut CodeStats,
        seen: &mut HashMap<(usize, String), usize>,
        language: &str,
    ) {
        if let Some(construct_type) = self.detector.detect(node, code) {
            let line = node.start_position().row + 1;
            let kind = construct_type.as_str().to_string();

            let key = (line, kind.clone());
            let count = seen.entry(key.clone()).or_insert(0);
            *count += 1;

            if *count == 1 {
                self.update_stats(construct_type, stats);
                let cost = self.calculate_cost(construct_type);

                breakdown.push(CostItem {
                    kind,
                    cost,
                    line,
                    description: format!("{} at line {}", construct_type.as_str(), line),
                });
            }
        }

        // Quality patterns detection for this node
        let quality_items = detect_quality_patterns(node, code, language);
        for item in quality_items {
            let key = (item.line, item.kind.clone());
            let count = seen.entry(key.clone()).or_insert(0);
            *count += 1;

            if *count == 1 {
                breakdown.push(CostItem {
                    kind: item.kind,
                    cost: item.cost,
                    line: item.line,
                    description: format!("Quality pattern at line {}", item.line),
                });
            }
        }

        for child in node.children(&mut node.walk()) {
            self.traverse_node(&child, code, breakdown, stats, seen, language);
        }
    }

    fn update_stats(&self, construct_type: ConstructType, stats: &mut CodeStats) {
        match construct_type {
            ConstructType::Class => stats.class_count += 1,
            ConstructType::Function => stats.function_count += 1,
            ConstructType::While | ConstructType::For => stats.has_complex_logic = true,
            _ => {}
        }
    }

    fn calculate_cost(&self, construct_type: ConstructType) -> i32 {
        let base_cost = match construct_type {
            ConstructType::Variable => self.config.rules.variable,
            ConstructType::Function => self.config.rules.function,
            ConstructType::If => self.config.rules.r#if,
            ConstructType::While => self.config.rules.r#while,
            ConstructType::For => self.config.rules.r#for,
            ConstructType::Class => self.config.rules.class,
            ConstructType::Ternary => 0,
        };

        let bonus = self.config.get_bonus(construct_type.as_str());
        let malus = self.config.get_malus(construct_type.as_str());

        base_cost + bonus + malus
    }

    fn get_max_budget(&self, profile: &Profile) -> i32 {
        self.config
            .get_profile(profile.as_str())
            .map(|p| p.max_budget)
            .unwrap_or(80)
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self::with_config(BudgetConfig::default()))
    }
}
