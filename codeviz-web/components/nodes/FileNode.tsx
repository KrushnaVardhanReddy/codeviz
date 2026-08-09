import React from 'react';
import { Handle, Position } from '@xyflow/react';
import { FileCode } from 'lucide-react';
import { NODE_COLORS } from '../../lib/colorMap';

interface FileNodeProps {
  data: {
    label: string;
    language?: string;
  };
}

const FileNode: React.FC<FileNodeProps> = ({ data }) => {
  const colors = NODE_COLORS['File'];

  return (
    <div
      className="w-[150px] rounded-lg shadow-sm flex flex-col overflow-hidden cursor-move transition-colors"
      style={{
        backgroundColor: colors.bg,
        borderColor: colors.border,
        borderWidth: '1px',
        borderStyle: 'solid'
      }}
    >
      <Handle type="target" position={Position.Top} />

      <div
        className="px-2 py-1 flex items-center gap-2"
        style={{
          backgroundColor: colors.bg,
          borderBottomColor: colors.border,
          borderBottomWidth: '1px',
          borderBottomStyle: 'solid'
        }}
      >
        <div className="w-4 h-4 rounded-sm flex items-center justify-center bg-white/10">
          <FileCode size={14} color={colors.border} />
        </div>
        <span className="font-mono text-[13px] text-white truncate">{data.label}</span>
      </div>

      {data.language && (
        <div className="p-2 font-mono text-[11px] text-gray-300 flex flex-col gap-1">
          <span
            className="px-1 rounded self-start"
            style={{
              color: colors.border,
              backgroundColor: `${colors.border}20`
            }}
          >
            {data.language}
          </span>
        </div>
      )}

      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};

export default FileNode;
