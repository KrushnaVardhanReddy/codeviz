import React from 'react';
import {
  FolderOpen,
  Folder,
  GitBranch,
  Network,
  History,
  FileText,
  HelpCircle,
  FileCode,
  Box,
  Plus
} from 'lucide-react';

export const Sidebar: React.FC = () => {
  return (
    <aside className="bg-slate-900/70 backdrop-blur-lg fixed left-0 top-16 h-[calc(100vh-64px)] w-64 border-r border-slate-700/10 flex flex-col py-4 gap-4 z-40 hidden md:flex">
      <div className="px-4 mb-4">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-10 h-10 rounded bg-slate-800 border border-slate-700/30 flex items-center justify-center overflow-hidden">
            <Network className="text-blue-500 w-5 h-5" />
          </div>
          <div>
            <div className="font-semibold text-blue-500 text-sm leading-tight">src/core</div>
            <div className="text-slate-400 text-xs">main branch</div>
          </div>
        </div>
        <button className="w-full bg-blue-500/10 border border-blue-500/30 text-blue-500 py-2 rounded text-sm font-medium hover:bg-blue-500/20 transition-colors flex items-center justify-center gap-2">
          <Plus className="w-4 h-4" /> New Analysis
        </button>
      </div>

      <div className="flex-1 overflow-y-auto px-2">
        <ul className="space-y-1">
          <li>
            <a className="flex items-center gap-3 py-2 text-cyan-400 font-bold border-l-2 border-cyan-400 pl-2 text-sm cursor-pointer hover:bg-slate-800/50 transition-all rounded-r" href="#">
              <FolderOpen className="w-4 h-4" /> Explorer
            </a>
          </li>
          <li>
            <a className="flex items-center gap-3 py-2 text-slate-400 hover:text-slate-200 pl-2 text-sm cursor-pointer hover:bg-slate-800/50 transition-all rounded-r" href="#">
              <GitBranch className="w-4 h-4" /> Branches
            </a>
          </li>
          <li>
            <a className="flex items-center gap-3 py-2 text-slate-400 hover:text-slate-200 pl-2 text-sm cursor-pointer hover:bg-slate-800/50 transition-all rounded-r" href="#">
              <Network className="w-4 h-4" /> Graph
            </a>
          </li>
          <li>
            <a className="flex items-center gap-3 py-2 text-slate-400 hover:text-slate-200 pl-2 text-sm cursor-pointer hover:bg-slate-800/50 transition-all rounded-r" href="#">
              <History className="w-4 h-4" /> History
            </a>
          </li>
        </ul>

        <div className="mt-8 px-2">
          <div className="text-xs text-slate-400 mb-2 uppercase tracking-wider font-semibold">Project Files</div>
          <ul className="space-y-1">
            <li className="flex items-center gap-2 py-1 text-slate-400 hover:text-slate-200 cursor-pointer text-sm pl-2">
              <Folder className="w-4 h-4" /> src
            </li>
            <li className="flex items-center gap-2 py-1 text-slate-400 hover:text-slate-200 cursor-pointer text-sm pl-6">
              <Folder className="w-4 h-4" /> components
            </li>
            <li className="flex items-center gap-2 py-1 text-slate-200 hover:text-blue-500 cursor-pointer text-sm pl-6">
              <FolderOpen className="w-4 h-4 text-cyan-400" /> core
            </li>
            <li className="flex items-center gap-2 py-1 text-slate-400 hover:text-slate-200 cursor-pointer text-sm pl-10">
              <FileCode className="w-3.5 h-3.5 text-purple-400" /> App.tsx
            </li>
            <li className="flex items-center gap-2 py-1 text-slate-400 hover:text-slate-200 cursor-pointer text-sm pl-10">
              <FileCode className="w-3.5 h-3.5 text-purple-400" /> Layout.tsx
            </li>
          </ul>
        </div>
      </div>

      <div className="px-2 mt-auto border-t border-slate-700/10 pt-4">
        <ul className="space-y-1">
          <li>
            <a className="flex items-center gap-3 py-2 text-slate-400 hover:text-slate-200 pl-2 text-sm cursor-pointer transition-all" href="#">
              <FileText className="w-4 h-4" /> Docs
            </a>
          </li>
          <li>
            <a className="flex items-center gap-3 py-2 text-slate-400 hover:text-slate-200 pl-2 text-sm cursor-pointer transition-all" href="#">
              <HelpCircle className="w-4 h-4" /> Help
            </a>
          </li>
        </ul>
      </div>
    </aside>
  );
};
