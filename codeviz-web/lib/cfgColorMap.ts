import { CfgBlockKind, CfgEdgeKind } from './graphTypes';

export const CFG_NODE_STYLES: Record<string, { bg: string; border: string; shape: string }> = {
  Entry: { bg: '#22C55E', border: '#166534', shape: 'Oval' }, // Green circle -> Oval shape
  Exit: { bg: '#EF4444', border: '#991B1B', shape: 'Oval' }, // Red circle -> Oval shape
  Block: { bg: '#FFFFFF', border: '#D1D5DB', shape: 'Rectangle' }, // White rect -> Rectangle
  Condition: { bg: '#3B82F6', border: '#1E3A8A', shape: 'Diamond' }, // Blue diamond -> Diamond
  LoopHeader: { bg: '#EAB308', border: '#854D0E', shape: 'Diamond' }, // Yellow diamond -> Diamond
  LoopBody: { bg: '#FFFFFF', border: '#D1D5DB', shape: 'Rectangle' }, // Default for standard block body
  SwitchArm: { bg: '#FFFFFF', border: '#D1D5DB', shape: 'Rectangle' }, // Default for switch arm body
  TryBlock: { bg: '#F97316', border: '#9A3412', shape: 'Rectangle' }, // Orange rect -> Rectangle
  CatchBlock: { bg: '#EF4444', border: '#991B1B', shape: 'Rectangle' }, // Red rect -> Rectangle
  FinallyBlock: { bg: '#A855F7', border: '#581C87', shape: 'Rectangle' }, // Purple rect -> Rectangle
  AwaitPoint: { bg: '#A855F7', border: '#581C87', shape: 'Pill' }, // Purple pill -> Pill shape
  ThrowPoint: { bg: '#EF4444', border: '#991B1B', shape: 'Octagon' }, // Red octagon -> Octagon
};

export const CFG_EDGE_STYLES: Record<string, { color: string; style: string }> = {
  Normal: { color: '#000000', style: 'Solid arrow' }, // Black
  TrueBranch: { color: '#22C55E', style: 'Solid' }, // Green
  FalseBranch: { color: '#EF4444', style: 'Solid' }, // Red
  LoopBack: { color: '#EAB308', style: 'Curved back arrow' }, // Yellow
  ExceptionEdge: { color: '#EF4444', style: 'Dashed arrow' }, // Red
  FinallyEdge: { color: '#A855F7', style: 'Dotted arrow' }, // Purple
  AsyncEdge: { color: '#A855F7', style: 'Wavy arrow' }, // Purple
};

export const getCfgNodeStyle = (kind: CfgBlockKind) => {
  return CFG_NODE_STYLES[kind] || { bg: '#FFFFFF', border: '#D1D5DB', shape: 'Rectangle' };
};

export const getCfgEdgeStyle = (kind: CfgEdgeKind) => {
  return CFG_EDGE_STYLES[kind] || { color: '#000000', style: 'Solid arrow' };
};
