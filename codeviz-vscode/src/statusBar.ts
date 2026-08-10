import * as vscode from 'vscode';

export class CodeVizStatusBar {
    private statusBarItem: vscode.StatusBarItem;

    constructor() {
        this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
        this.statusBarItem.command = 'codeviz.showOutput';
        this.setReady();
        this.statusBarItem.show();
    }

    public setReady() {
        this.statusBarItem.text = '$(check) CodeViz: Ready';
        this.statusBarItem.tooltip = 'CodeViz is ready';
    }

    public setParsing() {
        this.statusBarItem.text = '$(sync~spin) CodeViz: Parsing...';
        this.statusBarItem.tooltip = 'CodeViz is parsing the file';
    }

    public setError() {
        this.statusBarItem.text = '$(error) CodeViz: Error';
        this.statusBarItem.tooltip = 'CodeViz encountered an error. Click to view.';
    }

    public dispose() {
        this.statusBarItem.dispose();
    }
}
