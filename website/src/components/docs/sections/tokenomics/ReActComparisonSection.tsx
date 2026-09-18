import React from "react";
import { Sparkles } from "lucide-react";
import { MathBlock } from "@/components/MathBlock";

export const ReActComparisonSection: React.FC = () => {
  return (
    <div className="space-y-6">
      {/* Abstract callout */}
      <div className="rounded-xl border border-blue-500/20 bg-blue-500/5 p-4 text-xs text-zinc-300">
        <div className="font-semibold text-blue-400 flex items-center gap-1.5 mb-1">
          <Sparkles className="h-4 w-4" />
          Empirical Benchmark Principle
        </div>
        The 93.2% token savings metric is not a theoretical conjecture. It is measured by comparing an identical automation task executed by an LLM agent under two paradigms: <strong>Paradigm A (Legacy Uncompiled ReAct Scripts)</strong> vs <strong>Paradigm B (ASL 3.0 Atomic AOT Tool Invocation)</strong>.
      </div>

      <h2 className="text-xl font-bold text-white tracking-tight">
        1. Deconstructing the Legacy ReAct Multi-Turn Cycle
      </h2>
      <p className="text-sm text-zinc-300 leading-relaxed">
        In standard agent platforms (Claude Code, Google Antigravity, AutoGen), executing a procedural task requires an exploratory sequence of turns across the inference window:
      </p>

      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-4 font-mono text-xs">
        <div className="border-b border-zinc-850 pb-3">
          <div className="text-red-400 font-semibold">// Turn 1: Context & Discovery (~770 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Raw Markdown instruction injection (<code className="text-zinc-200">~650 tokens</code>) + LLM explores filesystem using <code className="text-zinc-200">cat script.py</code> (<code className="text-zinc-200">~120 tokens</code>).
          </div>
        </div>
        <div className="border-b border-zinc-850 pb-3">
          <div className="text-red-400 font-semibold">// Turn 2: Shell Invocation & Stderr Output (~480 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Subshell process launch, verbose stdout/stderr logs, environment variable discrepancies, or missing library warnings.
          </div>
        </div>
        <div className="border-b border-zinc-850 pb-3">
          <div className="text-red-400 font-semibold">// Turn 3: Stochastic Error Repair & Retry (~350 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Model attempts self-correction, adjusts flags, or modifies shell invocation arguments.
          </div>
        </div>
        <div>
          <div className="text-red-400 font-semibold">// Turn 4: Parsing & Formatting (~500 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Manual regex parsing of non-deterministic CLI output and synthesizing final assistant response.
          </div>
        </div>
      </div>

      <div className="py-2 text-center">
        <MathBlock
          block
          math={`\\mathcal{T}_{\\text{Legacy}} = 650 + 120 + 480 + 350 + 500 = 2{,}100\\text{ tokens}`}
        />
      </div>

      <h2 className="text-xl font-bold text-white tracking-tight pt-4">
        2. The ASL 3.0 Atomic In-Process Invocation
      </h2>
      <p className="text-sm text-zinc-300 leading-relaxed">
        In ASL 3.0, the unit is validated and compiled Ahead-of-Time (AOT). The LLM never reads the raw script or helper files; it only receives the compiled JSON Schema interface:
      </p>

      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-4 font-mono text-xs">
        <div className="border-b border-zinc-850 pb-3">
          <div className="text-emerald-400 font-semibold">// Step 1: Compact Tool Schema (~85 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Strictly typed tool definition generated directly from the YAML frontmatter interface.
          </div>
        </div>
        <div className="border-b border-zinc-850 pb-3">
          <div className="text-emerald-400 font-semibold">// Step 2: Atomic Typed Invocation (~32 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Single-turn structured function call strictly constrained by GBNF grammar masks (0% syntax errors).
          </div>
        </div>
        <div>
          <div className="text-emerald-400 font-semibold">// Step 3: Compact Rust Return Payload (~25 tokens)</div>
          <div className="text-zinc-400 mt-1">
            Zero noise, typed JSON dictionary returned directly by the ASL runtime.
          </div>
        </div>
      </div>

      <div className="py-2 text-center">
        <MathBlock
          block
          math={`\\mathcal{T}_{\\text{ASL}} = 85 + 32 + 25 = 142\\text{ tokens}`}
        />
      </div>
    </div>
  );
};
