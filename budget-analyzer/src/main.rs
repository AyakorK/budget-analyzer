use anyhow::Result;
use budget_analyzer::Analyzer;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "budget")]
#[command(author = "BlaBlaLang Team")]
#[command(version = "0.1.0")]
#[command(about = "Universal budget analyzer with semantic detection", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        path: PathBuf,

        #[arg(short, long)]
        json: bool,
    },

    Check {
        path: PathBuf,
    },

    Config {
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, json } => {
            analyze_path(&path, json)?;
        }
        Commands::Check { path } => {
            check_path(&path)?;
        }
        Commands::Config { json } => {
            show_config(json)?;
        }
    }

    Ok(())
}

fn analyze_path(path: &PathBuf, json_output: bool) -> Result<()> {
    let analyzer = Analyzer::new()?;
    let mut results = Vec::new();

    if path.is_file() {
        match analyzer.analyze_file(path) {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("{} {}: {}", "⚠️".yellow(), path.display(), e),
        }
    } else if path.is_dir() {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                if matches!(
                    ext.to_str(),
                    Some("ts")
                        | Some("tsx")
                        | Some("js")
                        | Some("jsx")
                        | Some("py")
                        | Some("rb")
                        | Some("go")
                        | Some("rs")
                        | Some("bl")
                ) {
                    match analyzer.analyze_file(file_path) {
                        Ok(result) => results.push(result),
                        Err(e) => eprintln!("{} {}: {}", "⚠️".yellow(), file_path.display(), e),
                    }
                }
            }
        }
    }

    if json_output {
        print_json(&results)?;
    } else {
        print_results(&results);
    }

    Ok(())
}

fn check_path(path: &PathBuf) -> Result<()> {
    let analyzer = Analyzer::new()?;
    let mut exceeded_count = 0;
    let mut total_count = 0;

    if path.is_file() {
        let result = analyzer.analyze_file(path)?;
        total_count = 1;
        if result.exceeded {
            exceeded_count = 1;
        }
        print_result(&result);
    } else if path.is_dir() {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                if matches!(
                    ext.to_str(),
                    Some("ts")
                        | Some("tsx")
                        | Some("js")
                        | Some("jsx")
                        | Some("py")
                        | Some("rb")
                        | Some("go")
                        | Some("rs")
                        | Some("bl")
                ) {
                    match analyzer.analyze_file(file_path) {
                        Ok(result) => {
                            total_count += 1;
                            if result.exceeded {
                                exceeded_count += 1;
                            }
                            print_result(&result);
                        }
                        Err(e) => eprintln!("{} {}: {}", "⚠️".yellow(), file_path.display(), e),
                    }
                }
            }
        }
    }

    println!();
    println!(
        "Summary: {} OK, {} EXCEEDED",
        total_count - exceeded_count,
        exceeded_count
    );

    if exceeded_count > 0 {
        eprintln!("{}", "❌ Budget check failed!".red().bold());
        std::process::exit(1);
    }

    Ok(())
}

fn show_config(json_output: bool) -> Result<()> {
    let config = budget_analyzer::BudgetConfig::load().unwrap_or_default();

    if json_output {
        let json = serde_json::to_string_pretty(&config)?;
        println!("{}", json);
    } else {
        println!("{}", "Budget Configuration".bold().underline());
        println!();
        println!("{}", "Profiles:".bold());
        for (name, profile) in &config.profiles {
            println!(
                "  {} {} points - {}",
                name.cyan(),
                profile.max_budget.to_string().yellow(),
                profile.description.dimmed()
            );
        }
        println!();
        println!("{}", "Rules:".bold());
        println!("  Variable:  {} pts", config.rules.variable);
        println!("  If:        {} pts", config.rules.r#if);
        println!("  While:     {} pts", config.rules.r#while);
        println!("  For:       {} pts", config.rules.r#for);
        println!("  Function:  {} pts", config.rules.function);
        println!("  Class:     {} pts", config.rules.class);
    }

    Ok(())
}

fn print_result(result: &budget_analyzer::AnalysisResult) {
    let status = if result.exceeded {
        "❌".red()
    } else {
        "✅".green()
    };

    let profile = result.profile.as_str().cyan();
    let points = format!("{}/{}", result.calculation.total, result.max_budget);

    let points_colored = if result.exceeded {
        points.red().bold()
    } else {
        points.green()
    };

    println!(
        "{} {} [{}] ({}) {} pts ({:.1}%)",
        status,
        result.file_path.display(),
        result.language.dimmed(),
        profile,
        points_colored,
        result.percentage()
    );
}

fn print_results(results: &[budget_analyzer::AnalysisResult]) {
    if results.is_empty() {
        println!("{}", "No files found to analyze".yellow());
        return;
    }

    println!("{}", "Budget Analysis Results".bold().underline());
    println!();

    for result in results {
        print_result(result);
    }

    println!();

    let exceeded = results.iter().filter(|r| r.exceeded).count();
    let total = results.len();
    let ok = total - exceeded;

    println!("Summary: {} OK, {} EXCEEDED", ok, exceeded);
}

fn print_json(results: &[budget_analyzer::AnalysisResult]) -> Result<()> {
    let json_results: Vec<_> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "file": r.file_path.to_string_lossy(),
                "language": r.language,
                "profile": r.profile.as_str(),
                "total": r.calculation.total,
                "maxBudget": r.max_budget,
                "exceeded": r.exceeded,
                "percentage": r.percentage(),
            })
        })
        .collect();

    let output = serde_json::to_string_pretty(&json_results)?;
    println!("{}", output);

    Ok(())
}
