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
import * as dagre from 'dagre';
import { Legend } from './Legend';

interface GraphCanvasProps {
  graph: CodeGraph;
}

// Normalize Rust-serialized NodeKind enum to a plain string
const kindToString = (kind: NodeKind): string => {
  if (typeof kind === 'string') return kind;
  if (typeof kind === 'object' && kind !== null) return Object.keys(kind)[0];
  return String(kind);
};

const getNodeStyle = (kind: NodeKind) => {
  const k = kindToString(kind);
  if (k === 'File') return { backgroundColor: '#1E3A5F', borderColor: '#3B82F6', color: 'white' };
  if (k === 'Module') return { backgroundColor: '#2D1B69', borderColor: '#8B5CF6', color: 'white' };
  if (k === 'Class') return { backgroundColor: '#7C2D12', borderColor: '#F97316', color: 'white' };
  if (k === 'Interface') return { backgroundColor: '#713F12', borderColor: '#EAB308', color: 'white', borderStyle: 'dashed' };
  if (k === 'Function') return { backgroundColor: '#14532D', borderColor: '#22C55E', color: 'white', borderRadius: '9999px' };
  if (k === 'Constant') return { backgroundColor: '#1F2937', borderColor: '#6B7280', color: 'white' };
  return { backgroundColor: '#333', borderColor: '#666', color: 'white' };
};

const getEdgeStyle = (kind: EdgeKind) => {
  if (kind === 'Imports') return { stroke: '#3B82F6' };
  if (kind === 'Calls') return { stroke: '#22C55E' };
  if (kind === 'Inherits') return { stroke: '#F97316', strokeWidth: 2 };
  if (kind === 'Implements') return { stroke: '#EAB308', strokeDasharray: '5,5' };
  if (kind === 'Returns') return { stroke: '#6B7280', strokeDasharray: '2,2' };
  if (kind === 'Instantiates') return { stroke: '#8B5CF6' };
  if (kind === 'Contains') return { stroke: '#6B7280', strokeDasharray: '3,3', strokeWidth: 1 };
  return { stroke: '#999' };
};

export const GraphCanvas: React.FC<GraphCanvasProps> = ({ graph }) => {
  const [selectedNode, setSelectedNode] = useState<any>(null);

  const [hiddenNodeKinds, setHiddenNodeKinds] = useState<Set<string>>(new Set());
  const [hiddenEdgeKinds, setHiddenEdgeKinds] = useState<Set<string>>(new Set());

  const onToggleNodeKind = useCallback((kind: string) => {
    setHiddenNodeKinds(prev => {
      const next = new Set(prev);
      if (next.has(kind)) next.delete(kind);
      else next.add(kind);
      return next;
    });
  }, []);

  const onToggleEdgeKind = useCallback((kind: string) => {
    setHiddenEdgeKinds(prev => {
      const next = new Set(prev);
      if (next.has(kind)) next.delete(kind);
      else next.add(kind);
      return next;
    });
  }, []);

  const onNodeClick = useCallback((event: React.MouseEvent, node: any) => {
    setSelectedNode(node);
  }, []);

  const closePanel = () => {
    setSelectedNode(null);
  };

  const [nodes, setNodes, onNodesChange] = useNodesState<ReactFlowNode>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<ReactFlowEdge>([]);

  useEffect(() => {
    const dagreGraph = new dagre.graphlib.Graph({ compound: true });
    dagreGraph.setDefaultEdgeLabel(() => ({}));
    dagreGraph.setGraph({ rankdir: 'TB', nodesep: 50, ranksep: 100 });

    // Workaround for Dagre compound node crashes:
    // We only pass top-level nodes to Dagre. Children are manually positioned inside parents.
    
    // Group children by parent
    const childrenByParent = new Map<string, string[]>();
    graph.nodes.forEach((n) => {
        let parentId = (n as any).parent_id;
        const pNode = graph.nodes.find(pn => pn.id === parentId);
        if (pNode && kindToString(pNode.kind) === 'File') parentId = undefined; // Ignore File nesting
        
        if (parentId) {
            if (!childrenByParent.has(parentId)) childrenByParent.set(parentId, []);
            childrenByParent.get(parentId)!.push(n.id);
        }
    });

    // Add top-level nodes to dagre
    graph.nodes.forEach((n) => {
        let parentId = (n as any).parent_id;
        const pNode = graph.nodes.find(pn => pn.id === parentId);
        if (pNode && kindToString(pNode.kind) === 'File') parentId = undefined;
        
        if (!parentId) {
            const numChildren = childrenByParent.get(n.id)?.length || 0;
            const width = Math.max(170, 200);
            const height = numChildren > 0 ? 80 + numChildren * 90 : 70;
            dagreGraph.setNode(n.id, { width, height });
        }
    });

    // Add edges between top-level nodes
    graph.edges.forEach((e) => {
        let fromParent = (graph.nodes.find(n => n.id === e.from_id) as any)?.parent_id;
        let toParent = (graph.nodes.find(n => n.id === e.to_id) as any)?.parent_id;
        const pNodeFrom = graph.nodes.find(pn => pn.id === fromParent);
        if (pNodeFrom && pNodeFrom.kind === 'File') fromParent = undefined;
        const pNodeTo = graph.nodes.find(pn => pn.id === toParent);
        if (pNodeTo && pNodeTo.kind === 'File') toParent = undefined;
        
        const from = fromParent || e.from_id;
        const to = toParent || e.to_id;
        // Avoid self edges or duplicate edges in dagre
        if (from !== to && dagreGraph.hasNode(from) && dagreGraph.hasNode(to)) {
            dagreGraph.setEdge(from, to);
        }
    });

    dagre.layout(dagreGraph);

    setNodes((currentNodes) => {
      const nodePositionMap = new Map(currentNodes.map(n => [n.id, n.position]));

      return graph.nodes.map((node) => {
        let parentIdToUse = (node as any).parent_id;
        const pNodeInfo = graph.nodes.find(pn => pn.id === parentIdToUse);
        if (pNodeInfo && kindToString(pNodeInfo.kind) === 'File') {
            parentIdToUse = undefined; // Do not nest inside File nodes visually
        }

        const numChildren = childrenByParent.get(node.id)?.length || 0;
        const isChild = !!parentIdToUse;
        const siblings = isChild ? (childrenByParent.get(parentIdToUse) || []) : [];
        const childIndex = siblings.indexOf(node.id);

        const CHILD_NODE_HEIGHT = 30;
        const CHILD_NODE_WIDTH = 150;
        const PARENT_V_PADDING = 46;
        const PARENT_H_PADDING = 20;
        const nodeWidth = numChildren > 0 ? CHILD_NODE_WIDTH + PARENT_H_PADDING * 2 : 150;
        const nodeHeight = numChildren > 0 ? PARENT_V_PADDING + numChildren * (CHILD_NODE_HEIGHT + 10) : undefined;
        const zIndex = numChildren > 0 ? -1 : 0;

        let position: { x: number; y: number };
        if (isChild) {
            // Position is relative to parent top-left corner
            position = {
                x: PARENT_H_PADDING,
                y: PARENT_V_PADDING + childIndex * (CHILD_NODE_HEIGHT + 10),
            };
        } else {
            const dNode = dagreGraph.node(node.id);
            const dagrePos = dNode ? { x: dNode.x - dNode.width / 2, y: dNode.y - dNode.height / 2 } : { x: 0, y: 0 };
            position = nodePositionMap.get(node.id) || dagrePos;
        }

        return {
          id: node.id,
          position,
          parentId: parentIdToUse || undefined,
          extent: isChild ? ('parent' as const) : undefined,
          width: nodeWidth,
          height: nodeHeight,
          zIndex,
          data: { label: node.label, kind: node.kind, testId: `node-${node.id}`, control_flow: node.control_flow },
          style: {
            ...getNodeStyle(node.kind),
            padding: isChild ? '4px 8px' : '10px',
            width: nodeWidth,
            height: nodeHeight,
            textAlign: 'center' as const,
            boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
            fontSize: isChild ? '11px' : '13px',
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
    return nodes.filter(n => {
      const kindStr = kindToString(n.data?.kind as NodeKind);
      return !hiddenNodeKinds.has(kindStr as string);
    }).map((n) => {
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
  }, [nodes, pathAnimation.currentStep, pathAnimation.activeNodes, hiddenNodeKinds]);

  const edgesToRender = useMemo(() => {
    const visibleNodeIds = new Set(nodesToRender.map(n => n.id));
    return edges.filter(e => {
      if (hiddenEdgeKinds.has(e.data?.kind as string)) return false;
      if (!visibleNodeIds.has(e.source) || !visibleNodeIds.has(e.target)) return false;
      return true;
    }).map((e) => {
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
  }, [edges, pathAnimation.currentStep, pathAnimation.activeEdges, hiddenEdgeKinds, nodesToRender]);

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
        <Controls position="top-right" className="bg-slate-800 fill-slate-200 border-slate-700" />
        <MiniMap
          nodeColor={(n) => n.style?.backgroundColor as string || '#eee'}
          maskColor="rgba(15, 23, 42, 0.7)"
          className="bg-slate-800 border-slate-700"
        />
      </ReactFlow>

      <Legend 
        hiddenNodeKinds={hiddenNodeKinds} 
        hiddenEdgeKinds={hiddenEdgeKinds} 
        onToggleNodeKind={onToggleNodeKind} 
        onToggleEdgeKind={onToggleEdgeKind} 
      />

      <DetailPanel
        node={selectedNode}
        onClose={closePanel}
        edges={rawEdges}
        onTraceStart={pathAnimation.start}
      />
    </div>
  );
};
