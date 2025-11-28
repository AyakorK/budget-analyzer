import * as path from 'path';
import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClient(Options,
  ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  console.log('Budget Analyzer extension activated');

  // Path to LSP server binary
  const serverPath = findServerBinary();

  if (!serverPath) {
    vscode.window.showErrorMessage(
      'Budget Analyzer LSP server not found. Please build it first: cd budget-lsp && cargo build --release'
    );
    return;
  }

  // Server options
  const serverOptions: ServerOptions = {
    run: { command: serverPath },
    debug: { command: serverPath },
  };

  // Client options
  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'typescript' },
      { scheme: 'file', language: 'javascript' },
      { scheme: 'file', language: 'python' },
      { scheme: 'file', language: 'ruby' },
      { scheme: 'file', language: 'go' },
      { scheme: 'file', language: 'rust' },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{ts,js,py,rb,go,rs}'),
    },
  };

  // Create the language client
  client = new LanguageClient(
    'budgetAnalyzer',
    'Budget Analyzer',
    serverOptions,
    clientOptions
  );

  // Start the client
  client.start();

  // Register commands
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

  console.log('Budget Analyzer LSP client started');
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

function findServerBinary(): string | null {
  const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
  if (!workspaceFolder) {
    return null;
  }

  const baseDir = workspaceFolder.uri.fsPath;

  let serverPath = path.join(baseDir, 'budget-lsp', 'target', 'release', 'budget-lsp');
  if (require('fs').existsSync(serverPath)) {
    return serverPath;
  }

  serverPath = path.join(baseDir, 'budget-lsp', 'target', 'debug', 'budget-lsp');
  if (require('fs').existsSync(serverPath)) {
    return serverPath;
  }

  return null;
}
