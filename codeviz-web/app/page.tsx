'use client';

import React from 'react';
import { TopNav } from '../components/TopNav';
import { Sidebar } from '../components/Sidebar';
import { GraphCanvas } from '../components/GraphCanvas';
import { CodeGraph } from '../lib/graphTypes';
import { Network } from 'lucide-react';

const sampleGraph: CodeGraph = {
  meta: {
    language: 'typescript',
    source_root: '/src',
    generated_at: new Date().toISOString(),
    node_count: 3,
    edge_count: 2,
  },
  nodes: [
    {
      id: 'App.tsx',
      label: 'App.tsx',
      kind: 'File',
      file_path: 'App.tsx',
      line: 1,
      is_public: true,
    },
    {
      id: 'Layout.tsx',
      label: 'Layout.tsx',
      kind: 'Module',
      file_path: 'Layout.tsx',
      line: 1,
      is_public: true,
    },
    {
      id: 'Dashboard.tsx',
      label: 'Dashboard.tsx',
      kind: { Function: { is_async: false } },
      file_path: 'Dashboard.tsx',
      line: 10,
      is_public: true,
      control_flow: {
        function_id: "example.ts::helloWorld",
        blocks: [{ id: "start", kind: "Entry", label: "Start", line: null }],
        cfg_edges: []
      }
    }
  ],
  edges: [
    {
      from_id: 'App.tsx',
      to_id: 'Layout.tsx',
      kind: 'Imports'
    },
    {
      from_id: 'App.tsx',
      to_id: 'Dashboard.tsx',
      kind: 'Calls'
    }
  ]
};

export default function Home() {
  const [graph, setGraph] = React.useState<CodeGraph | null>(null);
  const [repo, setRepo] = React.useState<'httpie' | 'flask'>('httpie');

  React.useEffect(() => {
    setGraph(null);
    fetch(`/${repo}.json`)
      .then(res => res.json())
      .then(data => setGraph(data))
      .catch(err => console.error(`Failed to load ${repo}.json:`, err));
  }, [repo]);

  const repoLabel = repo === 'httpie' ? 'HTTPie CLI' : 'Flask';

  return (
    <div className="flex flex-col h-screen overflow-hidden bg-slate-900 text-slate-100">
      <TopNav />
      <div className="flex flex-1 pt-16 h-[calc(100vh-64px)] w-full">
        <Sidebar />
        <main className="flex-1 ml-0 md:ml-64 p-4 md:p-6 h-full relative">
          <div className="w-full h-full bg-slate-800/50 rounded-xl border border-slate-700/50 shadow-2xl relative overflow-hidden flex flex-col">
            <div className="absolute top-4 left-4 z-10 flex items-center gap-3 pointer-events-none">
              <Network className="text-blue-500 w-5 h-5" />
              <h2 className="font-semibold text-lg text-slate-200">CodeViz Architecture Graph ({repoLabel})</h2>
            </div>

            {/* Repo switcher */}
            <div className="absolute top-3 right-4 z-20 pointer-events-auto">
              <select
                className="bg-slate-800 border border-slate-600 text-slate-200 text-xs rounded px-2 py-1.5 focus:outline-none focus:border-blue-500 cursor-pointer"
                value={repo}
                onChange={e => setRepo(e.target.value as 'httpie' | 'flask')}
              >
                <option value="httpie">📦 HTTPie CLI</option>
                <option value="flask">🌶️ Flask</option>
              </select>
            </div>

            <div className="flex-1 w-full relative">
              {graph
                ? <GraphCanvas graph={graph} />
                : <div className="flex items-center justify-center h-full text-slate-400">Loading {repoLabel} graph…</div>
              }
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}
