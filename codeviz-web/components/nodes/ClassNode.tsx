import React from 'react';
import { Handle, Position } from '@xyflow/react';

import { NODE_COLORS } from '../../lib/colorMap';

interface ClassNodeProps {
  data: {
    label: string;
  };
}

const ClassNode: React.FC<ClassNodeProps> = ({ data }) => {
  const colors = NODE_COLORS['Class'];

  return (
    <div
      className="px-4 py-2 flex items-center justify-center gap-2 cursor-move transition-colors"
      style={{
        backgroundColor: colors.bg,
        borderColor: colors.border,
        borderWidth: '2px', // bold border
        borderStyle: 'solid'
      }}
    >
      <Handle type="target" position={Position.Top} />

      <div
        className="w-4 h-4 flex items-center justify-center rotate-45"
        style={{ backgroundColor: colors.border }}
      >
        {/* Diamond equivalent */}
      </div>
      <span className="font-mono text-[13px] font-bold text-white truncate">{data.label}</span>

      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};

export default ClassNode;
