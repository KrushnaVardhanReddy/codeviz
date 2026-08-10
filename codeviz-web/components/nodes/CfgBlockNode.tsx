import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { getCfgNodeStyle } from '../../lib/cfgColorMap';
import { CfgBlockKind } from '../../lib/graphTypes';

interface CfgBlockNodeProps {
  data: {
    label: string;
    kind: CfgBlockKind;
    line?: number | null;
  };
}

export const CfgBlockNode: React.FC<CfgBlockNodeProps> = ({ data }) => {
  const style = getCfgNodeStyle(data.kind);

  const isDiamond = style.shape === 'Diamond';
  const isOval = style.shape === 'Oval';
  const isPill = style.shape === 'Pill';
  const isOctagon = style.shape === 'Octagon';

  // Base styles
  let containerClass = "flex items-center justify-center relative";
  let wrapperStyle: React.CSSProperties = {
    backgroundColor: style.bg,
    borderColor: style.border,
    borderWidth: '2px',
    borderStyle: 'solid',
    color: '#ffffff',
    fontSize: '12px',
    fontFamily: 'monospace',
    textAlign: 'center',
    boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1)',
  };

  let contentStyle: React.CSSProperties = {
    padding: '8px',
    maxWidth: '120px',
    wordWrap: 'break-word'
  };

  if (isDiamond) {
    // 80x80 square rotated 45deg
    wrapperStyle.width = '80px';
    wrapperStyle.height = '80px';
    wrapperStyle.transform = 'rotate(45deg)';
    wrapperStyle.borderRadius = '8px';

    // Counter-rotate the inner text
    contentStyle.transform = 'rotate(-45deg)';
    contentStyle.width = '100px';
    contentStyle.position = 'absolute';
    // Diamond logic is trickier with handles, React Flow places handles relative to the 80x80 container.
  } else if (isOval) {
    wrapperStyle.borderRadius = '50%';
    wrapperStyle.minWidth = '80px';
    wrapperStyle.minHeight = '40px';
    wrapperStyle.padding = '8px 16px';
  } else if (isPill) {
    wrapperStyle.borderRadius = '9999px';
    wrapperStyle.padding = '8px 16px';
    wrapperStyle.minWidth = '100px';
  } else if (isOctagon) {
    // CSS Clip path for octagon
    wrapperStyle.clipPath = 'polygon(30% 0%, 70% 0%, 100% 30%, 100% 70%, 70% 100%, 30% 100%, 0% 70%, 0% 30%)';
    wrapperStyle.padding = '16px';
    wrapperStyle.minWidth = '100px';
    wrapperStyle.minHeight = '100px';
    // When using clip-path, border might not render exactly on the edge as expected unless padded.
    // For simplicity, we just rely on bg color and a slight inner shadow or just standard styling
  } else {
    // Rectangle
    wrapperStyle.borderRadius = '4px';
    wrapperStyle.minWidth = '120px';
    wrapperStyle.padding = '8px';
  }

  // Adjust handle positions for diamond (which rotates the whole wrapper by 45deg)
  // If the wrapper is rotated 45deg, the Top handle (Position.Top) points Top-Left in visual space.
  // We want visual Top to map to logical Top-Left in the rotated div, etc.
  // Actually, to make React Flow routing work easily without calculating rotation offsets,
  // we can wrap the styled div in a non-rotated container and put handles on the container.
  return (
    <div className="relative flex items-center justify-center group" style={{ minWidth: '100px', minHeight: '100px' }} data-testid={`cfg-node-${data.kind}`}>
      <Handle type="target" position={Position.Top} className="!w-2 !h-2 !bg-gray-400 z-10" />

      {/* Visual node shape */}
      <div className={containerClass} style={wrapperStyle}>
        <div style={contentStyle} className="truncate" title={data.label}>
          {data.kind === 'LoopHeader' && <span className="mr-1">↻</span>}
          {data.label}
        </div>
      </div>

      {/* For true/false branches from Condition, we can place a left/right/bottom handle or just rely on multiple bottom edges handled by React Flow */}
      <Handle type="source" position={Position.Bottom} className="!w-2 !h-2 !bg-gray-400 z-10" />
      <Handle type="source" position={Position.Left} id="left" className="!w-2 !h-2 !bg-gray-400 z-10" />
      <Handle type="source" position={Position.Right} id="right" className="!w-2 !h-2 !bg-gray-400 z-10" />
    </div>
  );
};
