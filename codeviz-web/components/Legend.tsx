import React from 'react';

export const Legend: React.FC = () => {
  return (
    <div className="absolute bottom-4 left-4 right-4 bg-slate-900/80 backdrop-blur-md border border-slate-700/30 rounded p-3 text-xs text-slate-300 font-medium z-10">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-2">
        <div className="flex flex-wrap gap-4">
          <span className="flex items-center gap-1"><span className="text-orange-500">🔷</span> Class</span>
          <span className="flex items-center gap-1"><span className="text-yellow-500">◇</span> Interface</span>
          <span className="flex items-center gap-1"><span className="text-green-500">ƒ</span> Function</span>
          <span className="flex items-center gap-1"><span className="text-purple-500">📦</span> Module</span>
          <span className="flex items-center gap-1"><span className="text-blue-500">📄</span> File</span>
        </div>
        <div className="flex flex-wrap gap-4 text-[10px] sm:text-xs">
          <span className="flex items-center gap-1"><span className="w-4 h-0.5 bg-blue-500"></span> Imports</span>
          <span className="flex items-center gap-1"><span className="w-4 h-0.5 bg-green-500"></span> Calls</span>
          <span className="flex items-center gap-1"><span className="w-4 h-0.5 bg-orange-500"></span> Inherits</span>
          <span className="flex items-center gap-1"><span className="w-4 border-t border-dashed border-yellow-500"></span> Implements</span>
          <span className="flex items-center gap-1"><span className="w-4 border-t border-dotted border-gray-500"></span> Returns</span>
          <span className="flex items-center gap-1"><span className="w-4 h-0.5 bg-purple-500"></span> Instantiates</span>
        </div>
      </div>
    </div>
  );
};
