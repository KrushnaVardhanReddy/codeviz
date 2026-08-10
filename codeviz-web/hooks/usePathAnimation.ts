import { useState, useCallback, useRef, useEffect } from 'react';
import { Node as ReactFlowNode, Edge as ReactFlowEdge } from '@xyflow/react';

export interface PathStep {
  activeNodes: Set<string>;
  activeEdges: Set<string>;
  currentNodeId: string | null;
}

export const usePathAnimation = (nodes: ReactFlowNode[], edges: ReactFlowEdge[]) => {
  const [history, setHistory] = useState<PathStep[]>([]);
  const [currentStep, setCurrentStep] = useState<number>(-1);
  const [isPlaying, setIsPlaying] = useState<boolean>(false);
  const animationRef = useRef<number | null>(null);
  const lastFrameTimeRef = useRef<number>(0);

  // 400ms per step as per requirements
  const STEP_DURATION = 400;

  // Cleanup requestAnimationFrame on unmount
  useEffect(() => {
    return () => {
      if (animationRef.current !== null) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, []);

  const calculatePaths = useCallback((startNodeId: string) => {
    const steps: PathStep[] = [];
    const visitedNodes = new Set<string>();
    const visitedEdges = new Set<string>();

    const queue: { nodeId: string; edgeId: string | null }[] = [];

    queue.push({ nodeId: startNodeId, edgeId: null });
    visitedNodes.add(startNodeId);

    // Initial step: just the start node
    steps.push({
      activeNodes: new Set([startNodeId]),
      activeEdges: new Set(),
      currentNodeId: startNodeId,
    });

    // BFS
    let currentLevel = [...queue];

    while (currentLevel.length > 0) {
      const nextLevel: typeof queue = [];
      const currentLevelNodes = new Set<string>();
      const currentLevelEdges = new Set<string>();

      for (const { nodeId } of currentLevel) {
        // Find outgoing "Calls" edges from this node
        const outgoingEdges = edges.filter(
          (e) => e.source === nodeId && e.data?.kind === 'Calls'
        );

        for (const edge of outgoingEdges) {
          if (!visitedEdges.has(edge.id)) {
            visitedEdges.add(edge.id);
            currentLevelEdges.add(edge.id);

            if (!visitedNodes.has(edge.target)) {
              visitedNodes.add(edge.target);
              currentLevelNodes.add(edge.target);
              nextLevel.push({ nodeId: edge.target, edgeId: edge.id });
            }
          }
        }
      }

      if (currentLevelNodes.size > 0 || currentLevelEdges.size > 0) {
        // We aggregate the previous step's accumulated active sets so the path grows
        const previousStep = steps[steps.length - 1];

        steps.push({
          activeNodes: new Set([...previousStep.activeNodes, ...currentLevelNodes]),
          activeEdges: new Set([...previousStep.activeEdges, ...currentLevelEdges]),
          // Just use the first newly visited node as the "current" one for the label, or null if none
          currentNodeId: currentLevelNodes.values().next().value || null,
        });
      }

      currentLevel = nextLevel;
    }

    setHistory(steps);
    setCurrentStep(0);
    setIsPlaying(true);
  }, [edges]);

  const playNextFrame = useCallback((timestamp: number) => {
    if (!lastFrameTimeRef.current) {
      lastFrameTimeRef.current = timestamp;
    }

    const elapsed = timestamp - lastFrameTimeRef.current;

    if (elapsed >= STEP_DURATION) {
      setCurrentStep((prev) => {
        if (prev < history.length - 1) {
          lastFrameTimeRef.current = timestamp;
          return prev + 1;
        } else {
          // Reached the end
          setIsPlaying(false);
          lastFrameTimeRef.current = 0;
          return prev;
        }
      });
    }

    if (isPlaying) {
      animationRef.current = requestAnimationFrame(playNextFrame);
    }
  }, [history.length, isPlaying]);

  useEffect(() => {
    if (isPlaying && history.length > 0) {
      animationRef.current = requestAnimationFrame(playNextFrame);
    } else if (animationRef.current !== null) {
      cancelAnimationFrame(animationRef.current);
      animationRef.current = null;
      lastFrameTimeRef.current = 0;
    }
    return () => {
      if (animationRef.current !== null) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [isPlaying, playNextFrame, history.length]);

  const start = useCallback((nodeId: string) => {
    calculatePaths(nodeId);
  }, [calculatePaths]);

  const play = useCallback(() => {
    if (currentStep < history.length - 1) {
      setIsPlaying(true);
    } else if (currentStep === history.length - 1) {
      setCurrentStep(0);
      setIsPlaying(true);
    }
  }, [currentStep, history.length]);

  const pause = useCallback(() => {
    setIsPlaying(false);
  }, []);

  const stepForward = useCallback(() => {
    setIsPlaying(false);
    setCurrentStep((prev) => Math.min(prev + 1, history.length - 1));
  }, [history.length]);

  const stepBackward = useCallback(() => {
    setIsPlaying(false);
    setCurrentStep((prev) => Math.max(prev - 1, 0));
  }, []);

  const reset = useCallback(() => {
    setIsPlaying(false);
    setCurrentStep(0);
  }, []);

  const close = useCallback(() => {
    setIsPlaying(false);
    setHistory([]);
    setCurrentStep(-1);
  }, []);

  const currentPathState = history[currentStep] || {
    activeNodes: new Set(),
    activeEdges: new Set(),
    currentNodeId: null,
  };

  return {
    activeNodes: currentPathState.activeNodes,
    activeEdges: currentPathState.activeEdges,
    currentNodeId: currentPathState.currentNodeId,
    currentStep,
    totalSteps: history.length,
    isPlaying,
    start,
    play,
    pause,
    stepForward,
    stepBackward,
    reset,
    close,
  };
};
