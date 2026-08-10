import { getCfgNodeStyle, getCfgEdgeStyle } from './cfgColorMap';

describe('cfgColorMap', () => {
  describe('getCfgNodeStyle', () => {
    it('returns correct style for Entry', () => {
      const style = getCfgNodeStyle('Entry');
      expect(style.bg).toBe('#22C55E'); // Green
      expect(style.shape).toBe('Oval');
    });

    it('returns correct style for Condition', () => {
      const style = getCfgNodeStyle('Condition');
      expect(style.bg).toBe('#3B82F6'); // Blue
      expect(style.shape).toBe('Diamond');
    });

    it('returns correct style for LoopHeader', () => {
      const style = getCfgNodeStyle('LoopHeader');
      expect(style.bg).toBe('#EAB308'); // Yellow
      expect(style.shape).toBe('Diamond');
    });

    it('returns default for unknown kind', () => {
      // @ts-ignore
      const style = getCfgNodeStyle('UnknownKind');
      expect(style.bg).toBe('#FFFFFF');
      expect(style.shape).toBe('Rectangle');
    });
  });

  describe('getCfgEdgeStyle', () => {
    it('returns correct style for Normal', () => {
      const style = getCfgEdgeStyle('Normal');
      expect(style.color).toBe('#000000'); // Black
      expect(style.style).toBe('Solid arrow');
    });

    it('returns correct style for TrueBranch', () => {
      const style = getCfgEdgeStyle('TrueBranch');
      expect(style.color).toBe('#22C55E'); // Green
      expect(style.style).toBe('Solid');
    });
  });
});
