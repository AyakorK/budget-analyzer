import * as path from 'path';
import * as vscode from 'vscode';
import * as fs from 'fs'
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;
let statusBarItem: vscode.StatusBarItem;
let terminalWarningItem: vscode.StatusBarItem;
let bypassedTask: vscode.Task | null = null;

export function activate(context: vscode.ExtensionContext) {
  console.log('Budget Analyzer extension activated');

  const serverPath = findServerBinary();

  if (!serverPath) {
    vscode.window.showErrorMessage(
      'Budget Analyzer LSP server not found. Please build it first: cd budget-lsp && cargo build --release'
    );
    return;
  }

  const serverOptions: ServerOptions = {
    run: { command: serverPath },
    debug: { command: serverPath },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'typescript' },
      { scheme: 'file', language: 'javascript' },
      { scheme: 'file', language: 'python' },
      { scheme: 'file', language: 'ruby' },
      { scheme: 'file', language: 'go' },
      { scheme: 'file', language: 'rust' },
      { scheme: 'file', language: 'blablalang'}
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{ts,js,py,rb,go,rs,bl}'),
    },
  };

  client = new LanguageClient(
    'budgetAnalyzer',
    'Budget Analyzer',
    serverOptions,
    clientOptions
  );

  client.start().then(() => {
    console.log('Budget Analyzer LSP ready');
  }).catch((err) => {
    vscode.window.showErrorMessage(`Budget Analyzer LSP failed: ${err.message}`);
  });

  statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBarItem.command = 'budget-analyzer.toggleEnforce';
  updateStatusBar();
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  terminalWarningItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 1000);
  context.subscriptions.push(terminalWarningItem);

  context.subscriptions.push(
    vscode.commands.registerCommand('budget-analyzer.analyze', async () => {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showInformationMessage('No active editor');
        return;
      }

      vscode.window.showInformationMessage(
        `Analyzing ${path.basename(editor.document.fileName)}...`
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('budget-analyzer.analyzeWorkspace', async () => {
      vscode.window.showInformationMessage('Analyzing workspace...');
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('budget-analyzer.toggleEnforce', async () => {
      const config = vscode.workspace.getConfiguration('budgetAnalyzer');
      const current = config.get<boolean>('enforceOnBuild', false);
      await config.update('enforceOnBuild', !current, vscode.ConfigurationTarget.Workspace);
      updateStatusBar();
      updateTerminalWarning();

      vscode.window.showInformationMessage(
        `Budget enforcement ${!current ? 'enabled' : 'disabled'}`
      );
    })
  );

  context.subscriptions.push(
    vscode.tasks.onDidStartTask(async (e) => {
      const config = vscode.workspace.getConfiguration('budgetAnalyzer');
      const enforceOnBuild = config.get<boolean>('enforceOnBuild', false);

      if (!enforceOnBuild || bypassedTask === e.execution.task) {
        bypassedTask = null;
        return;
      }

      const taskName = e.execution.task.name.toLowerCase();
      const isBuildTask = taskName.includes('build') ||
                          taskName.includes('start') ||
                          taskName.includes('run') ||
                          taskName.includes('dev');

      if (!isBuildTask) {
        return;
      }

      console.log('Budget check before build:', taskName);

      const hasExceeded = await checkWorkspaceBudget();

      if (hasExceeded) {
        e.execution.terminate();

        const choice = await vscode.window.showErrorMessage(
          '❌ Budget exceeded! Build blocked.',
          'Show Details',
          'Continue Anyway',
          'Disable Enforcement'
        );

        if (choice === 'Continue Anyway') {
          bypassedTask = e.execution.task;
          vscode.tasks.executeTask(e.execution.task);
        } else if (choice === 'Disable Enforcement') {
          await config.update('enforceOnBuild', false, vscode.ConfigurationTarget.Workspace);
          updateStatusBar();
          updateTerminalWarning();
          bypassedTask = e.execution.task;
          vscode.tasks.executeTask(e.execution.task);
        } else if (choice === 'Show Details') {
          vscode.commands.executeCommand('workbench.actions.view.problems');
        }
      }
    })
  );

  context.subscriptions.push(
    vscode.tasks.onDidEndTask((e) => {
      if (bypassedTask === e.execution.task) {
        bypassedTask = null;
      }
    })
  );

  context.subscriptions.push(
    vscode.window.onDidOpenTerminal(() => {
      updateTerminalWarning();

      const config = vscode.workspace.getConfiguration('budgetAnalyzer');
      if (config.get<boolean>('enforceOnBuild', false)) {
        vscode.window.showWarningMessage(
          '⚠️ Budget enforcement is ON. Terminal commands are NOT blocked. Use VSCode tasks/UI to run builds.',
          'Got it'
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('budgetAnalyzer.enforceOnBuild')) {
        updateTerminalWarning();
      }
    })
  );

  updateTerminalWarning();

  console.log('Budget Analyzer LSP client started');
}

export function deactivate(): Thenable<void> | undefined {
  if (statusBarItem) {
    statusBarItem.dispose();
  }
  if (terminalWarningItem) {
    terminalWarningItem.dispose();
  }
  if (!client) {
    return undefined;
  }
  return client.stop();
}

function updateStatusBar() {
  const config = vscode.workspace.getConfiguration('budgetAnalyzer');
  const enforceOnBuild = config.get<boolean>('enforceOnBuild', false);

  if (enforceOnBuild) {
    statusBarItem.text = '$(shield) Budget: ENFORCED';
    statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
    statusBarItem.tooltip = 'Build enforcement enabled (click to disable)';
  } else {
    statusBarItem.text = '$(circle-outline) Budget: Monitoring';
    statusBarItem.backgroundColor = undefined;
    statusBarItem.tooltip = 'Build enforcement disabled (click to enable)';
  }
}

function updateTerminalWarning() {
  const config = vscode.workspace.getConfiguration('budgetAnalyzer');
  const enforceOnBuild = config.get<boolean>('enforceOnBuild', false);

  if (enforceOnBuild) {
    terminalWarningItem.text = '$(warning) Budget enforcement ON - Use UI to run builds';
    terminalWarningItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
    terminalWarningItem.tooltip = 'Terminal commands are NOT blocked by budget enforcement. Use VSCode tasks/UI for protected builds.';
    terminalWarningItem.show();
  } else {
    terminalWarningItem.hide();
  }
}

async function checkWorkspaceBudget(): Promise<boolean> {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  if (!workspaceFolder) {
    return false;
  }

  const files = await vscode.workspace.findFiles(
    '**/*.{ts,js,py,rb,go,rs,bl}',
    '**/node_modules/**'
  );

  const exceededFiles: string[] = [];

  for (const file of files) {
    const diagnostics = vscode.languages.getDiagnostics(file);

    const hasBudgetError = diagnostics.some(d =>
      d.source === 'budget-analyzer' &&
      d.severity === vscode.DiagnosticSeverity.Error &&
      d.message.includes('EXCEEDED')
    );

    if (hasBudgetError) {
      exceededFiles.push(path.basename(file.fsPath));
    }
  }

  if (exceededFiles.length > 0) {
    vscode.window.showErrorMessage(
      `Budget exceeded in ${exceededFiles.length} file(s): ${exceededFiles.join(', ')}`
    );
    return true;
  }

  return false;
}

function findServerBinary(): string | null {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  if (!workspaceFolder) {
    return null;
  }

  const baseDir = workspaceFolder.uri.fsPath;

  let serverPath = path.join(baseDir, 'budget-lsp', 'target', 'release', 'budget-lsp');
  if (fs.existsSync(serverPath)) {
    return serverPath;
  }

  serverPath = path.join(baseDir, 'budget-lsp', 'target', 'debug', 'budget-lsp');
  if (fs.existsSync(serverPath)) {
    return serverPath;
  }

  return null;
}