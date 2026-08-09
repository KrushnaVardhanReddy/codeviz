'use client';

import React, { useState, useCallback } from 'react';
import { ReactFlow, Controls, Background, useNodesState, useEdgesState, BackgroundVariant } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import FileNode from './nodes/FileNode';
import ModuleNode from './nodes/ModuleNode';
import ClassNode from './nodes/ClassNode';
import InterfaceNode from './nodes/InterfaceNode';
import FunctionNode from './nodes/FunctionNode';
import ConstantNode from './nodes/ConstantNode';
import CustomEdge from './edges/CustomEdge';
import DetailPanel from './DetailPanel';

const nodeTypes = {
  File: FileNode,
  Module: ModuleNode,
  Class: ClassNode,
  Interface: InterfaceNode,
  Function: FunctionNode,
  Constant: ConstantNode,
};

const edgeTypes = {
  custom: CustomEdge,
};

// Dummy data for initial layout
const initialNodes = [
  { id: '1', type: 'File', position: { x: 100, y: 100 }, data: { label: 'main.py', language: 'Python' } },
  { id: '2', type: 'Module', position: { x: 300, y: 100 }, data: { label: 'AuthModule' } },
  { id: '3', type: 'Class', position: { x: 500, y: 100 }, data: { label: 'User' } },
  { id: '4', type: 'Interface', position: { x: 700, y: 100 }, data: { label: 'IUser' } },
  { id: '5', type: 'Function', position: { x: 100, y: 300 }, data: { label: 'login()' } },
  { id: '6', type: 'Function', position: { x: 300, y: 300 }, data: { label: 'fetchData()', isAsync: true } },
  { id: '7', type: 'Constant', position: { x: 500, y: 300 }, data: { label: 'MAX_RETRIES' } },
];

const initialEdges = [
  { id: 'e1-5', source: '1', target: '5', type: 'custom', data: { kind: 'Calls' } },
  { id: 'e2-3', source: '2', target: '3', type: 'custom', data: { kind: 'Instantiates' } },
  { id: 'e3-4', source: '3', target: '4', type: 'custom', data: { kind: 'Implements' } },
  { id: 'e5-6', source: '5', target: '6', type: 'custom', data: { kind: 'Calls' } },
];

const DependencyGraph: React.FC = () => {
  const [nodes, , onNodesChange] = useNodesState(initialNodes);
  const [edges, , onEdgesChange] = useEdgesState(initialEdges);
  const [selectedNode, setSelectedNode] = useState<any>(null);

  const onNodeClick = useCallback((event: React.MouseEvent, node: any) => {
    setSelectedNode(node);
  }, []);

  const closePanel = () => {
    setSelectedNode(null);
  };

  return (
    <div className="w-full h-screen bg-[#0F172A] relative overflow-hidden">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={onNodeClick}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        fitView
      >
        <Background
          variant={BackgroundVariant.Dots}
          gap={16}
          size={1}
          color="#1E293B"
        />
        <Controls className="bg-gray-800 border-gray-700 fill-white" />
      </ReactFlow>

      {/* Legend */}
      <div className="absolute bottom-6 right-6 bg-[#21262D] border border-[#30363D] rounded-lg shadow-lg p-3 backdrop-blur-md bg-opacity-80 z-30 font-mono text-xs">
        <div className="text-gray-400 font-bold mb-2 uppercase tracking-widest text-[9px]">Legend</div>
        <div className="flex flex-col gap-2 text-white">
          <div className="flex items-center gap-2"><span className="text-xl">📄</span> File</div>
          <div className="flex items-center gap-2"><span className="text-xl">📦</span> Module</div>
          <div className="flex items-center gap-2"><span className="text-xl">🔷</span> Class</div>
          <div className="flex items-center gap-2"><span className="text-xl">◇</span> Interface</div>
          <div className="flex items-center gap-2"><span className="text-xl">ƒ</span> Function</div>
          <div className="flex items-center gap-2"><span className="text-xl">⚡</span> Async Fn</div>
          <div className="flex items-center gap-2"><span className="text-xl">π</span> Constant</div>
        </div>
      </div>

      <DetailPanel node={selectedNode} onClose={closePanel} edges={edges} />
    </div>
  );
};

export default DependencyGraph;
