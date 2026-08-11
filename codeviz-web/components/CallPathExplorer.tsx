import React from 'react';
import { Play, Pause, SkipForward, SkipBack, RotateCcw, X } from 'lucide-react';
import { Node as ReactFlowNode } from '@xyflow/react';

interface CallPathExplorerProps {
  currentStep: number;
  totalSteps: number;
  isPlaying: boolean;
  currentNodeId: string | null;
  nodes: ReactFlowNode[];
  onPlay: () => void;
  onPause: () => void;
  onStepForward: () => void;
  onStepBackward: () => void;
  onReset: () => void;
  onClose: () => void;
}

export const CallPathExplorer: React.FC<CallPathExplorerProps> = ({
  currentStep,
  totalSteps,
  isPlaying,
  currentNodeId,
  nodes,
  onPlay,
  onPause,
  onStepForward,
  onStepBackward,
  onReset,
  onClose,
}) => {
  if (currentStep < 0) return null;

  // Find node label for the current step
  const currentNode = currentNodeId ? nodes.find((n) => n.id === currentNodeId) : null;
  let labelText = '...';
  if (currentNode) {
    const labelData = currentNode.data?.label;
    if (typeof labelData === 'string') {
      labelText = labelData;
    } else if (labelData && typeof labelData === 'object' && 'props' in labelData) {
      labelText = (labelData as any).props?.children || currentNode.id;
    } else {
      labelText = currentNode.id;
    }
  } else if (currentStep === totalSteps - 1) {
      labelText = 'End of path';
  }

  return (
    <div className="absolute top-4 left-1/2 -translate-x-1/2 bg-slate-900/95 backdrop-blur-md border border-slate-700/50 rounded-xl shadow-2xl p-4 flex flex-col items-center gap-3 z-50 min-w-[320px]" data-testid="call-path-explorer">
      <div className="flex justify-between items-center w-full">
        <div className="text-slate-300 font-semibold text-sm">
          Call Path Explorer
        </div>
        <button
          onClick={onClose}
          className="p-1 text-slate-400 hover:text-white hover:bg-slate-800 rounded-md transition-colors"
          title="Close Explorer"
        >
          <X size={16} />
        </button>
      </div>

      <div className="text-center w-full bg-slate-950/50 rounded-md p-2 border border-slate-800">
        <div className="text-xs text-slate-400 mb-1">
          Step {currentStep + 1} of {totalSteps}
        </div>
        <div className="text-sm text-green-400 font-mono truncate max-w-[280px]">
          {labelText}
        </div>
      </div>

      <div className="flex items-center gap-2 mt-1">
        <button
          onClick={onReset}
          className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition-colors"
          title="Restart"
        >
          <RotateCcw size={18} />
        </button>

        <div className="w-px h-6 bg-slate-700 mx-1"></div>

        <button
          onClick={onStepBackward}
          disabled={currentStep === 0}
          className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition-colors disabled:opacity-50 disabled:hover:bg-transparent disabled:hover:text-slate-400"
          title="Step Backward"
        >
          <SkipBack size={20} />
        </button>

        {isPlaying ? (
          <button
            onClick={onPause}
            className="p-3 bg-green-600/20 text-green-500 hover:bg-green-600/30 rounded-full transition-colors mx-1"
            title="Pause"
          >
            <Pause size={24} className="fill-current" />
          </button>
        ) : (
          <button
            onClick={onPlay}
            disabled={currentStep === totalSteps - 1}
            className="p-3 bg-green-600/20 text-green-500 hover:bg-green-600/30 rounded-full transition-colors mx-1 disabled:opacity-50 disabled:hover:bg-green-600/20"
            title="Play"
          >
            <Play size={24} className="fill-current ml-1" />
          </button>
        )}

        <button
          onClick={onStepForward}
          disabled={currentStep === totalSteps - 1}
          className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition-colors disabled:opacity-50 disabled:hover:bg-transparent disabled:hover:text-slate-400"
          title="Step Forward"
        >
          <SkipForward size={20} />
        </button>
      </div>

      {/* Progress Bar */}
      <div className="w-full h-1.5 bg-slate-800 rounded-full overflow-hidden mt-1">
        <div
          className="h-full bg-green-500 transition-all duration-300 ease-in-out"
          style={{ width: `${((currentStep) / Math.max(1, totalSteps - 1)) * 100}%` }}
        ></div>
      </div>
    </div>
  );
};
