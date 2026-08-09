import { NODE_COLORS, EDGE_COLORS } from './colorMap';

describe('colorMap', () => {
  describe('NODE_COLORS', () => {
    it('returns correct colors for File node', () => {
      expect(NODE_COLORS['File']).toEqual({ bg: '#1E3A5F', border: '#3B82F6' });
    });

    it('returns correct colors for Module node', () => {
      expect(NODE_COLORS['Module']).toEqual({ bg: '#2D1B69', border: '#8B5CF6' });
    });

    it('returns correct colors for Class node', () => {
      expect(NODE_COLORS['Class']).toEqual({ bg: '#7C2D12', border: '#F97316' });
    });

    it('returns correct colors for Interface node', () => {
      expect(NODE_COLORS['Interface']).toEqual({ bg: '#713F12', border: '#EAB308' });
    });

    it('returns correct colors for Function node', () => {
      expect(NODE_COLORS['Function']).toEqual({ bg: '#14532D', border: '#22C55E' });
    });

    it('returns correct colors for Async Fn node', () => {
      expect(NODE_COLORS['Async Fn']).toEqual({ bg: '#14532D', border: '#22C55E' });
    });

    it('returns correct colors for Constant node', () => {
      expect(NODE_COLORS['Constant']).toEqual({ bg: '#1F2937', border: '#6B7280' });
    });
  });

  describe('EDGE_COLORS', () => {
    it('returns correct tokens for Imports edge', () => {
      expect(EDGE_COLORS['Imports']).toEqual({ color: '#3B82F6', style: 'Solid', arrow: 'Open arrow' });
    });

    it('returns correct tokens for Calls edge', () => {
      expect(EDGE_COLORS['Calls']).toEqual({ color: '#22C55E', style: 'Solid', arrow: 'Filled arrow' });
    });

    it('returns correct tokens for Inherits edge', () => {
      expect(EDGE_COLORS['Inherits']).toEqual({ color: '#F97316', style: 'Solid, 2px thick', arrow: 'Hollow triangle' });
    });

    it('returns correct tokens for Implements edge', () => {
      expect(EDGE_COLORS['Implements']).toEqual({ color: '#EAB308', style: 'Dashed', arrow: 'Open arrow' });
    });

    it('returns correct tokens for Returns edge', () => {
      expect(EDGE_COLORS['Returns']).toEqual({ color: '#6B7280', style: 'Dotted, thin', arrow: 'Dotted arrow' });
    });

    it('returns correct tokens for Instantiates edge', () => {
      expect(EDGE_COLORS['Instantiates']).toEqual({ color: '#8B5CF6', style: 'Solid', arrow: 'Diamond head' });
    });
  });
});
