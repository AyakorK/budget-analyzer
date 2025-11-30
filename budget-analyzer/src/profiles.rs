use crate::config::BudgetConfig;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Profile {
    Strict,
    Class,
    Service,
    Test,
    Custom(String),
}

impl Profile {
    pub fn as_str(&self) -> &str {
        match self {
            Profile::Strict => "strict",
            Profile::Class => "class",
            Profile::Service => "service",
            Profile::Test => "test",
            Profile::Custom(name) => name,
        }
    }
}

pub struct ProfileDetector<'a> {
    config: &'a BudgetConfig,
}

impl<'a> ProfileDetector<'a> {
    pub fn new(config: &'a BudgetConfig) -> Self {
        Self { config }
    }

    pub fn detect(&self, file_path: &Path, stats: &CodeStats) -> Profile {
        self.check_file_config(file_path)
            .or_else(|| self.check_file_patterns(file_path))
            .unwrap_or_else(|| {
                if self.config.auto_detect {
                    self.detect_from_stats(file_path, stats)
                } else {
                    Profile::Strict
                }
            })
    }

    fn check_file_config(&self, file_path: &Path) -> Option<Profile> {
        let path_str = file_path.to_string_lossy();
        self.config
            .files
            .get(path_str.as_ref())
            .map(|name| self.profile_from_string(name))
    }

    fn check_file_patterns(&self, file_path: &Path) -> Option<Profile> {
        let path_str = file_path.to_string_lossy();

        self.config
            .file_patterns
            .iter()
            .find(|(pattern, _)| self.matches_pattern(&path_str, pattern))
            .map(|(_, profile_name)| self.profile_from_string(profile_name))
    }

    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        if let Some(suffix) = pattern.strip_prefix("**/") {
            path.ends_with(suffix)
        } else if let Some(prefix) = pattern.strip_suffix('*') {
            path.starts_with(prefix)
        } else {
            path.contains(pattern)
        }
    }

    fn detect_from_stats(&self, file_path: &Path, stats: &CodeStats) -> Profile {
        let filename = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if filename.contains(".test.")
            || filename.contains(".spec.")
            || filename.starts_with("test_")
            || filename.ends_with("_test.")
        {
            return Profile::Test;
        }

        if stats.class_count > 0 {
            return Profile::Class;
        }

        if filename.contains(".utils.")
            || filename.contains("_utils.")
            || filename.contains("util")
            || filename.ends_with("_helpers.")
        {
            return Profile::Strict;
        }

        if filename.contains("Service")
            || filename.contains("service")
            || (stats.function_count >= 5 && stats.has_complex_logic)
        {
            return Profile::Service;
        }

        if stats.function_count > 0 {
            Profile::Strict
        } else {
            Profile::Strict
        }
    }

    fn profile_from_string(&self, name: &str) -> Profile {
        match name {
            "strict" => Profile::Strict,
            "class" => Profile::Class,
            "service" => Profile::Service,
            "test" => Profile::Test,
            custom => Profile::Custom(custom.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeStats {
    pub class_count: usize,
    pub function_count: usize,
    pub has_complex_logic: bool,
}

impl CodeStats {
    pub fn new() -> Self {
        Self {
            class_count: 0,
            function_count: 0,
            has_complex_logic: false,
        }
    }
}
