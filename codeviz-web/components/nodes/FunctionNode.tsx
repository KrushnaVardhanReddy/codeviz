import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { FunctionSquare, Zap } from 'lucide-react';
import { NODE_COLORS } from '../../lib/colorMap';

interface FunctionNodeProps {
  data: {
    label: string;
    isAsync?: boolean;
  };
}

const FunctionNode: React.FC<FunctionNodeProps> = ({ data }) => {
  const colors = data.isAsync ? NODE_COLORS['Async Fn'] : NODE_COLORS['Function'];

  return (
    <div className="relative">
      <div
        className="px-4 py-2 rounded-full flex items-center justify-center gap-2 shadow-sm cursor-move transition-colors"
        style={{
          backgroundColor: colors.bg,
          borderColor: colors.border,
          borderWidth: '1px',
          borderStyle: 'solid'
        }}
      >
        <Handle type="target" position={Position.Top} />

        <FunctionSquare size={16} color={colors.border} />
        <span className="font-mono text-[13px] text-white truncate">{data.label}</span>

        <Handle type="source" position={Position.Bottom} />
      </div>

      {data.isAsync && (
        <div
          className="absolute -top-2 -right-2 bg-yellow-400 rounded-full w-5 h-5 flex items-center justify-center shadow"
          title="Async Function"
        >
          <Zap size={12} color="#000" className="fill-current" />
        </div>
      )}
    </div>
  );
};

export default FunctionNode;
