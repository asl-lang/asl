import React from "react";
import Link from "next/link";
import { ArrowRight, Layers, FileCode, CheckCircle2, ShieldCheck } from "lucide-react";
import { CodeSwitcher } from "@/components/CodeSwitcher";

export default function TriadDocsPage() {
  return (
    <div className="space-y-8">
      <div>
        <div className="text-xs font-mono font-medium text-purple-400 uppercase tracking-wider">
          Architecture • Track 2
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          The Canonical Triad (.skill, .tool, .asl)
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed">
          How ASL 3.0 unifies autonomous agent capabilities, direct MCP tool calls, and native specifications under a polymorphic AST.
        </p>
      </div>

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <p>
          In traditional agent architectures, developers are forced to choose between incompatible formats: prompt files (<code className="text-zinc-200">.prompt.md</code>), agent descriptions (<code className="text-zinc-200">.agent</code>), or Python scripts. ASL replaces this fragmentation with the <strong>Canonical Triad</strong>:
        </p>

        {/* The 3 Formats Table */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 overflow-hidden font-mono text-xs">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="border-b border-zinc-800 bg-zinc-900/60 text-zinc-400">
                <th className="p-3">Extension</th>
                <th className="p-3">Role & Purpose</th>
                <th className="p-3">Shadow Projection (.md)</th>
                <th className="p-3">Primary Engine</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-850 text-zinc-300">
              <tr>
                <td className="p-3 font-bold text-blue-400">.skill</td>
                <td className="p-3">Autonomous agent capability with semantic instructions & rules</td>
                <td className="p-3 text-emerald-400 font-semibold">✅ Enabled (.skill.md)</td>
                <td className="p-3 text-zinc-400">asl-vm-starlark</td>
              </tr>
              <tr>
                <td className="p-3 font-bold text-emerald-400">.tool</td>
                <td className="p-3">Direct deterministic tool call (MCP stdio & SSE compatible)</td>
                <td className="p-3 text-zinc-500">❌ Clean (Atomic File)</td>
                <td className="p-3 text-zinc-400">asl-vm-starlark / C-ABI</td>
              </tr>
              <tr>
                <td className="p-3 font-bold text-purple-400">.asl</td>
                <td className="p-3">Native Agent Skill Language full specification unit</td>
                <td className="p-3 text-zinc-500">❌ Clean (Atomic File)</td>
                <td className="p-3 text-zinc-400">asl-core / Wasm</td>
              </tr>
            </tbody>
          </table>
        </div>

        <h2 className="text-xl font-bold text-white tracking-tight pt-4">
          Shadow Markdown Projection: Architectural Rationale
        </h2>
        <p>
          One of the core design achievements in ADR-0009 and ADR-0011 is <strong>Selective Shadow Projection</strong>:
        </p>
        <ul className="list-disc pl-5 space-y-2 text-zinc-400">
          <li>
            <strong className="text-zinc-200">Why only .skill projects a shadow:</strong> Skills frequently interface with human developers in IDEs, GitHub web readers, and non-ASL LLM harness tools. The projection generates an isolated <code className="text-zinc-300 font-mono">&lt;file&gt;.skill.md</code> reflecting the human-readable Markdown view with cryptographic synchronization.
          </li>
          <li>
            <strong className="text-zinc-200">Why .tool and .asl remain clean:</strong> Direct tools and language units are machine-evaluated. Generating companion markdown files for every single tool would clutter developer workspaces, increase token overhead during directory scans, and violate Axiom 1 (Strict File Atomicity).
          </li>
        </ul>

        <h2 className="text-xl font-bold text-white tracking-tight pt-4">
          Interactive Triad Comparison
        </h2>
        <p className="text-xs text-zinc-400">
          Click between the tabs below to inspect how the same intent is structured across the three canonical formats:
        </p>

        <CodeSwitcher />

        <div className="pt-6 flex items-center justify-between border-t border-zinc-850">
          <Link
            href="/docs/tokenomics"
            className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200"
          >
            <span>← 93.2% Token Reduction</span>
          </Link>
          <Link
            href="/docs/rules"
            className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300"
          >
            <span>Declarative Rules (asl:rules)</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
        </div>
      </div>
    </div>
  );
}
