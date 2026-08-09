"use client";

import React from "react";
import Editor from "@monaco-editor/react";

interface PlaygroundEditorProps {
  code: string;
  language: string;
  onChange: (value: string | undefined) => void;
}

export default function PlaygroundEditor({ code, language, onChange }: PlaygroundEditorProps) {
  return (
    <Editor
      height="100%"
      language={language}
      value={code}
      onChange={onChange}
      theme="vs-light"
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        wordWrap: "on",
        scrollBeyondLastLine: false,
      }}
    />
  );
}
