import React from 'react';
import { X } from 'lucide-react';
import CFGViewer from './CFGViewer';

interface DetailPanelProps {
  node: any | null;
  onClose: () => void;
  edges: any[];
}

const DetailPanel: React.FC<DetailPanelProps> = ({ node, onClose, edges }) => {
  const isOpen = node !== null;

  const connectedEdges = node
    ? edges.filter((e) => e.source === node.id || e.target === node.id)
    : [];

  return (
    <div
      className={`fixed top-0 right-0 h-full w-[350px] bg-gray-900/80 backdrop-blur-md border-l border-gray-700 shadow-2xl transition-transform duration-300 ease-in-out z-50 overflow-y-auto ${
        isOpen ? 'translate-x-0' : 'translate-x-full'
      }`}
    >
      {node && (
        <div className="p-4 flex flex-col h-full">
          <div className="flex items-center justify-between mb-4 border-b border-gray-700 pb-2">
            <div>
              <h2 className="text-lg font-bold text-white truncate max-w-[250px]">
                {node.data?.label || node.id}
              </h2>
              <span className="text-sm text-gray-400">
                Kind: {node.type || 'Unknown'}
              </span>
            </div>
            <button
              onClick={onClose}
              className="p-1 hover:bg-gray-800 rounded text-gray-400 hover:text-white transition-colors"
            >
              <X size={20} />
            </button>
          </div>

          <div className="mb-4">
            <h3 className="text-sm font-semibold text-gray-300 mb-2">Source Code Snippet</h3>
            <pre className="bg-gray-950 p-3 rounded-md text-xs font-mono text-gray-300 overflow-x-auto border border-gray-800">
              {node.data?.sourceSnippet || '// No snippet available'}
            </pre>
          </div>

          <div className="mb-4">
            <h3 className="text-sm font-semibold text-gray-300 mb-2">Control Flow Graph</h3>
            <CFGViewer sourceSnippet={node.data?.sourceSnippet} />
          </div>

          <div>
            <h3 className="text-sm font-semibold text-gray-300 mb-2">Connected Edges</h3>
            {connectedEdges.length === 0 ? (
              <p className="text-sm text-gray-500">No connections</p>
            ) : (
              <ul className="space-y-2">
                {connectedEdges.map((edge) => (
                  <li key={edge.id} className="text-xs bg-gray-800 p-2 rounded flex justify-between border border-gray-700">
                    <span className="text-gray-400">
                      {edge.source === node.id ? 'Out' : 'In'}:
                    </span>
                    <span className="font-mono text-gray-300 truncate max-w-[150px]">
                      {edge.source === node.id ? edge.target : edge.source}
                    </span>
                    <span className="text-gray-500 ml-1">({edge.data?.kind || 'Imports'})</span>
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
