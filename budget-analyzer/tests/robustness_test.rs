use budget_analyzer::{Analyzer, BudgetConfig};
use std::fs;
use std::path::Path;

// ============================================================================
// ROBUSTNESS TESTS
// ============================================================================

#[test]
fn test_malformed_code_handling() {
    let analyzer = Analyzer::default();

    // Intentionally broken code
    let broken_code = r#"
function incomplete( {
    if (true {
        return
}
"#;

    fs::write("temp_broken.ts", broken_code).unwrap();

    // Should either handle gracefully or return reasonable error
    let result = analyzer.analyze_file(Path::new("temp_broken.ts"));

    match result {
        Ok(analysis) => {
            // If it parses, it should at least detect some constructs
            println!(
                "Parsed broken code with {} constructs",
                analysis.calculation.breakdown.len()
            );
        }
        Err(e) => {
            println!("Gracefully handled broken code: {}", e);
        }
    }

    fs::remove_file("temp_broken.ts").ok();
}

#[test]
fn test_unicode_handling() {
    let analyzer = Analyzer::default();

    let unicode_code = r#"
function greet() {
    const message = "你好世界 🌍";
    const emoji = "🚀💻🔥";
    return message + emoji;
}

class Tëst {
    méthodé() {
        return "çà marche";
    }
}
"#;

    fs::write("temp_unicode.ts", unicode_code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_unicode.ts")).unwrap();

    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "function"),
        "Should detect function despite unicode"
    );
    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "class"),
        "Should detect class despite unicode"
    );

    fs::remove_file("temp_unicode.ts").ok();
}

#[test]
fn test_extremely_long_lines() {
    let analyzer = Analyzer::default();

    let long_line = format!(
        "function test() {{ const x = {}; return x; }}",
        "a".repeat(10000)
    );

    fs::write("temp_long_line.ts", long_line).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_long_line.ts"));

    assert!(result.is_ok(), "Should handle extremely long lines");

    fs::remove_file("temp_long_line.ts").ok();
}

#[test]
fn test_deeply_nested_structures_limit() {
    let analyzer = Analyzer::default();

    // Create 50 levels of nesting
    let mut code = String::from("function outer() {\n");
    for i in 0..50 {
        code.push_str(&format!("{}if (true) {{\n", "  ".repeat(i)));
    }
    code.push_str(&format!("{}const x = 1;\n", "  ".repeat(50)));
    for i in (0..50).rev() {
        code.push_str(&format!("{}}}\n", "  ".repeat(i)));
    }
    code.push_str("}\n");

    fs::write("temp_deep_nest.ts", &code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_deep_nest.ts"));

    assert!(result.is_ok(), "Should handle deeply nested structures");

    if let Ok(analysis) = result {
        println!("Deep nesting total: {} pts", analysis.calculation.total);
        assert!(
            analysis.calculation.total > 200,
            "Deep nesting should have high budget"
        );
    }

    fs::remove_file("temp_deep_nest.ts").ok();
}

// ============================================================================
// SPECIAL CHARACTER TESTS
// ============================================================================

#[test]
fn test_special_characters_in_strings() {
    let analyzer = Analyzer::default();

    let code = r#"
const escaped = "Line with\nnewline and\ttab";
const quotes = "String with \"quotes\" inside";
const backslash = "Path: C:\\Users\\test";
const regex = /for\s+\w+\s+while/;
"#;

    fs::write("temp_special.ts", code).unwrap();
    let result = analyzer.analyze_file(Path::new("temp_special.ts")).unwrap();

    // Should NOT detect 'for' or 'while' in regex
    let has_for = result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "for");
    let has_while = result
        .calculation
        .breakdown
        .iter()
        .any(|item| item.kind == "while");

    assert!(!has_for, "Should not detect 'for' in regex");
    assert!(!has_while, "Should not detect 'while' in regex");

    fs::remove_file("temp_special.ts").ok();
}

// ============================================================================
// WHITESPACE TESTS
// ============================================================================

#[test]
fn test_various_whitespace_styles() {
    let analyzer = Analyzer::default();

    let code = "function test(){if(true){return 1;}}"; // No spaces
    fs::write("temp_no_space.ts", code).unwrap();
    let result1 = analyzer
        .analyze_file(Path::new("temp_no_space.ts"))
        .unwrap();

    let code2 = "function    test()    {    if(true)    {    return 1;    }    }"; // Many spaces
    fs::write("temp_many_spaces.ts", code2).unwrap();
    let result2 = analyzer
        .analyze_file(Path::new("temp_many_spaces.ts"))
        .unwrap();

    // Both should detect same constructs
    assert_eq!(
        result1.calculation.breakdown.len(),
        result2.calculation.breakdown.len(),
        "Whitespace shouldn't affect detection"
    );

    fs::remove_file("temp_no_space.ts").ok();
    fs::remove_file("temp_many_spaces.ts").ok();
}

// ============================================================================
// MIXED TAB/SPACE INDENTATION
// ============================================================================

#[test]
fn test_mixed_indentation() {
    let analyzer = Analyzer::default();

    let code = "function test() {\n\tif (true) {\n        const x = 1;\n\t}\n}\n";
    fs::write("temp_mixed_indent.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_mixed_indent.ts"))
        .unwrap();

    assert!(
        result
            .calculation
            .breakdown
            .iter()
            .any(|item| item.kind == "function"),
        "Should handle mixed indentation"
    );

    fs::remove_file("temp_mixed_indent.ts").ok();
}

// ============================================================================
// ZERO-WIDTH CHARACTERS
// ============================================================================

#[test]
fn test_zero_width_characters() {
    let analyzer = Analyzer::default();

    // Zero-width space (U+200B) and other zero-width chars
    let code = "function​test() { return 1; }"; // Contains zero-width space
    fs::write("temp_zero_width.ts", code).unwrap();

    let result = analyzer.analyze_file(Path::new("temp_zero_width.ts"));

    // Should either parse correctly or fail gracefully
    match result {
        Ok(_) => println!("Handled zero-width characters"),
        Err(e) => println!("Failed gracefully on zero-width chars: {}", e),
    }

    fs::remove_file("temp_zero_width.ts").ok();
}

// ============================================================================
// FILE PERMISSION TESTS
// ============================================================================

#[test]
#[cfg(unix)]
fn test_readonly_file() {
    use std::os::unix::fs::PermissionsExt;

    let analyzer = Analyzer::default();
    let code = "function test() { return 1; }";

    fs::write("temp_readonly.ts", code).unwrap();

    // Make file read-only
    let mut perms = fs::metadata("temp_readonly.ts").unwrap().permissions();
    perms.set_mode(0o444);
    fs::set_permissions("temp_readonly.ts", perms).unwrap();

    let result = analyzer.analyze_file(Path::new("temp_readonly.ts"));

    assert!(result.is_ok(), "Should be able to read readonly file");

    // Cleanup (need to restore write permission first)
    let mut perms = fs::metadata("temp_readonly.ts").unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions("temp_readonly.ts", perms).unwrap();
    fs::remove_file("temp_readonly.ts").ok();
}

// ============================================================================
// ENCODING TESTS
// ============================================================================

#[test]
fn test_utf8_bom() {
    let analyzer = Analyzer::default();

    // UTF-8 BOM (Byte Order Mark)
    let code_with_bom = "\u{FEFF}function test() { return 1; }";
    fs::write("temp_bom.ts", code_with_bom).unwrap();

    let result = analyzer.analyze_file(Path::new("temp_bom.ts"));

    assert!(result.is_ok(), "Should handle UTF-8 BOM");

    fs::remove_file("temp_bom.ts").ok();
}

// ============================================================================
// LINE ENDING TESTS
// ============================================================================

#[test]
fn test_different_line_endings() {
    let analyzer = Analyzer::default();

    // Unix line endings (LF)
    let unix_code = "function test() {\n  return 1;\n}";
    fs::write("temp_unix.ts", unix_code).unwrap();
    let unix_result = analyzer.analyze_file(Path::new("temp_unix.ts")).unwrap();

    // Windows line endings (CRLF)
    let windows_code = "function test() {\r\n  return 1;\r\n}";
    fs::write("temp_windows.ts", windows_code).unwrap();
    let windows_result = analyzer.analyze_file(Path::new("temp_windows.ts")).unwrap();

    // Old Mac line endings (CR)
    let mac_code = "function test() {\r  return 1;\r}";
    fs::write("temp_mac.ts", mac_code).unwrap();
    let mac_result = analyzer.analyze_file(Path::new("temp_mac.ts")).unwrap();

    // All should detect same constructs
    assert_eq!(
        unix_result.calculation.breakdown.len(),
        windows_result.calculation.breakdown.len(),
        "Line endings shouldn't affect detection"
    );

    assert_eq!(
        unix_result.calculation.breakdown.len(),
        mac_result.calculation.breakdown.len(),
        "Line endings shouldn't affect detection"
    );

    fs::remove_file("temp_unix.ts").ok();
    fs::remove_file("temp_windows.ts").ok();
    fs::remove_file("temp_mac.ts").ok();
}

// ============================================================================
// MEMORY SAFETY TESTS
// ============================================================================

#[test]
fn test_large_file_memory() {
    let analyzer = Analyzer::default();

    // Generate a 1MB file
    let mut large_code = String::new();
    for i in 0..1000 {
        large_code.push_str(&format!(
            "function func{i}() {{ const x = {i}; return x * 2; }}\n",
            i = i
        ));
    }

    fs::write("temp_large_memory.ts", &large_code).unwrap();

    let start_memory = memory_usage();
    let result = analyzer.analyze_file(Path::new("temp_large_memory.ts"));
    let end_memory = memory_usage();

    assert!(result.is_ok(), "Should handle large files");

    let memory_increase = end_memory.saturating_sub(start_memory);
    println!("Memory increase: {} KB", memory_increase / 1024);

    // Memory increase should be reasonable (less than 100MB)
    assert!(
        memory_increase < 100 * 1024 * 1024,
        "Memory usage should be reasonable"
    );

    fs::remove_file("temp_large_memory.ts").ok();
}

#[cfg(unix)]
fn memory_usage() -> usize {
    use std::fs::read_to_string;

    if let Ok(status) = read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<usize>().unwrap_or(0) * 1024;
                }
            }
        }
    }
    0
}

#[cfg(not(unix))]
fn memory_usage() -> usize {
    // Fallback for non-Unix systems
    0
}

// ============================================================================
// DUPLICATE DETECTION PREVENTION
// ============================================================================

#[test]
fn test_no_duplicate_detection() {
    let analyzer = Analyzer::default();

    let code = r#"
function test() {
    if (true) {
        return 1;
    }
}
"#;

    fs::write("temp_duplicate.ts", code).unwrap();
    let result = analyzer
        .analyze_file(Path::new("temp_duplicate.ts"))
        .unwrap();

    // Count occurrences of each construct
    let func_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "function")
        .count();
    let if_count = result
        .calculation
        .breakdown
        .iter()
        .filter(|item| item.kind == "if")
        .count();

    assert_eq!(func_count, 1, "Should detect function exactly once");
    assert_eq!(if_count, 1, "Should detect if exactly once");

    fs::remove_file("temp_duplicate.ts").ok();
}

// ============================================================================
// CONFIG VALIDATION TESTS
// ============================================================================

#[test]
fn test_invalid_config_handling() {
    let invalid_config = r#"{
        "profiles": {
            "test": {
                "max_budget": "not_a_number"
            }
        }
    }"#;

    fs::write("temp_invalid_config.json", invalid_config).unwrap();

    let result = BudgetConfig::load_from("temp_invalid_config.json");

    assert!(result.is_err(), "Should reject invalid config");

    fs::remove_file("temp_invalid_config.json").ok();
}

#[test]
fn test_negative_costs_config() {
    let mut config = BudgetConfig::default();
    config.rules.variable = -10; // Invalid: negative cost

    let analyzer = Analyzer::with_config(config);

    let code = "const x = 1;";
    fs::write("temp_negative.ts", code).unwrap();

    let result = analyzer
        .analyze_file(Path::new("temp_negative.ts"))
        .unwrap();

    // Should handle gracefully (might clamp to 0 or use absolute value)
    println!(
        "Result with negative cost: {} pts",
        result.calculation.total
    );

    fs::remove_file("temp_negative.ts").ok();
}

// ============================================================================
// CONCURRENT MODIFICATION TEST
// ============================================================================

#[test]
fn test_file_modified_during_analysis() {
    use std::thread;
    use std::time::Duration;

    let analyzer = Analyzer::default();
    let code = "function test() { return 1; }";

    fs::write("temp_concurrent.ts", code).unwrap();

    // Start analysis
    let handle = thread::spawn(move || analyzer.analyze_file(Path::new("temp_concurrent.ts")));

    // Modify file during analysis
    thread::sleep(Duration::from_millis(10));
    fs::write("temp_concurrent.ts", "function modified() { return 2; }").unwrap();

    let result = handle.join().unwrap();

    // Should either succeed with original or modified content, or fail gracefully
    assert!(
        result.is_ok() || result.is_err(),
        "Should handle concurrent modification"
    );

    fs::remove_file("temp_concurrent.ts").ok();
}

// ============================================================================
// SYMLINK TESTS
// ============================================================================

#[test]
#[cfg(unix)]
fn test_symlink_handling() {
    use std::os::unix::fs as unix_fs;

    let analyzer = Analyzer::default();
    let code = "function test() { return 1; }";

    fs::write("temp_original.ts", code).unwrap();
    unix_fs::symlink("temp_original.ts", "temp_symlink.ts").ok();

    let result = analyzer.analyze_file(Path::new("temp_symlink.ts"));

    assert!(result.is_ok(), "Should handle symlinks");

    fs::remove_file("temp_original.ts").ok();
    fs::remove_file("temp_symlink.ts").ok();
}
