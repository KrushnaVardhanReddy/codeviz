import { CodeGraph, Node, Edge, NodeKind, EdgeKind, GraphMeta } from './graphTypes';

describe('graphTypes', () => {
  it('should allow valid CodeGraph JSON structures', () => {
    const meta: GraphMeta = {
      language: 'typescript',
      source_root: '/src',
      generated_at: '2023-10-27T10:00:00Z',
      node_count: 2,
      edge_count: 1,
    };

    const node1: Node = {
      id: 'src/index.ts',
      label: 'index.ts',
      kind: 'File',
      file_path: 'src/index.ts',
      line: 1,
      is_public: true,
    };

    const node2: Node = {
      id: 'src/index.ts::main',
      label: 'main',
      kind: { Function: { is_async: true } },
      file_path: 'src/index.ts',
      line: 5,
      is_public: false,
    };

    const edge: Edge = {
      from_id: 'src/index.ts',
      to_id: 'src/index.ts::main',
      kind: 'Calls',
    };

    const graph: CodeGraph = {
      nodes: [node1, node2],
      edges: [edge],
      meta,
    };

    expect(graph.nodes.length).toBe(2);
    expect(graph.edges.length).toBe(1);
    expect(graph.meta.language).toBe('typescript');
  });
});
