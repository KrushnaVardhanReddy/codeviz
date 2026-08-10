import React, { useMemo } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  Node as ReactFlowNode,
  Edge as ReactFlowEdge,
  BackgroundVariant
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { ControlFlowGraph } from '../lib/graphTypes';
import { CfgBlockNode } from './nodes/CfgBlockNode';
import { getCfgEdgeStyle } from '../lib/cfgColorMap';

interface CfgPanelProps {
  controlFlow?: ControlFlowGraph;
}

const nodeTypes = {
  cfgBlock: CfgBlockNode,
};

export const CfgPanel: React.FC<CfgPanelProps> = ({ controlFlow }) => {
  const nodes: ReactFlowNode[] = useMemo(() => {
    if (!controlFlow || !controlFlow.blocks) return [];
    // Basic vertical layout strategy
    return controlFlow.blocks.map((block, index) => {
      // 100px vertical spacing + 120px offset
      return {
        id: block.id,
        type: 'cfgBlock',
        position: { x: 200, y: 50 + index * 120 },
        data: {
          label: block.label,
          kind: block.kind,
          line: block.line
        }
      };
    });
  }, [controlFlow]);

  const edges: ReactFlowEdge[] = useMemo(() => {
    if (!controlFlow || !controlFlow.cfg_edges) return [];
    return controlFlow.cfg_edges.map((edge, index) => {
      const style = getCfgEdgeStyle(edge.kind);

      let label = edge.label || undefined;
      if (edge.kind === 'TrueBranch') label = '✓ true';
      if (edge.kind === 'FalseBranch') label = '✗ false';

      const isLoopBack = edge.kind === 'LoopBack';

      return {
        id: `cfg-e-${index}-${edge.from_id}-${edge.to_id}`,
        source: edge.from_id,
        target: edge.to_id,
        label,
        type: isLoopBack ? 'straight' : 'smoothstep',
        style: {
          stroke: style.color,
          strokeWidth: 2,
          strokeDasharray: style.style.includes('Dashed') ? '5 5' : style.style.includes('Dotted') ? '2 2' : undefined,
        },
        labelStyle: { fill: style.color, fontWeight: 700, fontSize: 12 },
        labelBgStyle: { fill: '#1F2937' },
        animated: edge.kind === 'AsyncEdge', // async suspension gets wavy/animated
        sourceHandle: edge.kind === 'TrueBranch' ? 'left' : edge.kind === 'FalseBranch' ? 'right' : undefined,
      };
    });
  }, [controlFlow]);

  if (!controlFlow || !controlFlow.blocks || controlFlow.blocks.length === 0) {
    return (
      <div className="bg-gray-800/50 border border-gray-700 border-dashed rounded-md p-4 text-center text-sm text-gray-400">
        CFG not available for this function.
      </div>
    );
  }

  return (
    <div className="w-full h-[400px] border border-slate-700 rounded-md overflow-hidden bg-slate-900 relative">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        fitView
        colorMode="dark"
        minZoom={0.5}
      >
        <Background variant={BackgroundVariant.Dots} gap={20} size={1} color="#334155" />
        <Controls className="bg-slate-800 fill-slate-200 border-slate-700" showInteractive={false} />
      </ReactFlow>
    </div>
  );
};

export default CfgPanel;
