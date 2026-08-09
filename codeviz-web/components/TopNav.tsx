import React from 'react';
import { Search, Settings, User } from 'lucide-react';

export const TopNav: React.FC = () => {
  return (
    <nav className="h-16 bg-slate-900 border-b border-slate-700 flex items-center justify-between px-6 fixed top-0 w-full z-50">
      <div className="flex items-center">
        <span className="text-xl font-bold text-blue-500 tracking-tight">CodeViz</span>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2 relative">
          <Search className="w-4 h-4 text-slate-400 absolute left-3" />
          <input
            className="bg-slate-800 border border-slate-700 rounded text-slate-200 pl-10 pr-4 py-1.5 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500 text-sm w-64 transition-colors"
            placeholder="Search architecture..."
            type="text"
          />
        </div>

        <button className="text-slate-400 hover:text-blue-500 transition-colors duration-200">
          <Settings className="w-5 h-5" />
        </button>
        <button className="text-slate-400 hover:text-blue-500 transition-colors duration-200">
          <User className="w-5 h-5" />
        </button>
      </div>
    </nav>
  );
};
