import React from "react";
import Link from "next/link";
import { ArrowRight, Terminal, Shield, Layers, Cpu, Code2, Check, Copy } from "lucide-react";
import { CodeSwitcher } from "@/components/CodeSwitcher";
import { PlaygroundSimulator } from "@/components/PlaygroundSimulator";
import { QuickstartExamples } from "@/components/QuickstartExamples";
import { InstallSnippet } from "@/components/InstallSnippet";

export default function HomePage() {
  return (
    <div className="relative">
      {/* Hero Section */}
      <section className="mx-auto max-w-5xl px-4 pt-16 pb-16 sm:px-6 sm:pt-24 lg:px-8 text-center">
        {/* Subtle version badge */}
        <div className="inline-flex items-center gap-2 rounded-full border border-zinc-800 bg-zinc-950 px-3 py-1 text-xs text-zinc-400 mb-6">
          <span className="font-mono text-zinc-200">ASL 3.0</span>
          <span className="text-zinc-600">•</span>
          <span>Open Language Specification</span>
        </div>

        {/* Clean Headline */}
        <h1 className="text-4xl sm:text-6xl font-bold tracking-tight text-white max-w-3xl mx-auto leading-[1.12]">
          The Hermetic Language for AI Agents.
        </h1>

        {/* Professional Subtitle */}
        <p className="mt-5 text-sm sm:text-base text-zinc-400 max-w-2xl mx-auto leading-relaxed">
          Agent Skill Language (ASL) is a semantic, deterministic programming language for autonomous AI agent skills.
          Combine natural language semantic prompts with sandboxed ASL deterministic logic, strict capability security, and zero ambient authority.
        </p>

        {/* Primary CTA Buttons */}
        <div className="mt-8 flex flex-wrap items-center justify-center gap-3">
          <Link
            href="/docs"
            className="flex items-center gap-2 rounded-md bg-white px-4 py-2.5 text-xs font-semibold text-black hover:bg-zinc-200 transition-colors"
          >
            <span>Get Started</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>

          <Link
            href="/docs/syntax"
            className="flex items-center gap-2 rounded-md border border-zinc-800 bg-zinc-900/60 px-4 py-2.5 text-xs font-medium text-zinc-300 hover:border-zinc-700 hover:text-white transition-colors"
          >
            <Code2 className="h-3.5 w-3.5" />
            <span>Syntax Reference</span>
          </Link>

          <Link
            href="/paper"
            className="flex items-center gap-2 rounded-md border border-zinc-800 bg-zinc-900/40 px-4 py-2.5 text-xs font-medium text-zinc-400 hover:border-zinc-700 hover:text-white transition-colors"
          >
            <Terminal className="h-3.5 w-3.5" />
            <span>Architecture Paper</span>
          </Link>
        </div>

        {/* Clean CLI Install Snippet */}
        <InstallSnippet />
      </section>

      {/* Hands-on Simple Examples: Skill, Tool, ASL */}
      <QuickstartExamples />

      {/* Code Architecture Triad Section */}
      <section className="mx-auto max-w-5xl px-4 py-12 sm:px-6 lg:px-8 border-t border-zinc-800">
        <div className="mb-8">
          <div className="text-xs font-mono font-semibold uppercase tracking-wider text-zinc-500">
            Atomic Architecture
          </div>
          <h2 className="text-2xl font-bold tracking-tight text-white mt-1">
            The Canonical Triad: One Specification, Three Formats
          </h2>
          <p className="text-xs sm:text-sm text-zinc-400 max-w-2xl mt-1.5 leading-relaxed">
            Every ASL unit encapsulates manifest metadata, model instructions, and deterministic execution bytecode in a single atomic file.
          </p>
        </div>

        <CodeSwitcher />
      </section>

      {/* Core Architectural Pillars */}
      <section className="mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8 border-t border-zinc-800">
        <div className="mb-10">
          <div className="text-xs font-mono font-semibold uppercase tracking-wider text-zinc-500">
            Engine Design
          </div>
          <h2 className="text-2xl font-bold tracking-tight text-white mt-1">
            Built for Robust Agent Workflows
          </h2>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-2.5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-2.5">
              <div className="p-2 rounded-lg bg-zinc-900 border border-zinc-800">
                <Code2 className="h-4 w-4 text-white" />
              </div>
              <h3 className="text-sm font-semibold text-white">Dual-Consumer AST</h3>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              Consumable simultaneously by neural language models as an invariant semantic directive and by a hermetic host runtime as a closed deterministic specification.
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-2.5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-2.5">
              <div className="p-2 rounded-lg bg-zinc-900 border border-zinc-800">
                <Shield className="h-4 w-4 text-white" />
              </div>
              <h3 className="text-sm font-semibold text-white">Capability Confinement (OCap)</h3>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              Strictly zero ambient authority. All filesystem, network, and environment interactions require explicitly passed capability handles bounded at compile time.
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-2.5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-2.5">
              <div className="p-2 rounded-lg bg-zinc-900 border border-zinc-800">
                <Cpu className="h-4 w-4 text-white" />
              </div>
              <h3 className="text-sm font-semibold text-white">Bounded Fuel Termination</h3>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              Every execution cycle is bounded by a monotonic fuel counter. Unbounded loops (<code className="text-zinc-300 font-mono">while</code>) and recursion are strictly disallowed.
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-2.5 hover:border-zinc-700 transition-colors">
            <div className="flex items-center gap-2.5">
              <div className="p-2 rounded-lg bg-zinc-900 border border-zinc-800">
                <Terminal className="h-4 w-4 text-white" />
              </div>
              <h3 className="text-sm font-semibold text-white">Zero Host Dependencies</h3>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              The core engine (<code className="text-zinc-300 font-mono">libasl</code> and <code className="text-zinc-300 font-mono">asl-cli</code>) is written in pure Rust. Runs without Python, pip, Node.js, or virtual environment breaks.
            </p>
          </div>
        </div>
      </section>

      {/* Interactive In-Browser Playground Section */}
      <section className="mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8 border-t border-zinc-800">
        <div className="mb-8">
          <div className="text-xs font-mono font-semibold uppercase tracking-wider text-zinc-500">
            Interactive Environment
          </div>
          <h2 className="text-2xl font-bold tracking-tight text-white mt-1">
            Test ASL in the Browser
          </h2>
          <p className="text-xs sm:text-sm text-zinc-400 max-w-2xl mt-1.5 leading-relaxed">
            Write declarative semantic rules in native ASL syntax, simulate AOT compilation (compatible with Starlark L1 bytecode), and test execution directly in WebAssembly.
          </p>
        </div>

        <PlaygroundSimulator />
      </section>

      {/* Quickstart Reference */}
      <section className="mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8 border-t border-zinc-800 mb-12">
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 sm:p-8">
          <h2 className="text-lg font-bold text-white tracking-tight">CLI Quickstart Reference</h2>
          <p className="text-xs text-zinc-400 mt-1">Common commands for developing and executing ASL skills.</p>

          <div className="mt-5 space-y-2.5 font-mono text-xs">
            <div className="rounded-lg border border-zinc-800/80 bg-black p-3 text-zinc-300 flex items-center justify-between">
              <code>asl run examples/git-conventional-commit.skill --input &#39;&#123;&quot;intent&quot;: &quot;fix parser bug&quot;&#125;&#39;</code>
              <span className="text-zinc-500 font-sans text-[11px] hidden sm:inline">Execute</span>
            </div>
            <div className="rounded-lg border border-zinc-800/80 bg-black p-3 text-zinc-300 flex items-center justify-between">
              <code>asl check examples/conventional-commit-rules.skill</code>
              <span className="text-zinc-500 font-sans text-[11px] hidden sm:inline">Validate</span>
            </div>
            <div className="rounded-lg border border-zinc-800/80 bg-black p-3 text-zinc-300 flex items-center justify-between">
              <code>asl serve --mcp</code>
              <span className="text-zinc-500 font-sans text-[11px] hidden sm:inline">Start MCP</span>
            </div>
          </div>

          <div className="mt-6 pt-5 border-t border-zinc-800/80 flex items-center justify-between">
            <span className="text-xs text-zinc-400">Ready to build deterministic skills?</span>
            <Link
              href="/docs"
              className="flex items-center gap-1.5 text-xs font-semibold text-white hover:text-zinc-300 transition-colors"
            >
              <span>Explore Documentation</span>
              <ArrowRight className="h-3.5 w-3.5" />
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
