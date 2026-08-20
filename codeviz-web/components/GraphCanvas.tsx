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
import { DrillBreadcrumb, BreadcrumbEntry } from './DrillBreadcrumb';
import * as dagre from 'dagre';
import { Legend } from './Legend';

interface GraphCanvasProps {
  graph: CodeGraph;
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

const kindToString = (kind: NodeKind): string => {
  if (typeof kind === 'string') return kind;
  if (typeof kind === 'object' && kind !== null) return Object.keys(kind)[0];
  return String(kind);
};

const getNodeStyle = (kind: NodeKind, isFocused = false): React.CSSProperties => {
  const k = kindToString(kind);
  const glow = isFocused ? '0 0 0 2px #4ADE80, 0 0 20px 6px rgba(74,222,128,0.35)' : undefined;
  if (k === 'Class')     return { backgroundColor: '#7C2D12', borderColor: '#F97316', color: 'white', boxShadow: glow };
  if (k === 'Interface') return { backgroundColor: '#713F12', borderColor: '#EAB308', color: 'white', borderStyle: 'dashed', boxShadow: glow };
  if (k === 'Function')  return { backgroundColor: '#14532D', borderColor: '#22C55E', color: 'white', borderRadius: '9999px', boxShadow: glow };
  if (k === 'Constant')  return { backgroundColor: '#1F2937', borderColor: '#4B5563', color: '#9CA3AF', boxShadow: glow };
  if (k === 'Module')    return { backgroundColor: '#2D1B69', borderColor: '#8B5CF6', color: 'white', boxShadow: glow };
  if (k === 'File')      return { backgroundColor: '#1E3A5F', borderColor: '#3B82F6', color: 'white', boxShadow: glow };
  return { backgroundColor: '#333', borderColor: '#666', color: 'white', boxShadow: glow };
};

const getEdgeStyle = (kind: EdgeKind): React.CSSProperties => {
  if (kind === 'Imports')      return { stroke: '#3B82F6' };
  if (kind === 'Calls')        return { stroke: '#22C55E' };
  if (kind === 'Inherits')     return { stroke: '#F97316', strokeWidth: 2 };
  if (kind === 'Implements')   return { stroke: '#EAB308', strokeDasharray: '5,5' };
  if (kind === 'Returns')      return { stroke: '#6B7280', strokeDasharray: '2,2' };
  if (kind === 'Instantiates') return { stroke: '#8B5CF6' };
  if (kind === 'Contains')     return { stroke: '#4B5563', strokeDasharray: '3,3', strokeWidth: 1 };
  return { stroke: '#999' };
};

function runDagre(
  nodeIds: string[],
  edgePairs: { from: string; to: string }[],
  opts: { rankdir?: string; nodesep?: number; ranksep?: number; nodeW?: number; nodeH?: number } = {}
): Map<string, { x: number; y: number }> {
  const g = new dagre.graphlib.Graph();
  g.setDefaultEdgeLabel(() => ({}));
  g.setGraph({ rankdir: opts.rankdir || 'TB', nodesep: opts.nodesep ?? 60, ranksep: opts.ranksep ?? 100 });
  const W = opts.nodeW ?? 160;
  const H = opts.nodeH ?? 44;
  nodeIds.forEach(id => g.setNode(id, { width: W, height: H }));
  edgePairs.forEach(({ from, to }) => {
    if (from !== to && g.hasNode(from) && g.hasNode(to)) g.setEdge(from, to);
  });
  dagre.layout(g);
  const pos = new Map<string, { x: number; y: number }>();
  nodeIds.forEach(id => {
    const n = g.node(id);
    if (n) pos.set(id, { x: n.x - n.width / 2, y: n.y - n.height / 2 });
  });
  return pos;
}

type PrimaryMode = 'structural' | 'execution';
type ViewMode = 'classes' | 'expanded' | 'focus';

// ─── Main Component ───────────────────────────────────────────────────────────

export const GraphCanvas: React.FC<GraphCanvasProps> = ({ graph: rawGraph }) => {
  // De-duplicate nodes and edges
  const graph = useMemo(() => {
    const nodesMap = new Map();
    rawGraph.nodes.forEach(n => nodesMap.set(n.id, n));
    const edgesMap = new Map();
    rawGraph.edges.forEach(e => edgesMap.set(`${e.from_id}->${e.to_id}-${e.kind}`, e));
    return { ...rawGraph, nodes: Array.from(nodesMap.values()), edges: Array.from(edgesMap.values()) };
  }, [rawGraph]);

  // ─── Pre-computed lookup maps ────────────────────────────────────────────────

  // label → node id (for resolving short-name edge targets)
  const labelToId = useMemo(() => {
    const m = new Map<string, string>();
    graph.nodes.forEach(n => {
      if (!m.has(n.label)) m.set(n.label, n.id); // first wins
    });
    return m;
  }, [graph.nodes]);

  // nodeId → raw node
  const nodeById = useMemo(() => {
    const m = new Map<string, any>();
    graph.nodes.forEach(n => m.set(n.id, n));
    return m;
  }, [graph.nodes]);

  // Resolve an edge target id — try exact match, then label match
  const resolveId = useCallback((id: string): string | null => {
    if (nodeById.has(id)) return id;
    const byLabel = labelToId.get(id);
    if (byLabel) return byLabel;
    // try matching the last segment after ::
    const shortName = id.split('::').pop() || id;
    return labelToId.get(shortName) ?? null;
  }, [nodeById, labelToId]);

  // Map: parentId → Set<childId>
  const childrenMap = useMemo(() => {
    const m = new Map<string, Set<string>>();
    graph.nodes.forEach(n => {
      const pid = (n as any).parent_id;
      if (pid) {
        if (!m.has(pid)) m.set(pid, new Set());
        m.get(pid)!.add(n.id);
      }
    });
    return m;
  }, [graph.nodes]);

  // Function → owning class (parent chain walk until Class/Interface found)
  const fnToClass = useMemo(() => {
    const m = new Map<string, string>();
    graph.nodes.forEach(n => {
      if (kindToString(n.kind) !== 'Function') return;
      let pid = (n as any).parent_id;
      while (pid) {
        const parent = nodeById.get(pid);
        if (!parent) break;
        const pk = kindToString(parent.kind);
        if (pk === 'Class' || pk === 'Interface') { m.set(n.id, pid); break; }
        pid = (parent as any).parent_id;
      }
    });
    return m;
  }, [graph.nodes, nodeById]);

  // Collect all functions that belong to a class (recursively through nested classes)
  const getAllFnsOfClass = useCallback((classId: string): string[] => {
    const result: string[] = [];
    const stack = [classId];
    while (stack.length) {
      const cur = stack.pop()!;
      const children = childrenMap.get(cur) || new Set();
      children.forEach(cid => {
        const child = nodeById.get(cid);
        if (!child) return;
        const k = kindToString(child.kind);
        if (k === 'Function') result.push(cid);
        else stack.push(cid); // recurse into nested classes
      });
    }
    return result;
  }, [childrenMap, nodeById]);

  // Direct callees of a function (resolved IDs)
  const getCallees = useCallback((fnId: string): string[] => {
    const result: string[] = [];
    graph.edges.forEach(e => {
      if (e.from_id !== fnId || e.kind !== 'Calls') return;
      const resolved = resolveId(e.to_id);
      if (resolved && resolved !== fnId) result.push(resolved);
    });
    return [...new Set(result)];
  }, [graph.edges, resolveId]);

  // ─── Entry Points (Execution Mode) ──────────────────────────────────────────
  const entryPoints = useMemo(() => {
    const fns = graph.nodes.filter(n => kindToString(n.kind) === 'Function');
    const inDegreeMap = new Map<string, number>();
    fns.forEach(f => inDegreeMap.set(f.id, 0));

    graph.edges.forEach(e => {
      if (e.kind === 'Calls') {
        const resolvedTo = resolveId(e.to_id);
        if (resolvedTo && inDegreeMap.has(resolvedTo)) {
          inDegreeMap.set(resolvedTo, (inDegreeMap.get(resolvedTo) || 0) + 1);
        }
      }
    });

    const candidates = fns.map(f => {
      const isMain = f.label.toLowerCase().includes('main');
      const inDegree = inDegreeMap.get(f.id) || 0;
      let score = 0;
      if (isMain) score += 10;
      if (inDegree === 0) score += 5;
      return { node: f, inDegree, isMain, score };
    });

    candidates.sort((a, b) => b.score - a.score || a.node.label.localeCompare(b.node.label));
    return candidates;
  }, [graph.nodes, graph.edges, resolveId]);

  // ─── Core Classes (src/flask only, no tests) ─────────────────────────────────
  const coreClasses = useMemo(() => {
    return graph.nodes.filter(n => {
      const k = kindToString(n.kind);
      if (k !== 'Class' && k !== 'Interface') return false;
      const fp = (n as any).file_path || '';
      return fp.includes('src/flask') || fp.includes('src\\flask');
    });
  }, [graph.nodes]);

  // ─── State ────────────────────────────────────────────────────────────────────
  const [primaryMode, setPrimaryMode] = useState<PrimaryMode>('structural');
  const [selectedEntryPoint, setSelectedEntryPoint] = useState<string | null>(null);
  const [expandedTreeNodes, setExpandedTreeNodes] = useState<Set<string>>(new Set());

  // Set default entry point when first opening execution mode
  useEffect(() => {
    if (primaryMode === 'execution' && !selectedEntryPoint && entryPoints.length > 0) {
      setSelectedEntryPoint(entryPoints[0].node.id);
      setExpandedTreeNodes(new Set([entryPoints[0].node.id]));
    }
  }, [primaryMode, selectedEntryPoint, entryPoints]);

  const [viewMode, setViewMode] = useState<ViewMode>('classes');
  const [expandedClassId, setExpandedClassId] = useState<string | null>(null);
  const [focusedFnId, setFocusedFnId] = useState<string | null>(null);
  const [breadcrumb, setBreadcrumb] = useState<BreadcrumbEntry[]>([]);
  const [selectedNode, setSelectedNode] = useState<any>(null);
  const [hiddenNodeKinds, setHiddenNodeKinds] = useState<Set<string>>(new Set());
  const [hiddenEdgeKinds, setHiddenEdgeKinds] = useState<Set<string>>(new Set());

  const onToggleNodeKind = useCallback((kind: string) => {
    setHiddenNodeKinds(prev => { const n = new Set(prev); n.has(kind) ? n.delete(kind) : n.add(kind); return n; });
  }, []);
  const onToggleEdgeKind = useCallback((kind: string) => {
    setHiddenEdgeKinds(prev => { const n = new Set(prev); n.has(kind) ? n.delete(kind) : n.add(kind); return n; });
  }, []);

  const [nodes, setNodes, onNodesChange] = useNodesState<ReactFlowNode>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<ReactFlowEdge>([]);

  // ─── Build visible graph ──────────────────────────────────────────────────────
  useEffect(() => {
    let visibleNodeIds: string[] = [];
    let visibleEdges: { from: string; to: string; kind: EdgeKind }[] = [];
    let focusedId: string | null = null;

    // ── Mode: execution flow (BFS) ──────────────────────────────────────────────
    if (primaryMode === 'execution') {
      if (selectedEntryPoint) {
        focusedId = selectedEntryPoint;

        // BFS traversal
        const queue: string[] = [selectedEntryPoint];
        const visitedNodes = new Set<string>();
        const seenEdges = new Set<string>();

        while (queue.length > 0) {
          const currentId = queue.shift()!;
          if (!visitedNodes.has(currentId)) {
            visitedNodes.add(currentId);

            // Only traverse children if this node has been clicked/expanded
            if (expandedTreeNodes.has(currentId)) {
              const callees = getCallees(currentId);
              callees.forEach(calleeId => {
                if (!visitedNodes.has(calleeId)) {
                  queue.push(calleeId);
                }
                const key = `${currentId}->${calleeId}-Calls`;
                if (!seenEdges.has(key)) {
                  seenEdges.add(key);
                  visibleEdges.push({ from: currentId, to: calleeId, kind: 'Calls' });
                }
              });
            }
          }
        }
        visibleNodeIds = Array.from(visitedNodes);
      }
    }
    // ── Mode: all core classes ──────────────────────────────────────────────────
    else if (viewMode === 'classes') {
      const filtered = coreClasses.filter(n => !hiddenNodeKinds.has(kindToString(n.kind)));
      visibleNodeIds = filtered.map(n => n.id);
      const visibleSet = new Set(visibleNodeIds);
      const seen = new Set<string>();

      graph.edges.forEach(e => {
        if (hiddenEdgeKinds.has(e.kind)) return;
        if (e.kind !== 'Inherits' && e.kind !== 'Instantiates' && e.kind !== 'Calls') return;

        // Resolve both ends
        let fromId = visibleSet.has(e.from_id) ? e.from_id : null;
        let toId   = visibleSet.has(e.to_id)   ? e.to_id   : resolveId(e.to_id);
        if (!toId || !visibleSet.has(toId)) toId = null;

        // For Calls: roll up fn→fn to class→class
        if (e.kind === 'Calls') {
          if (!fromId) fromId = fnToClass.get(e.from_id) ?? null;
          if (fromId && !visibleSet.has(fromId)) fromId = null;
          if (!toId) {
            const resolvedTo = resolveId(e.to_id);
            if (resolvedTo) toId = fnToClass.get(resolvedTo) ?? (visibleSet.has(resolvedTo) ? resolvedTo : null);
          }
        }

        if (fromId && toId && fromId !== toId) {
          const key = `${fromId}->${toId}-${e.kind}`;
          if (!seen.has(key)) { seen.add(key); visibleEdges.push({ from: fromId, to: toId, kind: e.kind }); }
        }
      });
    }

    // ── Mode: expanded class (class + its direct functions) ─────────────────────
    else if (viewMode === 'expanded' && expandedClassId) {
      focusedId = expandedClassId;
      const fnIds = getAllFnsOfClass(expandedClassId);
      const fnSet = new Set(fnIds);
      // Bring in callee nodes external to this class
      const externalNodes = new Set<string>();
      graph.edges.forEach(e => {
        if (e.kind !== 'Calls' || !fnSet.has(e.from_id)) return;
        const resolved = resolveId(e.to_id);
        if (resolved && resolved !== expandedClassId && !fnSet.has(resolved)) {
          externalNodes.add(resolved);
        }
      });
      visibleNodeIds = [expandedClassId, ...fnIds, ...Array.from(externalNodes)];
      const visibleSet = new Set(visibleNodeIds);

      const seen = new Set<string>();
      graph.edges.forEach(e => {
        if (hiddenEdgeKinds.has(e.kind)) return;
        let from = visibleSet.has(e.from_id) ? e.from_id : null;
        let to   = visibleSet.has(e.to_id)   ? e.to_id   : (resolveId(e.to_id) && visibleSet.has(resolveId(e.to_id)!) ? resolveId(e.to_id)! : null);
        if (!to && e.kind === 'Calls') {
          const r = resolveId(e.to_id);
          if (r && visibleSet.has(r)) to = r;
        }
        if (from && to && from !== to) {
          const key = `${from}->${to}-${e.kind}`;
          if (!seen.has(key)) { seen.add(key); visibleEdges.push({ from, to, kind: e.kind }); }
        }
      });
    }

    // ── Mode: focused function + its callees ─────────────────────────────────────
    else if (viewMode === 'focus' && focusedFnId) {
      focusedId = focusedFnId;
      const calleeIds = getCallees(focusedFnId);
      visibleNodeIds = [focusedFnId, ...calleeIds];
      const calleeSet = new Set(calleeIds);
      const seen = new Set<string>();
      graph.edges.forEach(e => {
        if (hiddenEdgeKinds.has(e.kind) || e.kind !== 'Calls') return;
        if (e.from_id !== focusedFnId && !calleeSet.has(e.from_id)) return;
        const to = calleeSet.has(e.to_id) ? e.to_id : (resolveId(e.to_id) && calleeSet.has(resolveId(e.to_id)!) ? resolveId(e.to_id)! : null);
        if (!to) return;
        const from = e.from_id;
        const key = `${from}->${to}`;
        if (!seen.has(key)) { seen.add(key); visibleEdges.push({ from, to, kind: 'Calls' }); }
      });
    }

    // ── Layout ───────────────────────────────────────────────────────────────────
    let layoutOpts: { rankdir: string; nodesep: number; ranksep: number; nodeW?: number; nodeH?: number };

    if (primaryMode === 'execution') {
      layoutOpts = { rankdir: 'TB', nodesep: 80, ranksep: 140, nodeW: 180, nodeH: 50 };
    } else {
      layoutOpts = viewMode === 'focus'
        ? { rankdir: 'TB', nodesep: 80, ranksep: 120 }
        : viewMode === 'expanded'
        ? { rankdir: 'LR', nodesep: 50, ranksep: 100 }
        : { rankdir: 'TB', nodesep: 80, ranksep: 140, nodeW: 180, nodeH: 50 };
    }

    const posMap = runDagre(
      visibleNodeIds,
      visibleEdges.map(e => ({ from: e.from, to: e.to })),
      layoutOpts
    );

    setNodes(
      visibleNodeIds.map(id => {
        const raw = nodeById.get(id);
        const kind = raw?.kind ?? 'Constant';
        const isFocused = id === focusedId;
        const pos = posMap.get(id) || { x: 0, y: 0 };
        const k = kindToString(kind);
        const label = raw?.label ?? id.split('::').pop() ?? id;
        return {
          id,
          position: pos,
          data: { label, kind, testId: `node-${id}`, control_flow: raw?.control_flow },
          style: {
            ...getNodeStyle(kind, isFocused),
            padding: '8px 16px',
            width: k === 'Function' ? 150 : 180,
            textAlign: 'center' as const,
            fontSize: k === 'Function' ? '11px' : '13px',
            fontWeight: isFocused ? 700 : 500,
            transition: 'all 0.2s ease',
          },
        };
      })
    );

    setEdges(
      visibleEdges.map((e, i) => ({
        id: `e${i}-${e.from}-${e.to}`,
        source: e.from,
        target: e.to,
        style: getEdgeStyle(e.kind),
        animated: e.kind === 'Calls',
        markerEnd: { type: 'arrowclosed' as any, color: getEdgeStyle(e.kind).stroke as string },
        data: { kind: e.kind },
      }))
    );
  }, [primaryMode, selectedEntryPoint, expandedTreeNodes, viewMode, expandedClassId, focusedFnId, graph, coreClasses, nodeById, resolveId,
      fnToClass, getAllFnsOfClass, getCallees, hiddenNodeKinds, hiddenEdgeKinds, setNodes, setEdges]);

  // ─── Click handlers ───────────────────────────────────────────────────────────
  const onNodeClick = useCallback((_event: React.MouseEvent, node: any) => {
    setSelectedNode(node);

    if (primaryMode === 'execution') {
      setExpandedTreeNodes(prev => {
        const next = new Set(prev);
        if (next.has(node.id)) {
          next.delete(node.id); // collapse
        } else {
          next.add(node.id); // expand
        }
        return next;
      });
      return;
    }

    const raw = nodeById.get(node.id);
    if (!raw) return;
    const k = kindToString(raw.kind);

    if (k === 'Class' || k === 'Interface') {
      const fns = getAllFnsOfClass(node.id);
      if (fns.length === 0) return; // leaf class, just show detail
      setExpandedClassId(node.id);
      setFocusedFnId(null);
      setViewMode('expanded');
      setBreadcrumb([{ id: node.id, label: raw.label, kind: 'class' }]);
    } else if (k === 'Function') {
      const callees = getCallees(node.id);
      if (callees.length === 0) return; // leaf fn
      setFocusedFnId(node.id);
      setViewMode('focus');
      setBreadcrumb(prev => {
        const classEntry = prev.find(c => c.kind === 'class');
        const fnEntry: BreadcrumbEntry = { id: node.id, label: raw.label, kind: 'function' };
        if (classEntry) {
          const idx = prev.findIndex(c => c.id === classEntry.id);
          return [...prev.slice(0, idx + 1), fnEntry];
        }
        return [fnEntry];
      });
    }
  }, [primaryMode, nodeById, getAllFnsOfClass, getCallees]);

  const onBreadcrumbNavigate = useCallback((index: number) => {
    if (index === -1) {
      setViewMode('classes'); setExpandedClassId(null);
      setFocusedFnId(null); setBreadcrumb([]); setSelectedNode(null);
      return;
    }
    const crumb = breadcrumb[index];
    if (!crumb) return;
    if (crumb.kind === 'class') {
      setExpandedClassId(crumb.id); setFocusedFnId(null);
      setViewMode('expanded'); setBreadcrumb(prev => prev.slice(0, index + 1));
    } else if (crumb.kind === 'function') {
      setFocusedFnId(crumb.id);
      setViewMode('focus'); setBreadcrumb(prev => prev.slice(0, index + 1));
    }
  }, [breadcrumb]);

  // ─── Path animation ───────────────────────────────────────────────────────────
  const rawEdges = useMemo(() => edges, [edges]);
  const rawNodes = useMemo(() => nodes, [nodes]);
  const pathAnimation = usePathAnimation(rawNodes, rawEdges);

  const nodesToRender = useMemo(() => nodes.map(n => {
    let s: React.CSSProperties = { ...n.style };
    if (pathAnimation.currentStep >= 0) {
      s = pathAnimation.activeNodes.has(n.id)
        ? { ...s, boxShadow: '0 0 15px 5px rgba(74,222,128,0.6)', borderColor: '#4ADE80', opacity: 1 }
        : { ...s, opacity: 0.2 };
    }
    return { ...n, style: s };
  }), [nodes, pathAnimation.currentStep, pathAnimation.activeNodes]);

  const edgesToRender = useMemo(() => edges.map(e => {
    let s: React.CSSProperties = { ...e.style };
    if (pathAnimation.currentStep >= 0) {
      s = pathAnimation.activeEdges.has(e.id)
        ? { ...s, strokeWidth: 3, stroke: '#4ADE80', opacity: 1 }
        : { ...s, opacity: 0.2 };
    }
    return { ...e, style: s };
  }), [edges, pathAnimation.currentStep, pathAnimation.activeEdges]);

  const statusLabel = useMemo(() => {
    if (viewMode === 'classes') return `${nodes.length} core classes · click to expand methods`;
    if (viewMode === 'expanded') return `${nodes.length - 1} methods · click one to see its callees`;
    if (viewMode === 'focus')    return `${nodes.length - 1} callees · click any to drill deeper`;
    return '';
  }, [viewMode, nodes.length]);

  return (
    <div className="w-full h-full relative" data-testid="graph-canvas">
      {/* ─── Mode Toggle & Entry Point Selector ─── */}
      <div className="absolute top-3 left-3 z-30 flex flex-col gap-2">
        <div className="flex bg-slate-800 rounded p-1 border border-slate-700 w-fit">
          <button
            className={`px-3 py-1 text-xs font-medium rounded transition-colors ${primaryMode === 'structural' ? 'bg-blue-600 text-white' : 'text-slate-400 hover:text-slate-200'}`}
            onClick={() => setPrimaryMode('structural')}
          >
            Structural
          </button>
          <button
            className={`px-3 py-1 text-xs font-medium rounded transition-colors ${primaryMode === 'execution' ? 'bg-blue-600 text-white' : 'text-slate-400 hover:text-slate-200'}`}
            onClick={() => setPrimaryMode('execution')}
          >
            Execution Flow
          </button>
        </div>

        {primaryMode === 'execution' && (
          <div className="bg-slate-800 border border-slate-700 rounded p-2 flex flex-col gap-1 w-64">
            <label className="text-[10px] uppercase text-slate-400 font-semibold tracking-wider">Entry Point</label>
            <select
              className="bg-slate-900 text-slate-200 border border-slate-700 rounded px-2 py-1 text-xs w-full focus:outline-none focus:border-blue-500"
              value={selectedEntryPoint || ''}
              onChange={(e) => {
                const newId = e.target.value;
                setSelectedEntryPoint(newId);
                setExpandedTreeNodes(new Set([newId]));
              }}
            >
              {entryPoints.map((ep) => (
                <option key={ep.node.id} value={ep.node.id}>
                  {ep.node.label} {ep.isMain ? '(Main)' : ep.inDegree === 0 ? '(Root)' : ''}
                </option>
              ))}
            </select>
          </div>
        )}
      </div>

      {primaryMode === 'structural' && (
        <DrillBreadcrumb crumbs={breadcrumb} onNavigate={onBreadcrumbNavigate} />
      )}

      <div className="absolute top-3 right-16 z-20 bg-slate-900/80 border border-slate-700 rounded-full px-3 py-1 text-xs text-slate-400 backdrop-blur-sm select-none">
        {statusLabel}
      </div>

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
        fitViewOptions={{ padding: 0.12 }}
        className="bg-slate-900"
        colorMode="dark"
      >
        <Background variant={BackgroundVariant.Dots} gap={22} size={1} color="#1E293B" />
        <Controls position="top-right" className="bg-slate-800 fill-slate-200 border-slate-700" />
        <MiniMap
          nodeColor={n => (n.style?.backgroundColor as string) || '#eee'}
          maskColor="rgba(15,23,42,0.75)"
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
        onClose={() => setSelectedNode(null)}
        edges={rawEdges}
        onTraceStart={pathAnimation.start}
      />
    </div>
  );
};
