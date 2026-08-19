'use client';

import React from 'react';
import { ChevronRight, LayoutGrid } from 'lucide-react';

export interface BreadcrumbEntry {
  id: string;
  label: string;
  kind: 'classes' | 'class' | 'function';
}

interface DrillBreadcrumbProps {
  crumbs: BreadcrumbEntry[];
  onNavigate: (index: number) => void;
}

export const DrillBreadcrumb: React.FC<DrillBreadcrumbProps> = ({ crumbs, onNavigate }) => {
  if (crumbs.length === 0) return null;

  return (
    <div className="absolute top-3 left-3 z-20 flex items-center gap-1 bg-slate-900/90 border border-slate-700 rounded-lg px-3 py-1.5 text-sm backdrop-blur-sm shadow-xl">
      {/* Root: All Classes */}
      <button
        onClick={() => onNavigate(-1)}
        className="flex items-center gap-1.5 text-slate-400 hover:text-blue-400 transition-colors font-medium"
      >
        <LayoutGrid className="w-3.5 h-3.5" />
        <span>All Classes</span>
      </button>

      {crumbs.map((crumb, i) => (
        <React.Fragment key={crumb.id}>
          <ChevronRight className="w-3 h-3 text-slate-600 flex-shrink-0" />
          <button
            onClick={() => onNavigate(i)}
            className={`
              px-1.5 py-0.5 rounded transition-colors font-mono text-xs max-w-[180px] truncate
              ${i === crumbs.length - 1
                ? 'text-emerald-400 font-semibold cursor-default'
                : 'text-slate-300 hover:text-white hover:bg-slate-700'
              }
            `}
            title={crumb.label}
          >
            {crumb.label}
          </button>
        </React.Fragment>
      ))}
    </div>
  );
};
