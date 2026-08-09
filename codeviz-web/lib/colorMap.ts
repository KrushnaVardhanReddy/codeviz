export const NODE_COLORS: Record<string, { bg: string; border: string }> = {
  File: { bg: '#1E3A5F', border: '#3B82F6' },
  Module: { bg: '#2D1B69', border: '#8B5CF6' },
  Class: { bg: '#7C2D12', border: '#F97316' },
  Interface: { bg: '#713F12', border: '#EAB308' },
  Function: { bg: '#14532D', border: '#22C55E' },
  'Async Fn': { bg: '#14532D', border: '#22C55E' },
  Constant: { bg: '#1F2937', border: '#6B7280' },
};

export const EDGE_COLORS: Record<string, { color: string; style: string; arrow: string }> = {
  Imports: { color: '#3B82F6', style: 'Solid', arrow: 'Open arrow' },
  Calls: { color: '#22C55E', style: 'Solid', arrow: 'Filled arrow' },
  Inherits: { color: '#F97316', style: 'Solid, 2px thick', arrow: 'Hollow triangle' },
  Implements: { color: '#EAB308', style: 'Dashed', arrow: 'Open arrow' },
  Returns: { color: '#6B7280', style: 'Dotted, thin', arrow: 'Dotted arrow' },
  Instantiates: { color: '#8B5CF6', style: 'Solid', arrow: 'Diamond head' },
};
