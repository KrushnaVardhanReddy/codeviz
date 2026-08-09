import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { NODE_COLORS } from '../../lib/colorMap';

interface InterfaceNodeProps {
  data: {
    label: string;
  };
}

const InterfaceNode: React.FC<InterfaceNodeProps> = ({ data }) => {
  const colors = NODE_COLORS['Interface'];

  return (
    <div
      className="px-4 py-2 flex items-center justify-center gap-2 cursor-move transition-colors"
      style={{
        backgroundColor: colors.bg,
        borderColor: colors.border,
        borderWidth: '1px',
        borderStyle: 'dashed'
      }}
    >
      <Handle type="target" position={Position.Top} />

      <div
        className="w-4 h-4 flex items-center justify-center rotate-45 border"
        style={{ borderColor: colors.border }}
      >
        {/* Diamond (outline) equivalent */}
      </div>
      <span className="font-mono text-[13px] font-bold text-white truncate">{data.label}</span>

      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};

export default InterfaceNode;
