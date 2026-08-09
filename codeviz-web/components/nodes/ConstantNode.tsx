import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { Pi } from 'lucide-react';
import { NODE_COLORS } from '../../lib/colorMap';

interface ConstantNodeProps {
  data: {
    label: string;
  };
}

const ConstantNode: React.FC<ConstantNodeProps> = ({ data }) => {
  const colors = NODE_COLORS['Constant'];

  return (
    <div
      className="px-3 py-1 rounded shadow-sm flex items-center justify-center gap-1 cursor-move transition-colors"
      style={{
        backgroundColor: colors.bg,
        borderColor: colors.border,
        borderWidth: '1px',
        borderStyle: 'solid'
      }}
    >
      <Handle type="target" position={Position.Top} />

      <Pi size={12} color={colors.border} />
      <span className="font-mono text-[11px] text-gray-300 truncate">{data.label}</span>

      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};

export default ConstantNode;
