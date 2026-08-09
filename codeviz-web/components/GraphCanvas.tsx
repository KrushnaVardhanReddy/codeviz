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
import { CodeGraph, NodeKind, EdgeKind } from '../lib/graphTypes';

interface GraphCanvasProps {
  graph: CodeGraph;
}

// Helper to determine node styles based on NodeKind
const getNodeStyle = (kind: NodeKind) => {
  if (kind === 'File') return { backgroundColor: '#1E3A5F', borderColor: '#3B82F6', color: 'white' };
  if (kind === 'Module') return { backgroundColor: '#2D1B69', borderColor: '#8B5CF6', color: 'white' };
  if (kind === 'Class') return { backgroundColor: '#7C2D12', borderColor: '#F97316', color: 'white' };
  if (kind === 'Interface') return { backgroundColor: '#713F12', borderColor: '#EAB308', color: 'white', borderStyle: 'dashed' };
  if (typeof kind === 'object' && 'Function' in kind) return { backgroundColor: '#14532D', borderColor: '#22C55E', color: 'white', borderRadius: '9999px' };
  if (kind === 'Constant') return { backgroundColor: '#1F2937', borderColor: '#6B7280', color: 'white' };
  return { backgroundColor: '#333', borderColor: '#666', color: 'white' };
};

// Helper to determine edge styles based on EdgeKind
const getEdgeStyle = (kind: EdgeKind) => {
  if (kind === 'Imports') return { stroke: '#3B82F6' };
  if (kind === 'Calls') return { stroke: '#22C55E' };
  if (kind === 'Inherits') return { stroke: '#F97316', strokeWidth: 2 };
  if (kind === 'Implements') return { stroke: '#EAB308', strokeDasharray: '5,5' };
  if (kind === 'Returns') return { stroke: '#6B7280', strokeDasharray: '2,2' };
  if (kind === 'Instantiates') return { stroke: '#8B5CF6' };
  return { stroke: '#999' };
};

export const GraphCanvas: React.FC<GraphCanvasProps> = ({ graph }) => {
  const nodes: ReactFlowNode[] = useMemo(() => {
    // Very basic layout algorithm for demonstration
    return graph.nodes.map((node, index) => {
      const col = index % 3;
      const row = Math.floor(index / 3);
      return {
        id: node.id,
        position: { x: 100 + col * 250, y: 100 + row * 150 },
        data: { label: node.label },
        style: {
          ...getNodeStyle(node.kind),
          padding: '10px',
          width: 150,
          textAlign: 'center',
          boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'
        }
      };
    });
  }, [graph.nodes]);

  const edges: ReactFlowEdge[] = useMemo(() => {
    return graph.edges.map((edge, index) => ({
      id: `e${index}-${edge.from_id}-${edge.to_id}`,
      source: edge.from_id,
      target: edge.to_id,
      style: getEdgeStyle(edge.kind),
      animated: edge.kind === 'Calls',
    }));
  }, [graph.edges]);

  return (
    <div className="w-full h-full relative">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        fitView
        className="bg-slate-900"
        colorMode="dark"
      >
        <Background variant={BackgroundVariant.Dots} gap={20} size={1} color="#1E293B" />
        <Controls className="bg-slate-800 fill-slate-200 border-slate-700" />
        <MiniMap
          nodeColor={(n) => n.style?.backgroundColor as string || '#eee'}
          maskColor="rgba(15, 23, 42, 0.7)"
          className="bg-slate-800 border-slate-700"
        />
      </ReactFlow>
    </div>
  );
};
