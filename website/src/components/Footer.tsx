import React from "react";
import Link from "next/link";
import { Github, FileText, Sparkles, Terminal } from "lucide-react";

export const Footer: React.FC = () => {
  return (
    <footer className="border-t border-zinc-800/80 bg-black/90 py-12 text-zinc-400">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
          <div className="md:col-span-2 space-y-3">
            <div className="flex items-center gap-2">
              <span className="font-mono text-base font-bold text-white tracking-tighter">λ ASL 3.0</span>
              <span className="text-xs text-zinc-500">Open Specification</span>
            </div>
            <p className="text-xs text-zinc-400 max-w-md leading-relaxed">
              Agent Skill Language is an AI-First, hermetic, capability-secure execution framework and deterministic orchestration runtime for autonomous AI agents.
            </p>
            <p className="text-[11px] text-zinc-500">
              Designed & authored by <span className="text-zinc-300 font-medium">Jean Catarina</span>.
            </p>
          </div>

          <div className="space-y-2">
            <h4 className="text-xs font-semibold uppercase tracking-wider text-zinc-200">Ecosystem</h4>
            <ul className="space-y-1.5 text-xs">
              <li><Link href="/docs/tokenomics" className="hover:text-white transition-colors">93.2% Token Reduction</Link></li>
              <li><Link href="/docs/triad" className="hover:text-white transition-colors">The Triad (.skill, .tool, .asl)</Link></li>
              <li><Link href="/docs/axioms" className="hover:text-white transition-colors">The 7 Axioms & OCap</Link></li>
              <li><Link href="/docs/cli" className="hover:text-white transition-colors">CLI & MCP Reference</Link></li>
            </ul>
          </div>

          <div className="space-y-2">
            <h4 className="text-xs font-semibold uppercase tracking-wider text-zinc-200">Scientific Foundation</h4>
            <ul className="space-y-1.5 text-xs">
              <li><Link href="/paper" className="hover:text-white transition-colors">Scientific Paper (Full-Text)</Link></li>
              <li><Link href="/playground" className="hover:text-white transition-colors">WebAssembly Playground</Link></li>
              <li><a href="https://github.com/asl-lang/asl" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">GitHub Repository</a></li>
              <li><a href="https://github.com/asl-lang/asl/blob/main/LICENSE-MIT" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">MIT / Apache-2.0 License</a></li>
            </ul>
          </div>
        </div>

        <div className="mt-8 pt-6 border-t border-zinc-900 flex flex-col sm:flex-row items-center justify-between text-[11px] text-zinc-500">
          <p>© {new Date().getFullYear()} ASL Project & Jean Catarina. Open-source under MIT / Apache 2.0.</p>
          <p className="mt-2 sm:mt-0 font-mono">Zero Ambient Authority • 100% KV-Cache Invariant</p>
        </div>
      </div>
    </footer>
  );
};
