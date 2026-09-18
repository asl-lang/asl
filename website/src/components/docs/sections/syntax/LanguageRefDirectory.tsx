import React from "react";
import Link from "next/link";
import { ArrowRight, Binary, Workflow, Terminal, Shield } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const LanguageRefDirectory: React.FC = () => {
  return (
    <DocsSection
      title="4. Complete Language Reference Manual"
      subtitle="Deep-dive into the formal language specifications across dedicated reference chapters:"
    >
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-2">
        <Link
          href="/docs/types"
          className="group rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2 hover:border-zinc-700 transition-colors"
        >
          <div className="flex items-center justify-between">
            <span className="font-bold text-white flex items-center gap-2">
              <Binary className="h-4 w-4 text-emerald-400" />
              <span>Types &amp; Data Model</span>
            </span>
            <ArrowRight className="h-4 w-4 text-zinc-500 group-hover:text-white transition-colors" />
          </div>
          <p className="text-xs text-zinc-400">
            Primitives (<code className="text-zinc-300">int, bool, string, None</code>), containers (<code className="text-zinc-300">list, dict, tuple, struct</code>), and JSON Schema cross-compilation.
          </p>
        </Link>

        <Link
          href="/docs/control-flow"
          className="group rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2 hover:border-zinc-700 transition-colors"
        >
          <div className="flex items-center justify-between">
            <span className="font-bold text-white flex items-center gap-2">
              <Workflow className="h-4 w-4 text-blue-400" />
              <span>Variables, Loops &amp; Functions</span>
            </span>
            <ArrowRight className="h-4 w-4 text-zinc-500 group-hover:text-white transition-colors" />
          </div>
          <p className="text-xs text-zinc-400">
            Lexical scoping, conditionals, bounded <code className="text-zinc-300">for</code> loops, halting proofs (why <code className="text-rose-400">while</code> and recursion are banned), and entrypoints.
          </p>
        </Link>

        <Link
          href="/docs/stdlib"
          className="group rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2 hover:border-zinc-700 transition-colors"
        >
          <div className="flex items-center justify-between">
            <span className="font-bold text-white flex items-center gap-2">
              <Terminal className="h-4 w-4 text-purple-400" />
              <span>Standard Library &amp; Builtins</span>
            </span>
            <ArrowRight className="h-4 w-4 text-zinc-500 group-hover:text-white transition-colors" />
          </div>
          <p className="text-xs text-zinc-400">
            Complete reference for string, list, and dict methods, plus <code className="text-zinc-300">json.encode</code>, <code className="text-zinc-300">json.decode</code>, and <code className="text-zinc-300">struct()</code>.
          </p>
        </Link>

        <Link
          href="/docs/context"
          className="group rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2 hover:border-zinc-700 transition-colors"
        >
          <div className="flex items-center justify-between">
            <span className="font-bold text-white flex items-center gap-2">
              <Shield className="h-4 w-4 text-amber-400" />
              <span>Capability Context (ctx)</span>
            </span>
            <ArrowRight className="h-4 w-4 text-zinc-500 group-hover:text-white transition-colors" />
          </div>
          <p className="text-xs text-zinc-400">
            Object Capability (OCap) host interface: <code className="text-zinc-300">ctx.fs</code>, <code className="text-zinc-300">ctx.crypto</code>, <code className="text-zinc-300">ctx.fuel</code>, and sandbox containment.
          </p>
        </Link>
      </div>
    </DocsSection>
  );
};
