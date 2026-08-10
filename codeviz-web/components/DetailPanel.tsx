import React from 'react';
import { X } from 'lucide-react';
import CfgPanel from './CfgPanel';

interface DetailPanelProps {
  node: any | null;
  onClose: () => void;
  edges: any[];
  onTraceStart?: (nodeId: string) => void;
}

const DetailPanel: React.FC<DetailPanelProps> = ({ node, onClose, edges, onTraceStart }) => {
  const isOpen = node !== null;

  // Determine if node is a Function
  const isFunction = node && (
    (typeof node.data?.kind === 'object' && node.data?.kind !== null && 'Function' in node.data.kind) ||
    node.data?.kind === 'Function' ||
    node.type === 'Function'
  );

  const connectedEdges = node
    ? edges.filter((e) => e.source === node.id || e.target === node.id)
    : [];

  const kindStr = node?.data?.kind ? (typeof node.data.kind === 'string' ? node.data.kind : 'Function') : node?.type || 'Unknown';
  const isFunction = kindStr === 'Function' || kindStr === 'Async Fn' || (typeof node?.data?.kind === 'object' && 'Function' in node.data.kind);

  // Extract control_flow from node or node.data
  const controlFlow = node?.control_flow || node?.data?.control_flow;

  return (
    <div
      data-testid="detail-panel"
      className={`fixed top-0 right-0 h-full w-[350px] bg-slate-900/90 backdrop-blur-md border-l border-slate-700 shadow-2xl transition-transform duration-300 ease-in-out z-50 overflow-y-auto ${
        isOpen ? 'translate-x-0' : 'translate-x-full'
      }`}
    >
      {node && (
        <div className="p-4 flex flex-col h-full">
          <div className="flex items-center justify-between mb-4 border-b border-slate-700 pb-2">
            <div>
              <h2 className="text-lg font-bold text-white truncate max-w-[250px]" data-testid="detail-panel-title">
                {typeof node.data?.label === 'string' ? node.data?.label : (node.data?.label?.props?.children || node.id)}
              </h2>
              <span className="text-sm text-slate-400">
                Kind: {kindStr}
              </span>
            </div>
            <button
              onClick={onClose}
              data-testid="close-panel-btn"
              className="p-1 hover:bg-slate-800 rounded text-slate-400 hover:text-white transition-colors"
            >
              <X size={20} />
            </button>
          </div>

          {isFunction && onTraceStart && (
            <div className="mb-4">
              <button
                onClick={() => onTraceStart(node.id)}
                className="w-full bg-green-600 hover:bg-green-500 text-white font-semibold py-2 px-4 rounded-md transition-colors shadow-lg shadow-green-900/20 flex justify-center items-center gap-2"
                data-testid="trace-paths-btn"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>
                Trace Paths
              </button>
            </div>
          )}

          <div className="mb-4">
            <h3 className="text-sm font-semibold text-slate-300 mb-2">Source Code Snippet</h3>
            <pre className="bg-slate-950 p-3 rounded-md text-xs font-mono text-slate-300 overflow-x-auto border border-slate-800">
              {node.data?.sourceSnippet || '// No snippet available'}
            </pre>
          </div>

          {isFunction && (
            <div className="mb-4">
              <h3 className="text-sm font-semibold text-slate-300 mb-2">Control Flow Graph</h3>
              <CfgPanel controlFlow={controlFlow} />
            </div>
          )}

          <div>
            <h3 className="text-sm font-semibold text-slate-300 mb-2">Connected Edges</h3>
            {connectedEdges.length === 0 ? (
              <p className="text-sm text-slate-500">No connections</p>
            ) : (
              <ul className="space-y-2">
                {connectedEdges.map((edge) => (
                  <li key={edge.id} className="text-xs bg-slate-800 p-2 rounded flex justify-between border border-slate-700">
                    <span className="text-slate-400">
                      {edge.source === node.id ? 'Out' : 'In'}:
                    </span>
                    <span className="font-mono text-slate-300 truncate max-w-[150px]">
                      {edge.source === node.id ? edge.target : edge.source}
                    </span>
                    <span className="text-slate-500 ml-1">({edge.data?.kind || 'Imports'})</span>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default DetailPanel;
