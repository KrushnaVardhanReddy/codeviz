import React from 'react';

interface LegendProps {
  hiddenNodeKinds?: Set<string>;
  hiddenEdgeKinds?: Set<string>;
  onToggleNodeKind?: (kind: string) => void;
  onToggleEdgeKind?: (kind: string) => void;
  primaryMode?: 'structural' | 'execution';
}

// Which kinds are relevant in each mode
const EXECUTION_NODE_KINDS = new Set(['Function', 'Class', 'Module']);
const EXECUTION_EDGE_KINDS = new Set(['Calls']);

export const Legend: React.FC<LegendProps> = ({
  hiddenNodeKinds = new Set(),
  hiddenEdgeKinds = new Set(),
  onToggleNodeKind,
  onToggleEdgeKind,
  primaryMode = 'structural',
}) => {
  const isExecution = primaryMode === 'execution';

  const nodeKinds = [
    { kind: 'Class',     icon: <span className="text-orange-400">🔷</span>, label: 'Class' },
    { kind: 'Interface', icon: <span className="text-yellow-400">◇</span>,  label: 'Interface' },
    { kind: 'Function',  icon: <span className="text-green-400">ƒ</span>,   label: 'Function' },
    { kind: 'Module',    icon: <span className="text-purple-400">📦</span>, label: 'Module' },
    { kind: 'File',      icon: <span className="text-blue-400">📄</span>,   label: 'File' },
  ];

  const edgeKinds = [
    { kind: 'Imports',      icon: <span className="w-4 h-0.5 inline-block bg-blue-500 align-middle" />,                          label: 'Imports' },
    { kind: 'Calls',        icon: <span className="w-4 h-0.5 inline-block bg-green-500 align-middle" />,                         label: 'Calls' },
    { kind: 'Inherits',     icon: <span className="w-4 h-0.5 inline-block bg-orange-500 align-middle" />,                        label: 'Inherits' },
    { kind: 'Implements',   icon: <span className="w-4 inline-block border-t border-dashed border-yellow-500 align-middle" />,   label: 'Implements' },
    { kind: 'Returns',      icon: <span className="w-4 inline-block border-t border-dotted border-gray-400 align-middle" />,     label: 'Returns' },
    { kind: 'Instantiates', icon: <span className="w-4 h-0.5 inline-block bg-purple-500 align-middle" />,                       label: 'Instantiates' },
    { kind: 'Contains',     icon: <span className="w-4 inline-block border-t border-dashed border-gray-400 align-middle" />,     label: 'Contains' },
  ];

  const renderItem = (
    kind: string,
    icon: React.ReactNode,
    label: string,
    isHidden: boolean,
    isIrrelevant: boolean,
    onToggle?: (k: string) => void
  ) => {
    const canToggle = !!onToggle && !isIrrelevant;
    return (
      <span
        key={kind}
        title={isIrrelevant ? `Not used in Execution Flow mode` : isHidden ? `Click to show ${label}` : `Click to hide ${label}`}
        className={[
          'flex items-center gap-1 select-none transition-all duration-200',
          canToggle ? 'cursor-pointer hover:opacity-80' : isIrrelevant ? 'cursor-not-allowed' : '',
          isHidden ? 'opacity-25' : isIrrelevant ? 'opacity-20 grayscale' : 'opacity-100',
        ].join(' ')}
        onClick={() => canToggle && onToggle(kind)}
      >
        {icon}
        <span>{label}</span>
        {isIrrelevant && <span className="text-[9px] text-slate-500 ml-0.5">n/a</span>}
      </span>
    );
  };

  return (
    <div className="absolute bottom-4 left-4 right-4 bg-slate-900/80 backdrop-blur-md border border-slate-700/30 rounded p-3 text-xs text-slate-300 font-medium z-10 pointer-events-auto">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-2">
        {/* Node kinds */}
        <div className="flex flex-wrap gap-4">
          {nodeKinds.map(nk => {
            const irrelevant = isExecution && !EXECUTION_NODE_KINDS.has(nk.kind);
            return renderItem(nk.kind, nk.icon, nk.label, hiddenNodeKinds.has(nk.kind), irrelevant, onToggleNodeKind);
          })}
        </div>
        {/* Edge kinds */}
        <div className="flex flex-wrap gap-4 text-[10px] sm:text-xs">
          {edgeKinds.map(ek => {
            const irrelevant = isExecution && !EXECUTION_EDGE_KINDS.has(ek.kind);
            return renderItem(ek.kind, ek.icon, ek.label, hiddenEdgeKinds.has(ek.kind), irrelevant, onToggleEdgeKind);
          })}
        </div>
      </div>
      {isExecution && (
        <p className="text-[9px] text-slate-500 mt-1.5">
          Execution Flow only shows <span className="text-slate-400">Function / Class</span> nodes and <span className="text-slate-400">Calls</span> edges · other types marked <span className="text-slate-400">n/a</span>
        </p>
      )}
    </div>
  );
};
