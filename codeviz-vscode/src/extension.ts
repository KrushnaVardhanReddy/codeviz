import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import { GraphPanelProvider } from './graphPanel';
import { CodeVizStatusBar } from './statusBar';

let outputChannel: vscode.OutputChannel;
let statusBar: CodeVizStatusBar;
let graphPanelProvider: GraphPanelProvider;

export async function activate(context: vscode.ExtensionContext) {
    outputChannel = vscode.window.createOutputChannel('CodeViz');
    statusBar = new CodeVizStatusBar();

    // 1. ALWAYS register commands first (synchronously)
    graphPanelProvider = new GraphPanelProvider(context.extensionUri);
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(GraphPanelProvider.viewType, graphPanelProvider)
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('codeviz.showGraph', () => {
            vscode.commands.executeCommand('codeviz-explorer.focus');
        }),
        vscode.commands.registerCommand('codeviz.refreshGraph', () => {
            updateGraphForCurrentFile();
        }),
        vscode.commands.registerCommand('codeviz.openWebUi', () => {
            vscode.env.openExternal(vscode.Uri.parse('http://localhost:3000'));
        }),
        vscode.commands.registerCommand('codeviz.showOutput', () => {
            outputChannel.show();
        })
    );

    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((document) => {
            if (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document === document) {
                updateGraphForCurrentFile();
            }
        }),
        vscode.window.onDidChangeActiveTextEditor((editor) => {
            if (editor) {
                updateGraphForCurrentFile();
            }
        })
    );

    // 2. NOW do the workspace checks
    // Check for codeviz.toml
    if (!vscode.workspace.workspaceFolders || vscode.workspace.workspaceFolders.length === 0) {
        return;
    }

    const workspaceRoot = vscode.workspace.workspaceFolders[0].uri.fsPath;
    const configPath = path.join(workspaceRoot, 'codeviz.toml');

    if (!fs.existsSync(configPath)) {
        return;
    }

    // Initial update
    if (vscode.window.activeTextEditor) {
        updateGraphForCurrentFile();
    }
}

async function updateGraphForCurrentFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;

    const document = editor.document;
    if (document.isUntitled) return;

    const useWasm = vscode.workspace.getConfiguration('codeviz').get('useWasm', false);
    const forceCli = process.env.CODEVIZ_FORCE_CLI === '1';

    statusBar.setParsing();

    if (useWasm && !forceCli) {
        try {
            const codevizWasm = await import('codeviz');
            await codevizWasm.init();
            const ext = path.extname(document.fileName).substring(1);
            let lang = ext;
            if (ext === 'py') lang = 'python';
            else if (ext === 'js') lang = 'javascript';
            else if (ext === 'ts') lang = 'typescript';
            else if (ext === 'rs') lang = 'rust';
            else if (ext === 'go') lang = 'go';
            else if (ext === 'java') lang = 'java';

            const source = document.getText();
            const jsonGraph = codevizWasm.parse_to_json(source, lang);
            // wait, we can just use codevizWasm.parse to get mermaid string directly
            // index.ts exports: parse(source: string, language: string, diagram_kind: string): string
            const graphStr = codevizWasm.parse(source, lang, 'module');
            graphPanelProvider.updateGraph(graphStr);
            statusBar.setReady();
        } catch (error: any) {
            outputChannel.appendLine(`WASM Error: ${error}`);
            statusBar.setError();
        }
    } else {
        const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
        const cwd = workspaceFolder ? workspaceFolder.uri.fsPath : path.dirname(document.fileName);

        const tempOutputFile = path.join(cwd, '.codeviz_temp.md');
        fs.writeFileSync(tempOutputFile, '<!-- CODEVIZ_START -->\n<!-- CODEVIZ_END -->');

        cp.exec(`codeviz run --path . --diagram module --output .codeviz_temp.md`, { cwd }, (error, stdout, stderr) => {
            if (error) {
                outputChannel.appendLine(`Error: ${error.message}`);
                outputChannel.appendLine(stderr);
                statusBar.setError();
                if (fs.existsSync(tempOutputFile)) {
                    fs.unlinkSync(tempOutputFile);
                }
                return;
            }

            if (fs.existsSync(tempOutputFile)) {
                const markdown = fs.readFileSync(tempOutputFile, 'utf8');
                const match = markdown.match(/<!-- CODEVIZ_START -->\n```mermaid\n([\s\S]*?)\n```\n<!-- CODEVIZ_END -->/);

                if (match && match[1]) {
                    graphPanelProvider.updateGraph(match[1]);
                } else {
                    outputChannel.appendLine('Failed to extract Mermaid from temporary output file.');
                    statusBar.setError();
                }
                fs.unlinkSync(tempOutputFile);
            }
            statusBar.setReady();
        });
    }
}

export function deactivate() {
    if (statusBar) {
        statusBar.dispose();
    }
}
