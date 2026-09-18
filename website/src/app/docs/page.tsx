import React from "react";
import Link from "next/link";
import { ArrowRight, Sparkles, Shield, Cpu, Layers, CheckCircle2 } from "lucide-react";
import { MathBlock } from "@/components/MathBlock";

export default function DocsOverviewPage() {
  return (
    <div className="space-y-8">
      <div>
        <div className="text-xs font-mono font-medium text-blue-400 uppercase tracking-wider">
          Documentation • Track 1
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Agent Skill Language Overview
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed">
          An AI-First, hermetic programming language and deterministic runtime designed specifically for autonomous agent orchestration.
        </p>
      </div>

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <h2 className="text-xl font-bold text-white tracking-tight">
          The Problem: The Fragility Triad of Legacy Agents
        </h2>
        <p>
          Current AI agent frameworks (such as raw Claude Code tools, OpenAI Operator actions, or general-purpose MCP servers) execute unconfined scripts in Python, Bash, or Node.js. In production, this architecture suffers from three catastrophic failure modes:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 my-6">
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
            <div className="text-xs font-bold text-red-400 font-mono">1. Ambient Authority</div>
            <p className="text-xs text-zinc-400">
              Unconfined scripts execute with full OS root/user privileges, allowing indirect prompt injections to execute remote code (RCE) or exfiltrate environment secrets.
            </p>
          </div>
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
            <div className="text-xs font-bold text-amber-400 font-mono">2. Environment Drift</div>
            <p className="text-xs text-zinc-400">
              Scripts depend on global interpreters, virtual environments, and package managers. Breakages force LLMs into 10+ turn repair loops wasting thousands of tokens.
            </p>
          </div>
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
            <div className="text-xs font-bold text-blue-400 font-mono">3. Token Bloat & Cache Loss</div>
            <p className="text-xs text-zinc-400">
              Exploratory terminal loops (<code className="text-zinc-200">cat</code>, <code className="text-zinc-200">ls</code>, stderr tracebacks) consume ~2,100 tokens per action and destroy inference engine KV-cache prefixes.
            </p>
          </div>
        </div>

        <h2 className="text-xl font-bold text-white tracking-tight pt-4">
          The ASL Solution: Dual-Consumer AST
        </h2>
        <p>
          Literate Programming (Knuth, 1984) united human prose with compiler code. ASL reinvents this for the AI era: an atomic skill unit is simultaneously consumed by two distinct entities:
        </p>

        <div className="rounded-xl border border-zinc-800 bg-black p-4 font-mono text-xs my-4 space-y-2">
          <div className="text-zinc-400">// Mathematical Definition of an ASL Unit:</div>
          <MathBlock block math={`\\mathcal{S} = \\langle \\mathcal{M}, \\mathcal{P}, \\mathcal{D} \\rangle`} />
          <ul className="list-disc pl-5 space-y-1 text-zinc-400">
            <li><span className="text-zinc-200 font-semibold">\mathcal&#123;M&#125; (Manifest)</span>: Immutable YAML frontmatter with capabilities, OCap bounds, and entrypoint signatures.</li>
            <li><span className="text-zinc-200 font-semibold">\mathcal&#123;P&#125; (Semantic Envelope)</span>: CommonMark prose parsed by the LLM (Consumer \alpha) for intent and context.</li>
            <li><span className="text-zinc-200 font-semibold">\mathcal&#123;D&#125; (Deterministic Block)</span>: Native ASL deterministic logic (evaluated by Consumer \beta, 100% compatible with Starlark L1 runtime).</li>
          </ul>
        </div>

        <h2 className="text-xl font-bold text-white tracking-tight pt-4">
          Core Tenets & Guarantees
        </h2>
        <div className="space-y-3">
          <div className="flex items-start gap-3">
            <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <strong className="text-white">Atomic Single-File Distribution:</strong> Each skill lives in a single atomic file (<code className="text-zinc-200 font-mono">.skill</code>, <code className="text-zinc-200 font-mono">.tool</code>, <code className="text-zinc-200 font-mono">.asl</code>). No subdirectories, loose helper scripts, or companion virtual environments.
            </div>
          </div>
          <div className="flex items-start gap-3">
            <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <strong className="text-white">Pure Rust Runtime:</strong> Zero external interpreter requirements (no Python, Node.js, or JVM needed on the host system).
            </div>
          </div>
          <div className="flex items-start gap-3">
            <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <strong className="text-white">Guaranteed Termination via Fuel:</strong> Monotonically decreasing opcode counters prevent infinite loops or Denial-of-Service stalls.
            </div>
          </div>
          <div className="flex items-start gap-3">
            <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
            <div>
              <strong className="text-white">93.2% Token Reduction:</strong> AOT schema validation eliminates ReAct exploratory turns and stochastic JSON repair retries.
            </div>
          </div>
        </div>

        <div className="pt-6 flex items-center justify-between border-t border-zinc-800">
          <span className="text-xs text-zinc-500">Next Track</span>
          <Link
            href="/docs/syntax"
            className="flex items-center gap-1.5 text-xs font-semibold text-white hover:text-zinc-300 transition-colors"
          >
            <span>Complete Syntax Reference</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
        </div>
      </div>
    </div>
  );
}
