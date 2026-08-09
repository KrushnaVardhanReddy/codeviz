export interface CodeGraph {
  nodes: Node[];
  edges: Edge[];
  meta: GraphMeta;
}

export interface Node {
  id: string;
  label: string;
  kind: NodeKind;
  file_path: string;
  line: number | null;
  is_public: boolean;
}

export type NodeKind =
  | 'File'
  | 'Module'
  | { Function: { is_async: boolean } }
  | 'Class'
  | 'Interface'
  | 'Constant';

export interface Edge {
  from_id: string;
  to_id: string;
  kind: EdgeKind;
}

export type EdgeKind =
  | 'Imports'
  | 'Calls'
  | 'Inherits'
  | 'Implements'
  | 'Returns'
  | 'Instantiates';

export interface GraphMeta {
  language: string;
  source_root: string;
  generated_at: string;
  node_count: number;
  edge_count: number;
}
