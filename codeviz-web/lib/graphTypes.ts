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
  control_flow?: ControlFlowGraph;
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
  | 'Instantiates'
  | 'Contains';

export interface GraphMeta {
  language: string;
  source_root: string;
  generated_at: string;
  node_count: number;
  edge_count: number;
}

export interface ControlFlowGraph {
  function_id: string;
  blocks: CfgBlock[];
  cfg_edges: CfgEdge[];
}

export interface CfgBlock {
  id: string;
  kind: CfgBlockKind;
  label: string;
  line: number | null;
}

export type CfgBlockKind =
  | 'Entry'
  | 'Exit'
  | 'Block'
  | 'Condition'
  | 'LoopHeader'
  | 'LoopBody'
  | 'SwitchArm'
  | 'TryBlock'
  | 'CatchBlock'
  | 'FinallyBlock'
  | 'AwaitPoint'
  | 'ThrowPoint';

export interface CfgEdge {
  from_id: string;
  to_id: string;
  kind: CfgEdgeKind;
  label: string | null;
}

export type CfgEdgeKind =
  | 'Normal'
  | 'TrueBranch'
  | 'FalseBranch'
  | 'LoopBack'
  | 'ExceptionEdge'
  | 'FinallyEdge'
  | 'AsyncEdge';
