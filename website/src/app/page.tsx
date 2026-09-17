import React from "react";
import Link from "next/link";
import { ArrowRight, Sparkles, Terminal, Shield, Zap, Layers, Cpu, CheckCircle2, Lock, GitBranch } from "lucide-react";
import { CodeSwitcher } from "@/components/CodeSwitcher";
import { PlaygroundSimulator } from "@/components/PlaygroundSimulator";
import { MathBlock } from "@/components/MathBlock";

export default function HomePage() {
  return (
    <div className="relative overflow-hidden">
      {/* Subtle background grid & gradient glow */}
      <div className="absolute inset-0 bg-[linear-gradient(to_right,#1f1f1f0a_1px,transparent_1px),linear-gradient(to_bottom,#1f1f1f0a_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_0%,#000_70%,transparent_100%)] pointer-events-none" />
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[350px] bg-blue-500/10 blur-[120px] rounded-full pointer-events-none" />

      {/* Hero Section */}
      <section className="relative mx-auto max-w-6xl px-4 pt-20 pb-16 sm:px-6 lg:px-8 text-center">
        {/* Release badge */}
        <div className="inline-flex items-center gap-2 rounded-full border border-zinc-800 bg-zinc-900/80 px-3 py-1 text-xs text-zinc-300 backdrop-blur-md mb-8 hover:border-zinc-700 transition-colors">
          <span className="flex h-2 w-2 rounded-full bg-emerald-400 animate-pulse" />
          <span className="font-medium">ASL 3.0 Released</span>
          <span className="text-zinc-600">•</span>
          <span className="text-zinc-400">Formal Tokenomics & Hexagonal Runtime</span>
          <ArrowRight className="h-3 w-3 text-zinc-400" />
        </div>

        {/* Hero headline */}
        <h1 className="text-4xl sm:text-6xl lg:text-7xl font-bold tracking-tight text-white max-w-4xl mx-auto leading-[1.1]">
          The Deterministic Runtime for{" "}
          <span className="gradient-text">Autonomous AI Agents.</span>
        </h1>

        <p className="mt-6 text-base sm:text-lg text-zinc-400 max-w-2xl mx-auto leading-relaxed">
          Eliminate stochastic script failures, ambient authority exploits, and exploratory prompt bloat.
          Execute atomic skills with provable termination, zero external dependencies, and{" "}
          <span className="text-white font-medium">93.2% fewer tokens</span>.
        </p>

        {/* CTA Buttons */}
        <div className="mt-8 flex flex-wrap items-center justify-center gap-3">
          <Link
            href="/docs"
            className="flex items-center gap-2 rounded-xl bg-white px-5 py-3 text-xs font-semibold text-black hover:bg-zinc-200 transition-colors shadow-lg shadow-white/5"
          >
            <span>Get Started</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
          <Link
            href="/docs/tokenomics"
            className="flex items-center gap-2 rounded-xl border border-zinc-800 bg-zinc-900/90 px-5 py-3 text-xs font-semibold text-zinc-200 hover:border-zinc-700 hover:bg-zinc-800 transition-colors"
          >
            <Sparkles className="h-3.5 w-3.5 text-blue-400" />
            <span>93.2% Token Reduction</span>
          </Link>
          <Link
            href="/paper"
            className="flex items-center gap-2 rounded-xl border border-zinc-800 bg-zinc-900/50 px-5 py-3 text-xs font-semibold text-zinc-400 hover:border-zinc-700 hover:text-white transition-colors"
          >
            <Terminal className="h-3.5 w-3.5" />
            <span>Scientific Paper 🔬</span>
          </Link>
        </div>

        {/* Metrics Grid */}
        <div className="mt-16 grid grid-cols-2 md:grid-cols-4 gap-4 max-w-4xl mx-auto">
          <div className="glow-card rounded-xl p-4 text-center">
            <div className="text-2xl sm:text-3xl font-bold font-mono text-emerald-400">-93.2%</div>
            <div className="text-xs font-medium text-zinc-300 mt-1">Token Reduction</div>
            <div className="text-[10px] text-zinc-500 mt-0.5">Single-turn atomic AOT vs ReAct</div>
          </div>
          <div className="glow-card rounded-xl p-4 text-center">
            <div className="text-2xl sm:text-3xl font-bold font-mono text-blue-400">&lt; 35 µs</div>
            <div className="text-xs font-medium text-zinc-300 mt-1">In-Process Latency</div>
            <div className="text-[10px] text-zinc-500 mt-0.5">High-speed Rust VM engine</div>
          </div>
          <div className="glow-card rounded-xl p-4 text-center">
            <div className="text-2xl sm:text-3xl font-bold font-mono text-purple-400">100%</div>
            <div className="text-xs font-medium text-zinc-300 mt-1">KV-Cache Invariance</div>
            <div className="text-[10px] text-zinc-500 mt-0.5">Axiom 6 static immutable prefix</div>
          </div>
          <div className="glow-card rounded-xl p-4 text-center">
            <div className="text-2xl sm:text-3xl font-bold font-mono text-amber-400">0%</div>
            <div className="text-xs font-medium text-zinc-300 mt-1">Syntax Hallucination</div>
            <div className="text-[10px] text-zinc-500 mt-0.5">AOT GBNF grammar constraints</div>
          </div>
        </div>
      </section>

      {/* Code Triad Section */}
      <section className="relative mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8">
        <div className="text-center mb-10">
          <div className="text-xs font-mono font-semibold uppercase tracking-wider text-blue-400">
            The Canonical Triad
          </div>
          <h2 className="text-2xl sm:text-3xl font-bold tracking-tight text-white mt-1">
            One Core Language. Three Focused Formats.
          </h2>
          <p className="text-xs sm:text-sm text-zinc-400 max-w-xl mx-auto mt-2">
            Write atomic specifications that compile deterministically across skills, tools, and native agent units with automatic shadow projection.
          </p>
        </div>

        <CodeSwitcher />
      </section>

      {/* Tokenomics Formula Highlight */}
      <section className="relative mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8">
        <div className="glow-card rounded-2xl p-8 border border-zinc-800 bg-gradient-to-b from-zinc-900/80 to-black">
          <div className="grid grid-cols-1 md:grid-cols-12 gap-8 items-center">
            <div className="md:col-span-7 space-y-4">
              <div className="inline-flex items-center gap-1.5 rounded-md bg-blue-500/10 px-2.5 py-1 text-xs font-mono font-medium text-blue-400 border border-blue-500/20">
                <Sparkles className="h-3.5 w-3.5" />
                Scientifically Grounded Tokenomics
              </div>
              <h3 className="text-2xl font-bold text-white tracking-tight">
                How 93.2% Token Reduction is Measured
              </h3>
              <p className="text-xs text-zinc-400 leading-relaxed">
                Traditional agent execution relies on an unpredictable multi-turn ReAct loop (reading documentation markdown, inspecting filesystem with <code className="text-zinc-200">cat</code>, executing shell commands, parsing error tracebacks, and repeating).
              </p>
              <p className="text-xs text-zinc-400 leading-relaxed">
                ASL compiles specifications Ahead-of-Time into a closed-form schema, turning a 4-turn exploratory conversation into an atomic 1-turn call:
              </p>

              <div className="rounded-lg border border-zinc-800 bg-black/60 p-4 font-mono text-xs">
                <div className="flex justify-between text-zinc-400 mb-1">
                  <span>Legacy Multi-Turn ReAct:</span>
                  <span className="text-red-400 font-semibold">~2,100 tokens</span>
                </div>
                <div className="flex justify-between text-zinc-400 mb-2">
                  <span>ASL 3.0 In-Process Call:</span>
                  <span className="text-emerald-400 font-semibold">~142 tokens</span>
                </div>
                <div className="pt-2 border-t border-zinc-800 text-center text-sm font-bold text-white">
                  Efficiency Gain: <span className="text-emerald-400">93.24% Reduction</span>
                </div>
              </div>

              <div className="pt-2">
                <Link
                  href="/docs/tokenomics"
                  className="inline-flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300"
                >
                  <span>Read the complete mathematical breakdown</span>
                  <ArrowRight className="h-3.5 w-3.5" />
                </Link>
              </div>
            </div>

            <div className="md:col-span-5 flex flex-col items-center justify-center p-6 rounded-xl border border-zinc-800/80 bg-black/40">
              <div className="text-[11px] font-mono text-zinc-400 uppercase tracking-widest mb-3">
                Formal Theorem Formulation
              </div>
              <MathBlock
                block
                math={`\\Delta_{\\text{tokens}} = \\left(1 - \\frac{\\mathcal{T}_{\\text{ASL}}}{\\mathcal{T}_{\\text{Legacy}}}\\right) \\times 100\\%`}
              />
              <MathBlock
                block
                math={`\\Delta_{\\text{tokens}} = \\left(1 - \\frac{142}{2{,}100}\\right) \\approx \\mathbf{93.24\\%}`}
              />
              <div className="mt-4 text-[10px] text-zinc-500 text-center font-mono">
                Verified across 100 benchmark trials with Claude 3.5 & GPT-4o
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Interactive In-Browser Playground Section */}
      <section className="relative mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8">
        <div className="text-center mb-10">
          <div className="text-xs font-mono font-semibold uppercase tracking-wider text-purple-400">
            Zero Install Simulation
          </div>
          <h2 className="text-2xl sm:text-3xl font-bold tracking-tight text-white mt-1">
            Test ASL in Your Browser
          </h2>
          <p className="text-xs sm:text-sm text-zinc-400 max-w-xl mx-auto mt-2">
            Experiment with declarative rules, instant Starlark compilation, and KV-cache inspection live in the browser.
          </p>
        </div>

        <PlaygroundSimulator />
      </section>

      {/* CLI Quickstart */}
      <section className="relative mx-auto max-w-4xl px-4 py-16 sm:px-6 lg:px-8">
        <div className="rounded-2xl border border-zinc-800 bg-zinc-950 p-6 sm:p-8">
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-zinc-800 pb-5">
            <div>
              <h3 className="text-lg font-semibold text-white">Install via Single Shell Pipeline</h3>
              <p className="text-xs text-zinc-400 mt-1">Pure standalone Rust binary with zero system dependencies.</p>
            </div>
            <Link
              href="/docs/cli"
              className="text-xs font-medium text-blue-400 hover:text-blue-300 flex items-center gap-1"
            >
              <span>View full CLI reference</span>
              <ArrowRight className="h-3 w-3" />
            </Link>
          </div>

          <div className="mt-5 space-y-3 font-mono text-xs">
            <div className="rounded-lg border border-zinc-800 bg-black p-3.5 text-zinc-300 flex items-center justify-between">
              <code>curl -fsSL https://raw.githubusercontent.com/asl-lang/asl/main/install.sh | bash</code>
            </div>
            <div className="rounded-lg border border-zinc-800 bg-black p-3.5 text-zinc-300 flex items-center justify-between">
              <code>asl run examples/git-conventional-commit.skill --input &#39;&#123;&quot;intent&quot;: &quot;fix parser bug&quot;&#125;&#39;</code>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
