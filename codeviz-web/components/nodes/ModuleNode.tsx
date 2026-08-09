import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { Package } from 'lucide-react';
import { NODE_COLORS } from '../../lib/colorMap';

interface ModuleNodeProps {
  data: {
    label: string;
  };
}

const ModuleNode: React.FC<ModuleNodeProps> = ({ data }) => {
  const colors = NODE_COLORS['Module'];

  return (
    <div
      className="min-w-[150px] rounded-lg shadow-sm flex flex-col overflow-hidden cursor-move transition-colors"
      style={{
        backgroundColor: colors.bg,
        borderColor: colors.border,
        borderWidth: '1px',
        borderStyle: 'solid'
      }}
    >
      <Handle type="target" position={Position.Top} />

      <div
        className="px-2 py-2 flex items-center justify-center gap-2"
        style={{
          backgroundColor: colors.bg,
          borderBottomColor: colors.border,
          borderBottomWidth: '1px',
          borderBottomStyle: 'solid'
        }}
      >
        <Package size={16} color={colors.border} />
        <span className="font-mono text-[13px] font-bold text-white truncate">{data.label}</span>
      </div>

      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};

export default ModuleNode;
