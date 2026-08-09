"use client";

import { PlaygroundLayout } from "@/components/PlaygroundLayout";
import { TopNav } from "@/components/TopNav";

export default function PlaygroundPage() {
  return (
    <div className="flex flex-col h-screen min-h-screen bg-white pt-16">
      <TopNav />
      <div className="flex-1 overflow-hidden">
        <PlaygroundLayout />
      </div>
    </div>
  );
}
