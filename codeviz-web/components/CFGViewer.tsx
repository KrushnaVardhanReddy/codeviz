import React from 'react';

interface CFGViewerProps {
  sourceSnippet?: string;
}

const CFGViewer: React.FC<CFGViewerProps> = ({ sourceSnippet }) => {
  // If the snippet is empty or missing, render a fallback
  if (!sourceSnippet) {
    return (
      <div className="bg-gray-800/50 border border-gray-700 border-dashed rounded-md p-4 text-center text-sm text-gray-400">
        No source logic available for CFG visualization.
      </div>
    );
  }

  // Mocking the Stitch CFG flow blocks specifically for the demo processData snippet
  // In Phase 12, this will be dynamically generated from the backend CFG JSON IR
  const isDemo = sourceSnippet.includes('processData');

  return (
    <div className="space-y-3 font-mono text-xs w-full">
      {isDemo ? (
        <div className="relative border-l-2 border-purple-500 pl-4 space-y-2 ml-2">
          {/* Function Signature */}
          <div className="bg-gray-900/60 border border-gray-700 rounded-md p-2 shadow-sm text-blue-300 transition-colors hover:bg-gray-800/80 hover:border-purple-500 cursor-default">
            <span className="text-purple-400">async function</span> <span className="text-blue-300">processData</span>(rawData)
          </div>

          {/* Variable declaration */}
          <div className="bg-gray-900/60 border border-gray-700 rounded-md p-2 shadow-sm text-gray-300 transition-colors hover:bg-gray-800/80 hover:border-purple-500 cursor-default">
            <span className="text-pink-400">let</span> results = [];
          </div>

          {/* For Loop */}
          <div className="relative border-l-2 border-cyan-500 pl-4 space-y-2 mt-2 ml-2">
            <div className="absolute -left-[18px] top-3 w-4 h-0.5 bg-cyan-500"></div>
            <div className="bg-gray-900/60 border border-gray-700 rounded-md p-2 shadow-sm text-cyan-300 transition-colors hover:bg-gray-800/80 hover:border-cyan-400 cursor-default">
              <span className="text-pink-400">for</span> (const item of rawData)
            </div>

            {/* If Statement */}
            <div className="relative border-l-2 border-emerald-500 pl-4 space-y-2 mt-2 ml-2">
              <div className="absolute -left-[18px] top-3 w-4 h-0.5 bg-emerald-500"></div>
              <div className="bg-gray-900/60 border border-gray-700 rounded-md p-2 shadow-sm text-emerald-300 transition-colors hover:bg-gray-800/80 hover:border-emerald-400 cursor-default">
                <span className="text-pink-400">if</span> (isValid(item))
              </div>

              {/* Action */}
              <div className="relative border-l-2 border-gray-600 pl-4 space-y-2 mt-2 ml-2">
                <div className="absolute -left-[18px] top-3 w-4 h-0.5 bg-gray-600"></div>
                <div className="bg-gray-900/60 border border-gray-700 rounded-md p-2 shadow-sm text-gray-300 transition-colors hover:bg-gray-800/80 hover:border-gray-400 cursor-default">
                  results.<span className="text-blue-300">push</span>(transform(item));
                </div>
              </div>
            </div>
          </div>

          {/* Return Statement */}
          <div className="bg-gray-900/60 border border-gray-700 rounded-md p-2 shadow-sm text-gray-300 mt-2 transition-colors hover:bg-gray-800/80 hover:border-purple-500 cursor-default">
            <span className="text-pink-400">return</span> results;
          </div>
        </div>
      ) : (
        <div className="bg-gray-800/50 border border-gray-700 border-dashed rounded-md p-4 text-center text-sm text-gray-400">
          CFG rendering is currently optimized for mocked demo data. Real CFG JSON expected from backend.
        </div>
      )}
    </div>
  );
};

export default CFGViewer;
