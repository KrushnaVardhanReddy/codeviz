import React, { useMemo, useState, useCallback, useEffect } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  Node as ReactFlowNode,
  Edge as ReactFlowEdge,
  BackgroundVariant,
  useNodesState,
  useEdgesState,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { CodeGraph, NodeKind, EdgeKind } from '../lib/graphTypes';
import DetailPanel from './DetailPanel';
import { usePathAnimation } from '../hooks/usePathAnimation';
import { CallPathExplorer } from './CallPathExplorer';

interface GraphCanvasProps {
  graph: CodeGraph;
}

const getNodeStyle = (kind: NodeKind) => {
  if (kind === 'File') return { backgroundColor: '#1E3A5F', borderColor: '#3B82F6', color: 'white' };
  if (kind === 'Module') return { backgroundColor: '#2D1B69', borderColor: '#8B5CF6', color: 'white' };
  if (kind === 'Class') return { backgroundColor: '#7C2D12', borderColor: '#F97316', color: 'white' };
  if (kind === 'Interface') return { backgroundColor: '#713F12', borderColor: '#EAB308', color: 'white', borderStyle: 'dashed' };
  if (typeof kind === 'object' && 'Function' in kind) return { backgroundColor: '#14532D', borderColor: '#22C55E', color: 'white', borderRadius: '9999px' };
  if (kind === 'Constant') return { backgroundColor: '#1F2937', borderColor: '#6B7280', color: 'white' };
  return { backgroundColor: '#333', borderColor: '#666', color: 'white' };
};

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
  const [selectedNode, setSelectedNode] = useState<any>(null);

  const onNodeClick = useCallback((event: React.MouseEvent, node: any) => {
    setSelectedNode(node);
  }, []);

  const closePanel = () => {
    setSelectedNode(null);
  };

  const [nodes, setNodes, onNodesChange] = useNodesState<ReactFlowNode>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<ReactFlowEdge>([]);

  useEffect(() => {
    setNodes((currentNodes) => {
      const nodePositionMap = new Map(currentNodes.map(n => [n.id, n.position]));

      return graph.nodes.map((node, index) => {
        const col = index % 3;
        const row = Math.floor(index / 3);
        const position = nodePositionMap.get(node.id) || { x: 100 + col * 250, y: 100 + row * 150 };

        return {
          id: node.id,
          position,
          data: { label: node.label, kind: node.kind, testId: `node-${node.id}`, control_flow: node.control_flow },
          style: {
            ...getNodeStyle(node.kind),
            padding: '10px',
            width: 150,
            textAlign: 'center' as const,
            boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'
          }
        };
      });
    });

    setEdges(
      graph.edges.map((edge, index) => ({
        id: `e${index}-${edge.from_id}-${edge.to_id}`,
        source: edge.from_id,
        target: edge.to_id,
        style: getEdgeStyle(edge.kind),
        animated: edge.kind === 'Calls',
        data: { kind: edge.kind }
      }))
    );
  }, [graph, setNodes, setEdges]);

  // For usePathAnimation, we need stable references to current raw graph structures.
  const rawEdges = useMemo(() => graph.edges.map((edge, index) => ({
      id: `e${index}-${edge.from_id}-${edge.to_id}`,
      source: edge.from_id,
      target: edge.to_id,
      style: getEdgeStyle(edge.kind),
      animated: edge.kind === 'Calls',
      data: { kind: edge.kind }
  })), [graph.edges]);

  const rawNodes = useMemo(() => nodes, [nodes]); // Use internal state nodes

  const pathAnimation = usePathAnimation(rawNodes, rawEdges);

  const nodesToRender = useMemo(() => {
    return nodes.map((n) => {
      let finalStyle: React.CSSProperties = { ...n.style };

      if (pathAnimation.currentStep >= 0) {
        if (pathAnimation.activeNodes.has(n.id)) {
          // Highlight active node
          finalStyle.boxShadow = '0 0 15px 5px rgba(34, 197, 94, 0.6)';
          finalStyle.borderWidth = '2px';
          finalStyle.borderColor = '#4ADE80';
          finalStyle.opacity = 1;
        } else {
          // Dim inactive node
          finalStyle.opacity = 0.2;
        }
      }

      return {
        ...n,
        style: finalStyle,
        data: {
          ...n.data,
          label: <div data-testid={`node-${n.id}`}>{String(n.data.label)}</div>
        }
      };
    });
  }, [nodes, pathAnimation.currentStep, pathAnimation.activeNodes]);

  const edgesToRender = useMemo(() => {
    return edges.map((e) => {
      let finalStyle: React.CSSProperties = { ...e.style };

      if (pathAnimation.currentStep >= 0) {
        if (pathAnimation.activeEdges.has(e.id)) {
          // Highlight active edge
          finalStyle.strokeWidth = 3;
          finalStyle.stroke = '#4ADE80';
          finalStyle.opacity = 1;
        } else {
          // Dim inactive edge
          finalStyle.opacity = 0.2;
        }
      }

      return {
        ...e,
        style: finalStyle,
      };
    });
  }, [edges, pathAnimation.currentStep, pathAnimation.activeEdges]);

  return (
    <div className="w-full h-full relative" data-testid="graph-canvas">
      {pathAnimation.currentStep >= 0 && (
        <CallPathExplorer
          currentStep={pathAnimation.currentStep}
          totalSteps={pathAnimation.totalSteps}
          isPlaying={pathAnimation.isPlaying}
          currentNodeId={pathAnimation.currentNodeId}
          nodes={rawNodes}
          onPlay={pathAnimation.play}
          onPause={pathAnimation.pause}
          onStepForward={pathAnimation.stepForward}
          onStepBackward={pathAnimation.stepBackward}
          onReset={pathAnimation.reset}
          onClose={pathAnimation.close}
        />
      )}

      <ReactFlow
        nodes={nodesToRender}
        edges={edgesToRender}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={onNodeClick}
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

      <DetailPanel
        node={selectedNode}
        onClose={closePanel}
        edges={rawEdges}
        onTraceStart={pathAnimation.start}
      />
    </div>
  );
};
