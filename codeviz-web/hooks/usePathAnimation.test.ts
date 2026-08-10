import { renderHook, act } from '@testing-library/react';
import { usePathAnimation } from './usePathAnimation';
import { Node as ReactFlowNode, Edge as ReactFlowEdge } from '@xyflow/react';

const mockNodes: ReactFlowNode[] = [
  { id: 'a', position: { x: 0, y: 0 }, data: { label: 'A' } },
  { id: 'b', position: { x: 0, y: 0 }, data: { label: 'B' } },
  { id: 'c', position: { x: 0, y: 0 }, data: { label: 'C' } },
  { id: 'd', position: { x: 0, y: 0 }, data: { label: 'D' } },
];

const mockEdges: ReactFlowEdge[] = [
  { id: 'e1', source: 'a', target: 'b', data: { kind: 'Calls' } },
  { id: 'e2', source: 'a', target: 'c', data: { kind: 'Calls' } },
  { id: 'e3', source: 'b', target: 'd', data: { kind: 'Calls' } },
  { id: 'e4', source: 'c', target: 'd', data: { kind: 'Calls' } },
  { id: 'e5', source: 'b', target: 'a', data: { kind: 'Imports' } },
];

describe('usePathAnimation', () => {
  beforeEach(() => {
    jest.useFakeTimers();
    let idCounter = 1;
    jest.spyOn(window, 'requestAnimationFrame').mockImplementation((cb: FrameRequestCallback) => {
      const id = idCounter++;
      setTimeout(() => cb(performance.now()), 16);
      return id;
    });
    jest.spyOn(window, 'cancelAnimationFrame').mockImplementation((id: number) => {
      // noop for testing fake timers
    });
  });

  afterEach(() => {
    jest.useRealTimers();
    jest.restoreAllMocks();
  });

  it('initializes correctly', () => {
    const { result } = renderHook(() => usePathAnimation(mockNodes, mockEdges));

    expect(result.current.currentStep).toBe(-1);
    expect(result.current.totalSteps).toBe(0);
    expect(result.current.activeNodes.size).toBe(0);
    expect(result.current.activeEdges.size).toBe(0);
  });

  it('calculates path and starts tracing', () => {
    const { result } = renderHook(() => usePathAnimation(mockNodes, mockEdges));

    act(() => {
      result.current.start('a');
    });

    expect(result.current.totalSteps).toBe(3);
    expect(result.current.currentStep).toBe(0);
    expect(result.current.isPlaying).toBe(true);

    expect(result.current.activeNodes.has('a')).toBe(true);
    expect(result.current.activeNodes.size).toBe(1);
    expect(result.current.activeEdges.size).toBe(0);
  });

  it('steps forward manually correctly', () => {
    const { result } = renderHook(() => usePathAnimation(mockNodes, mockEdges));

    act(() => {
      result.current.start('a');
    });

    act(() => {
      result.current.pause();
    });

    expect(result.current.currentStep).toBe(0);

    act(() => {
      result.current.stepForward();
    });

    expect(result.current.currentStep).toBe(1);
    expect(result.current.activeNodes.has('a')).toBe(true);
    expect(result.current.activeNodes.has('b')).toBe(true);
    expect(result.current.activeNodes.has('c')).toBe(true);
    expect(result.current.activeNodes.size).toBe(3);

    expect(result.current.activeEdges.has('e1')).toBe(true);
    expect(result.current.activeEdges.has('e2')).toBe(true);
    expect(result.current.activeEdges.size).toBe(2);

    act(() => {
      result.current.stepForward();
    });

    expect(result.current.currentStep).toBe(2);
    expect(result.current.activeNodes.has('d')).toBe(true);
    expect(result.current.activeNodes.size).toBe(4);
    expect(result.current.activeEdges.has('e3')).toBe(true);
    expect(result.current.activeEdges.has('e4')).toBe(true);
    expect(result.current.activeEdges.size).toBe(4);

    act(() => {
      result.current.stepForward();
    });

    expect(result.current.currentStep).toBe(2);
  });

  it('steps backward manually correctly', () => {
    const { result } = renderHook(() => usePathAnimation(mockNodes, mockEdges));

    act(() => {
      result.current.start('a');
      result.current.pause();
    });

    act(() => {
      result.current.stepForward();
      result.current.stepForward();
    });

    expect(result.current.currentStep).toBe(2);

    act(() => {
      result.current.stepBackward();
    });

    expect(result.current.currentStep).toBe(1);
    expect(result.current.activeNodes.has('d')).toBe(false);

    act(() => {
      result.current.stepBackward();
    });

    expect(result.current.currentStep).toBe(0);

    act(() => {
      result.current.stepBackward();
    });

    expect(result.current.currentStep).toBe(0);
  });

  it('resets correctly', () => {
    const { result } = renderHook(() => usePathAnimation(mockNodes, mockEdges));

    act(() => {
      result.current.start('a');
      result.current.pause();
    });

    act(() => {
      result.current.stepForward();
    });

    expect(result.current.currentStep).toBe(1);

    act(() => {
      result.current.reset();
    });

    expect(result.current.currentStep).toBe(0);
    expect(result.current.isPlaying).toBe(false);
  });

  it('closes correctly', () => {
    const { result } = renderHook(() => usePathAnimation(mockNodes, mockEdges));

    act(() => {
      result.current.start('a');
    });

    act(() => {
      result.current.close();
    });

    expect(result.current.currentStep).toBe(-1);
    expect(result.current.totalSteps).toBe(0);
    expect(result.current.isPlaying).toBe(false);
  });
});
