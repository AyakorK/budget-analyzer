use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use budget_analyzer::Analyzer;
use std::path::PathBuf;
use std::collections::HashMap;

struct Backend {
    client: Client,
    analyzer: Analyzer,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            analyzer: Analyzer::default(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Budget Analyzer LSP Ultimate Edition started!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("📂 Opened: {}", params.text_document.uri))
            .await;

        self.analyze_and_publish(&params.text_document.uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.analyze_and_publish(&params.text_document.uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.analyze_and_publish(&params.text_document.uri).await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let path = PathBuf::from(uri.path());
        let position = params.text_document_position_params.position;

        match self.analyzer.analyze_file(&path) {
            Ok(result) => {
                if position.line == 0 {
                    let mut breakdown_by_type: HashMap<String, (i32, usize)> = HashMap::new();

                    for item in &result.calculation.breakdown {
                        let entry = breakdown_by_type.entry(item.kind.clone()).or_insert((0, 0));
                        entry.0 += item.cost;
                        entry.1 += 1;
                    }

                    let mut breakdown_text = String::new();
                    for (kind, (total_cost, count)) in breakdown_by_type.iter() {
                        breakdown_text.push_str(&format!("\n- **{}**: {}pts (×{})", kind, total_cost, count));
                    }

                    let status_emoji = if result.exceeded { "🔴" } else if result.percentage() > 80.0 { "🟡" } else { "🟢" };

                    let message = format!(
                        "# {} Budget Analysis\n\n\
                         **Language:** `{}`  \n\
                         **Profile:** `{}`  \n\
                         **Budget:** **{}/{}** pts  \n\
                         **Percentage:** {}%  \n\
                         **Status:** {}  \n\n\
                         ## Breakdown by construct type:\
                         {}\n\n\
                         ## Total constructs: {}\n\n\
                         {}",
                        status_emoji,
                        result.language,
                        result.profile.as_str(),
                        result.calculation.total,
                        result.max_budget,
                        format!("{:.1}", result.percentage()),
                        if result.exceeded { "❌ **EXCEEDED**" } else { "✅ **OK**" },
                        breakdown_text,
                        result.calculation.breakdown.len(),
                        if result.exceeded {
                            "💡 **Tip:** Consider splitting this into smaller functions or simplifying logic."
                        } else if result.percentage() > 80.0 {
                            "⚠️ **Warning:** Approaching budget limit. Consider refactoring soon."
                        } else {
                            "👍 **Good:** Budget is healthy!"
                        }
                    );

                    return Ok(Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: message,
                        }),
                        range: None,
                    }));
                } else {
                    let line_num = (position.line + 1) as usize;
                    let line_items: Vec<_> = result.calculation.breakdown.iter()
                        .filter(|item| item.line == line_num)
                        .collect();

                    if !line_items.is_empty() {
                        let mut details = format!("## Line {} Budget Breakdown\n\n", line_num);
                        let mut total = 0;

                        for item in &line_items {
                            total += item.cost;
                            let sign = if item.cost > 0 { "+" } else { "" };
                            details.push_str(&format!("- **{}**: {}{} pts\n", item.kind, sign, item.cost));
                        }

                        details.push_str(&format!("\n**Line Total:** {} pts", total));

                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: details,
                            }),
                            range: None,
                        }));
                    }
                }

                Ok(None)
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("❌ Analysis error: {}", e))
                    .await;
                Ok(None)
            }
        }
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = &params.text_document.uri;
        let path = PathBuf::from(uri.path());

        match self.analyzer.analyze_file(&path) {
            Ok(result) => {
                let mut hints = Vec::new();

                let mut line_info: HashMap<usize, (i32, Vec<String>)> = HashMap::new();

                for item in &result.calculation.breakdown {
                    let entry = line_info.entry(item.line).or_insert((0, Vec::new()));
                    entry.0 += item.cost;
                    entry.1.push(format!("{}{}", if item.cost > 0 { "+" } else { "" }, item.cost));
                }

                // Add hints for lines with significant cost
                for (line, (total_cost, _)) in line_info {
                    if total_cost.abs() >= 5 {
                        let sign = if total_cost > 0 { "+" } else { "" };
                        let hint_text = format!(" [{}{}pts]", sign, total_cost);

                        hints.push(InlayHint {
                            position: Position::new((line - 1) as u32, 200),
                            label: InlayHintLabel::String(hint_text),
                            kind: Some(InlayHintKind::TYPE),
                            text_edits: None,
                            tooltip: Some(InlayHintTooltip::String(format!(
                                "Line budget: {}{} pts (hover for details)",
                                if total_cost > 0 { "+" } else { "" },
                                total_cost
                            ))),
                            padding_left: Some(true),
                            padding_right: Some(false),
                            data: None,
                        });
                    }
                }

                let (status_emoji, status_text) = if result.exceeded {
                    ("🔴", "EXCEEDED")
                } else if result.percentage() > 80.0 {
                    ("🟡", "WARNING")
                } else {
                    ("🟢", "OK")
                };

                let budget_hint = format!(
                    " {} {}/{} pts ({}%) {} (hover for details)",
                    status_emoji,
                    result.calculation.total,
                    result.max_budget,
                    format!("{:.0}", result.percentage()),
                    status_text
                );

                hints.push(InlayHint {
                    position: Position::new(0, 200),
                    label: InlayHintLabel::String(budget_hint),
                    kind: Some(InlayHintKind::PARAMETER),
                    text_edits: None,
                    tooltip: Some(InlayHintTooltip::String(
                        "Hover on this line for full budget breakdown".to_string()
                    )),
                    padding_left: Some(true),
                    padding_right: Some(false),
                    data: None,
                });

                Ok(Some(hints))
            }
            Err(_) => Ok(None),
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let path = PathBuf::from(uri.path());

        match self.analyzer.analyze_file(&path) {
            Ok(result) => {
                let mut actions = Vec::new();

                if result.exceeded {
                    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: "💡 View Detailed Budget Breakdown".to_string(),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: Some(params.context.diagnostics.clone()),
                        edit: None,
                        command: Some(Command {
                            title: "Show Budget Details".to_string(),
                            command: "budget-analyzer.showDetails".to_string(),
                            arguments: None,
                        }),
                        is_preferred: Some(true),
                        disabled: None,
                        data: None,
                    }));
                }

                Ok(Some(actions))
            }
            Err(_) => Ok(None),
        }
    }
}

impl Backend {
    async fn analyze_and_publish(&self, uri: &Url) {
        let path = PathBuf::from(uri.path());

        match self.analyzer.analyze_file(&path) {
            Ok(result) => {
                let mut diagnostics = Vec::new();

                // Main budget diagnostic on line 0 only
                if result.exceeded {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position::new(0, 0),
                            end: Position::new(0, 50),  // Shorter range
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("budget-analyzer".to_string()),
                        message: format!(
                            "Budget EXCEEDED: {}/{} pts ({}%)",
                            result.calculation.total,
                            result.max_budget,
                            format!("{:.1}", result.percentage())
                        ),
                        code: Some(NumberOrString::String("budget-exceeded".to_string())),
                        ..Diagnostic::default()
                    });
                } else if result.percentage() > 80.0 {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position::new(0, 0),
                            end: Position::new(0, 50),
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("budget-analyzer".to_string()),
                        message: format!(
                            "Budget WARNING: {}/{} pts ({}%)",
                            result.calculation.total,
                            result.max_budget,
                            format!("{:.1}", result.percentage())
                        ),
                        code: Some(NumberOrString::String("budget-warning".to_string())),
                        ..Diagnostic::default()
                    });
                }

                // Individual construct diagnostics with tighter ranges
                for item in &result.calculation.breakdown {
                    let severity = if item.cost >= 15 {
                        Some(DiagnosticSeverity::WARNING)
                    } else if item.cost >= 10 {
                        Some(DiagnosticSeverity::INFORMATION)
                    } else if item.cost < 0 {
                        Some(DiagnosticSeverity::HINT)
                    } else {
                        None
                    };

                    if let Some(sev) = severity {
                        let message = if item.cost < 0 {
                            format!("Bonus: {} ({}pts)", item.kind, item.cost)
                        } else if item.cost >= 15 {
                            format!("High cost: {} ({}pts)", item.kind, item.cost)
                        } else {
                            format!("{}: {}pts", item.kind, item.cost)
                        };

                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position::new((item.line - 1) as u32, 4),
                                end: Position::new((item.line - 1) as u32, 80),
                            },
                            severity: Some(sev),
                            source: Some("budget-analyzer".to_string()),
                            message,
                            code: Some(NumberOrString::String(format!("budget-{}", item.kind))),
                            ..Diagnostic::default()
                        });
                    }
                }

                self.client
                    .publish_diagnostics(uri.clone(), diagnostics, None)
                    .await;

                let status_icon = if result.exceeded {
                    "🔴"
                } else if result.percentage() > 80.0 {
                    "🟡"
                } else {
                    "🟢"
                };

                self.client
                    .log_message(
                        MessageType::INFO,
                        format!(
                            "{} Budget: {}/{} pts ({}%)",
                            status_icon,
                            result.calculation.total,
                            result.max_budget,
                            format!("{:.0}", result.percentage())
                        ),
                    )
                    .await;
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("❌ Error: {}", e))
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
