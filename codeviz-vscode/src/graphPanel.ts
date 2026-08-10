import * as vscode from 'vscode';

export class GraphPanelProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'codeviz.graphView';
    private _view?: vscode.WebviewView;

    constructor(
        private readonly _extensionUri: vscode.Uri,
    ) { }

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        context: vscode.WebviewViewResolveContext,
        _token: vscode.CancellationToken,
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };

        webviewView.webview.html = this._getHtmlForWebview();
    }

    public updateGraph(mermaidData: string) {
        if (this._view) {
            this._view.webview.postMessage({ type: 'updateGraph', data: mermaidData });
        }
    }

    private _getHtmlForWebview() {
        return `<!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>CodeViz Graph</title>
                <script type="module">
                    import mermaid from 'https://cdnjs.cloudflare.com/ajax/libs/mermaid/10.9.1/mermaid.esm.min.mjs';
                    mermaid.initialize({ startOnLoad: true });

                    window.addEventListener('message', async event => {
                        const message = event.data;
                        if (message.type === 'updateGraph') {
                            const container = document.getElementById('graph-container');
                            if (container) {
                                container.innerHTML = '<div class="mermaid">' + message.data + '</div>';
                                // Re-run mermaid on the newly injected div
                                await mermaid.run();
                            }
                        }
                    });
                </script>
            </head>
            <body>
                <div id="graph-container">
                    <p>No graph available. Save a file to parse.</p>
                </div>
            </body>
            </html>`;
    }
}
