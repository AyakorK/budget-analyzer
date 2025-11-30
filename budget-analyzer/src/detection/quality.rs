use std::collections::HashMap;
use tree_sitter::Node;

#[derive(Clone)]
struct PatternMatcher {
    strategies: Vec<MatchStrategy>,
}

#[derive(Clone)]
enum MatchStrategy {
    CallExpression {
        node_type: &'static str,
        field: &'static str,
        keywords: Vec<&'static str>,
    },
    Identifier {
        keywords: Vec<&'static str>,
    },
    MacroInvocation {
        keywords: Vec<&'static str>,
    },
}

#[derive(Clone)]
struct LanguageQualityConfig {
    debug_print_matchers: PatternMatcher,
    logger_matchers: PatternMatcher,
    comment_node: &'static str,
    try_node: Option<&'static str>,
    doc_comment_prefix: &'static str,
}

lazy_static::lazy_static! {
    static ref QUALITY_CONFIGS: HashMap<&'static str, LanguageQualityConfig> = {
        let mut map = HashMap::new();

        // TypeScript / JavaScript
        let ts_config = LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call_expression",
                        field: "function",
                        keywords: vec!["console.log", "console.debug", "console.warn"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call_expression",
                        field: "function",
                        keywords: vec!["logger.error", "logger.warn", ".error", ".warn"],
                    },
                ],
            },
            comment_node: "comment",
            try_node: Some("try_statement"),
            doc_comment_prefix: "/**",
        };
        map.insert("TypeScript", ts_config.clone());
        map.insert("JavaScript", ts_config);

        // Python
        map.insert("Python", LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call",
                        field: "function",
                        keywords: vec!["print"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call",
                        field: "function",
                        keywords: vec!["logging.error", "logging.warning", ".error", ".warning"],
                    },
                ],
            },
            comment_node: "comment",
            try_node: Some("try_statement"),
            doc_comment_prefix: "\"\"\"",
        });

        // Ruby
        map.insert("Ruby", LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call",
                        field: "method",
                        keywords: vec!["puts", "p", "pp", "print"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call",
                        field: "method",
                        keywords: vec!["logger.error", "logger.warn"],
                    },
                ],
            },
            comment_node: "comment",
            try_node: Some("begin"),
            doc_comment_prefix: "#",
        });

        // Go (MULTI-STRATEGY!)
        map.insert("Go", LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call_expression",
                        field: "function",
                        keywords: vec!["fmt.Println", "fmt.Printf", "log.Println"],
                    },
                    MatchStrategy::Identifier {
                        keywords: vec!["println", "print"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "call_expression",
                        field: "function",
                        keywords: vec!["log.Error", "log.Warn"],
                    },
                ],
            },
            comment_node: "comment",
            try_node: None,
            doc_comment_prefix: "//",
        });

        // Rust (MACRO STRATEGY!)
        map.insert("Rust", LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::MacroInvocation {
                        keywords: vec!["println!", "dbg!", "print!", "eprintln!"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::MacroInvocation {
                        keywords: vec!["error!", "warn!", "info!"],
                    },
                ],
            },
            comment_node: "line_comment",
            try_node: Some("match_expression"),
            doc_comment_prefix: "///",
        });

        map.insert("Java", LanguageQualityConfig {
            debug_print_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "method_invocation",
                        field: "name",
                        keywords: vec!["println", "print"],
                    },
                ],
            },
            logger_matchers: PatternMatcher {
                strategies: vec![
                    MatchStrategy::CallExpression {
                        node_type: "method_invocation",
                        field: "object",
                        keywords: vec!["logger", "LOG"],
                    },
                ],
            },
            comment_node: "comment",
            try_node: Some("try_statement"),
            doc_comment_prefix: "/**",
        });

        map
    };
}

pub fn detect_quality_patterns(node: &Node, source: &str, language: &str) -> Vec<BudgetItem> {
    let mut items = Vec::new();

    let config = match QUALITY_CONFIGS.get(language) {
        Some(c) => c,
        None => return items,
    };

    // Debug prints detection
    if let Some(item) =
        check_pattern_matcher(node, source, &config.debug_print_matchers, "console_log", 3)
    {
        items.push(item);
    }

    // Structured loggers detection
    if let Some((kind, cost)) = check_logger_matcher(node, source, &config.logger_matchers) {
        items.push(BudgetItem {
            kind,
            cost,
            line: node.start_position().row + 1,
        });
    }

    // Comments
    if node.kind() == config.comment_node {
        items.extend(check_comments(node, source, config.doc_comment_prefix));
    }

    // Try/catch
    if let Some(try_node) = config.try_node {
        if node.kind() == try_node {
            items.push(BudgetItem {
                kind: "try_catch".to_string(),
                cost: -2,
                line: node.start_position().row + 1,
            });
        }
    }

    items
}

fn check_pattern_matcher(
    node: &Node,
    source: &str,
    matcher: &PatternMatcher,
    result_kind: &str,
    cost: i32,
) -> Option<BudgetItem> {
    for strategy in &matcher.strategies {
        match strategy {
            MatchStrategy::CallExpression {
                node_type,
                field,
                keywords,
            } => {
                if node.kind() == *node_type {
                    if let Some(func_node) = node.child_by_field_name(field) {
                        let func_text = func_node.utf8_text(source.as_bytes()).ok()?;
                        for keyword in keywords {
                            if func_text == *keyword
                                || func_text.ends_with(keyword)
                                || func_text.contains(keyword)
                            {
                                return Some(BudgetItem {
                                    kind: result_kind.to_string(),
                                    cost,
                                    line: node.start_position().row + 1,
                                });
                            }
                        }
                    }
                }
            }
            MatchStrategy::Identifier { keywords } => {
                if node.kind() == "identifier" {
                    let text = node.utf8_text(source.as_bytes()).ok()?;
                    for keyword in keywords {
                        if text == *keyword {
                            return Some(BudgetItem {
                                kind: result_kind.to_string(),
                                cost,
                                line: node.start_position().row + 1,
                            });
                        }
                    }
                }
            }
            MatchStrategy::MacroInvocation { keywords } => {
                if node.kind() == "macro_invocation" {
                    if let Some(macro_node) = node.child(0) {
                        let macro_text = macro_node.utf8_text(source.as_bytes()).ok()?;
                        for keyword in keywords {
                            if macro_text == *keyword {
                                return Some(BudgetItem {
                                    kind: result_kind.to_string(),
                                    cost,
                                    line: node.start_position().row + 1,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn check_logger_matcher(
    node: &Node,
    source: &str,
    matcher: &PatternMatcher,
) -> Option<(String, i32)> {
    for strategy in &matcher.strategies {
        match strategy {
            MatchStrategy::CallExpression {
                node_type,
                field,
                keywords,
            } => {
                if node.kind() == *node_type {
                    if let Some(func_node) = node.child_by_field_name(field) {
                        let func_text = func_node.utf8_text(source.as_bytes()).ok()?;
                        for keyword in keywords {
                            if func_text.contains(keyword) {
                                let (kind, cost) = if keyword.contains("error") {
                                    ("logger_error", -2)
                                } else if keyword.contains("warn") {
                                    ("logger_warn", -1)
                                } else {
                                    ("logger_info", -1)
                                };
                                return Some((kind.to_string(), cost));
                            }
                        }
                    }
                }
            }
            MatchStrategy::MacroInvocation { keywords } => {
                if node.kind() == "macro_invocation" {
                    if let Some(macro_node) = node.child(0) {
                        let macro_text = macro_node.utf8_text(source.as_bytes()).ok()?;
                        for keyword in keywords {
                            if macro_text == *keyword {
                                let (kind, cost) = if keyword.contains("error") {
                                    ("logger_error", -2)
                                } else if keyword.contains("warn") {
                                    ("logger_warn", -1)
                                } else {
                                    ("logger_info", -1)
                                };
                                return Some((kind.to_string(), cost));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn check_comments(node: &Node, source: &str, doc_prefix: &str) -> Vec<BudgetItem> {
    let mut items = Vec::new();
    let text = node.utf8_text(source.as_bytes()).unwrap_or("");

    if text.contains("TODO") {
        items.push(BudgetItem {
            kind: "todo_comment".to_string(),
            cost: 2,
            line: node.start_position().row + 1,
        });
    }

    if text.contains("FIXME") {
        items.push(BudgetItem {
            kind: "fixme_comment".to_string(),
            cost: 3,
            line: node.start_position().row + 1,
        });
    }

    if text.contains("HACK") {
        items.push(BudgetItem {
            kind: "hack_comment".to_string(),
            cost: 4,
            line: node.start_position().row + 1,
        });
    }

    if text.trim_start().starts_with(doc_prefix)
        && !text.contains("TODO")
        && !text.contains("FIXME")
    {
        items.push(BudgetItem {
            kind: "doc_comment".to_string(),
            cost: -1,
            line: node.start_position().row + 1,
        });
    }

    items
}

pub struct BudgetItem {
    pub kind: String,
    pub cost: i32,
    pub line: usize,
}
