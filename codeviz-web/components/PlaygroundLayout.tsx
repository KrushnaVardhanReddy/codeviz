"use client";

import React, { useState, useEffect, useCallback, useRef } from "react";
import PlaygroundEditor from "./PlaygroundEditor";
import { GraphCanvas } from "./GraphCanvas";
import initWasm, { parse_and_build_graph } from "../../codeviz-wasm/pkg/codeviz_wasm";

const EXAMPLES = {
  python: `import os
from pathlib import Path

class Animal:
    def __init__(self):
        pass

class Dog(Animal):
    def bark(self):
        print("Woof!")

def main():
    d = Dog()
    d.bark()

if __name__ == "__main__":
    main()
`,
  typescript: `import React from 'react';

interface Props {
  name: string;
}

export class Greeter extends React.Component<Props> {
  render() {
    return <div>Hello {this.props.name}</div>;
  }
}

export const App = () => {
  return <Greeter name="World" />;
};
`
};

export function PlaygroundLayout() {
  const [code, setCode] = useState(EXAMPLES.python);
  const [language, setLanguage] = useState("python");
  const [graphData, setGraphData] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [wasmLoaded, setWasmLoaded] = useState(false);
  const parserRef = useRef<any>(null);
  const wasmInitRef = useRef<boolean>(false);
  const debounceRef = useRef<NodeJS.Timeout | null>(null);
  const ParserModuleRef = useRef<any>(null);

  useEffect(() => {
    async function loadDependencies() {
      try {
        if (!wasmInitRef.current) {
          await initWasm();

          // dynamic import to avoid next.js build errors for server components
          const Parser = await import('web-tree-sitter');
          const ParserMod = (Parser as any).default || Parser;
          ParserModuleRef.current = ParserMod;

          await ParserMod.init({
            locateFile(scriptName: string, scriptDirectory: string) {
              return `/tree-sitter-wasms/${scriptName}`;
            }
          });
          parserRef.current = new ParserMod();
          wasmInitRef.current = true;
          setWasmLoaded(true);
        }
      } catch (err: any) {
        setError("Failed to initialize WASM: " + err.message);
      }
    }
    loadDependencies();
  }, []);

  const parseAndRender = useCallback(async (currentCode: string, currentLang: string) => {
    if (!wasmLoaded || !parserRef.current || !ParserModuleRef.current) return;

    try {
      const ParserMod = ParserModuleRef.current;
      const langUrl = `/tree-sitter-wasms/tree-sitter-${currentLang}.wasm`;
      const Lang = await ParserMod.Language.load(langUrl);
      parserRef.current.setLanguage(Lang);

      const tree = parserRef.current.parse(currentCode);

      // Basic recursive function to extract JSON AST
      const extractNode = (node: any): any => {
        return {
          type: node.type,
          text: node.text,
          start_position: { row: node.startPosition.row, column: node.startPosition.column },
          end_position: { row: node.endPosition.row, column: node.endPosition.column },
          children: node.children.map(extractNode)
        };
      };

      const astJson = JSON.stringify(extractNode(tree.rootNode));

      const filename = currentLang === "python" ? "main.py" : "main.ts";
      const graphJson = parse_and_build_graph(currentLang, filename, astJson);
      setGraphData(JSON.parse(graphJson));
      setError(null);
    } catch (err: any) {
      setError(err.message || "Parse error");
    }
  }, [wasmLoaded]);

  useEffect(() => {
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = setTimeout(() => {
      parseAndRender(code, language);
    }, 500);

    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, [code, language, parseAndRender]);

  const handleLanguageChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newLang = e.target.value;
    setLanguage(newLang);
    setCode(EXAMPLES[newLang as keyof typeof EXAMPLES] || "");
  };

  return (
    <div className="flex h-full w-full bg-gray-50 overflow-hidden">
      <div className="w-1/2 flex flex-col border-r border-gray-200 h-full">
        <div className="p-4 bg-white border-b border-gray-200 flex justify-between items-center">
          <div className="flex items-center space-x-2">
            <span className="font-semibold text-gray-700">Language:</span>
            <select
              value={language}
              onChange={handleLanguageChange}
              className="border border-gray-300 rounded px-2 py-1 text-sm bg-white"
            >
              <option value="python">Python</option>
              <option value="typescript">TypeScript</option>
            </select>
          </div>
          {error && (
            <div className="text-red-500 text-xs truncate max-w-xs" title={error}>
              {error}
            </div>
          )}
        </div>
        <div className="flex-1 overflow-hidden" data-testid="playground-editor">
          <PlaygroundEditor
            code={code}
            onChange={(val) => setCode(val || "")}
            language={language}
          />
        </div>
      </div>

      <div className="w-1/2 h-full flex flex-col">
        <div className="p-4 bg-white border-b border-gray-200 flex justify-between items-center h-[60px]">
          <span className="font-semibold text-gray-700">Graph Preview</span>
        </div>
        <div className="flex-1 bg-white relative">
          {!wasmLoaded && (
            <div className="absolute inset-0 flex items-center justify-center bg-white bg-opacity-75 z-10">
              Loading WASM engines...
            </div>
          )}
          {graphData && <GraphCanvas graph={graphData} />}
        </div>
      </div>
    </div>
  );
}
