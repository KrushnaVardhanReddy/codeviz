import React from 'react';

interface LegendProps {
  hiddenNodeKinds?: Set<string>;
  hiddenEdgeKinds?: Set<string>;
  onToggleNodeKind?: (kind: string) => void;
  onToggleEdgeKind?: (kind: string) => void;
}

export const Legend: React.FC<LegendProps> = ({
  hiddenNodeKinds = new Set(),
  hiddenEdgeKinds = new Set(),
  onToggleNodeKind,
  onToggleEdgeKind
}) => {
  const nodeKinds = [
    { kind: 'Class', icon: <span className="text-orange-500">🔷</span>, label: 'Class' },
    { kind: 'Interface', icon: <span className="text-yellow-500">◇</span>, label: 'Interface' },
    { kind: 'Function', icon: <span className="text-green-500">ƒ</span>, label: 'Function' },
    { kind: 'Module', icon: <span className="text-purple-500">📦</span>, label: 'Module' },
    { kind: 'File', icon: <span className="text-blue-500">📄</span>, label: 'File' },
  ];

  const edgeKinds = [
    { kind: 'Imports', icon: <span className="w-4 h-0.5 bg-blue-500"></span>, label: 'Imports' },
    { kind: 'Calls', icon: <span className="w-4 h-0.5 bg-green-500"></span>, label: 'Calls' },
    { kind: 'Inherits', icon: <span className="w-4 h-0.5 bg-orange-500"></span>, label: 'Inherits' },
    { kind: 'Implements', icon: <span className="w-4 border-t border-dashed border-yellow-500"></span>, label: 'Implements' },
    { kind: 'Returns', icon: <span className="w-4 border-t border-dotted border-gray-500"></span>, label: 'Returns' },
    { kind: 'Instantiates', icon: <span className="w-4 h-0.5 bg-purple-500"></span>, label: 'Instantiates' },
    { kind: 'Contains', icon: <span className="w-4 border-t border-dashed border-gray-400"></span>, label: 'Contains' },
  ];

  const renderItem = (kind: string, icon: React.ReactNode, label: string, isHidden: boolean, onToggle?: (k: string) => void) => (
    <span 
      key={kind}
      className={`flex items-center gap-1 transition-opacity select-none ${onToggle ? 'cursor-pointer hover:opacity-80' : ''} ${isHidden ? 'opacity-30' : 'opacity-100'}`}
      onClick={() => onToggle && onToggle(kind)}
    >
      {icon} {label}
    </span>
  );

  return (
    <div className="absolute bottom-4 left-4 right-4 bg-slate-900/80 backdrop-blur-md border border-slate-700/30 rounded p-3 text-xs text-slate-300 font-medium z-10 pointer-events-auto">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-2">
        <div className="flex flex-wrap gap-4">
          {nodeKinds.map(nk => renderItem(nk.kind, nk.icon, nk.label, hiddenNodeKinds.has(nk.kind), onToggleNodeKind))}
        </div>
        <div className="flex flex-wrap gap-4 text-[10px] sm:text-xs">
          {edgeKinds.map(ek => renderItem(ek.kind, ek.icon, ek.label, hiddenEdgeKinds.has(ek.kind), onToggleEdgeKind))}
        </div>
      </div>
    </div>
  );
};
