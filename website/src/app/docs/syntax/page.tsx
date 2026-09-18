import React from "react";
import Link from "next/link";
import { ArrowRight, Code, Shield, Layers, Terminal, Binary, Workflow, Sparkles, BookOpen } from "lucide-react";

export default function SyntaxReferencePage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-zinc-500 uppercase tracking-wider">
          Language Specification
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Syntax &amp; File Anatomy
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Formal structure, dual-consumer semantic model, and complete manifest schema for Agent Skill Language (ASL 3.0). Every ASL unit unites machine metadata, neural instructions, and deterministic logic in a single atomic file.
        </p>
      </div>

      {/* 1. Tri-Partite File Anatomy */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. The Tri-Partite File Anatomy</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          An ASL file is divided into three distinct syntactic regions delimited by standard YAML document markers (<code className="text-zinc-200 font-mono">---</code>):
        </p>

        <div className="rounded-lg border border-zinc-800 bg-[#0d0d0d] p-4 font-mono text-xs text-zinc-300 overflow-x-auto">
          <pre>{`---
# REGION 1: Frontmatter Manifest (Strict YAML)
asl_version: "3.0"
name: "git-commit-helper"
version: "1.0.0"
description: "Generates semantic Conventional Commits from git status"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "execute"
  input_schema:
    type: "object"
    required: ["intent"]
    properties:
      intent: { type: "string" }
capabilities:
  fs:
    confined_read_roots: ["./"]
limits:
  max_fuel_opcodes: 100000
---

# REGION 2: Semantic Prompt Envelope (CommonMark Markdown)
## 1. Intent
Generates conventional commit messages based on user intent and workspace diffs.

## 2. Decision Protocol
Invoke this skill when the user asks to summarize staged changes or prepare a commit.

---

\`\`\`asl:deterministic
# REGION 3: ASL Deterministic Block (asl:deterministic or asl:rules)
def execute(ctx, input):
    intent = input.get("intent", "").strip()
    if not intent:
        return {"valid": False, "error": "Intent is mandatory."}
    return {
        "valid": True,
        "formatted": "chore: " + intent,
        "fuel_used": ctx.fuel.consumed()
    }
\`\`\``}</pre>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-3 pt-2 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="font-semibold text-white font-mono">1. Frontmatter Manifest</span>
            <p className="text-zinc-400">Strict YAML parsed into <code className="text-zinc-300 font-mono">SkillManifest</code>. Validates JSON schemas, capabilities, and fuel limits ahead of execution.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="font-semibold text-white font-mono">2. Semantic Envelope</span>
            <p className="text-zinc-400">Pure CommonMark consumed by the LLM reasoning loop. Kept byte-for-byte immutable across agent turns for 100% KV-cache invariance.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="font-semibold text-white font-mono">3. Deterministic Block</span>
            <p className="text-zinc-400">Native ASL procedural code (<code className="text-zinc-300 font-mono">asl:deterministic</code>) or declarative rules (<code className="text-zinc-300 font-mono">asl:rules</code>) executed in the ASL sandboxed VM (compatible with Starlark L1).</p>
          </div>
        </div>
      </section>

      {/* 2. Dual-Consumer Execution Paradigm */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">2. The Dual-Consumer Execution Model</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          Why is ASL called a <em>Semantic Language</em>? Unlike traditional languages (C, Python, Rust) that target only a CPU interpreter or compiler, an ASL program serves <strong>two simultaneous consumers</strong>:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
          <div className="rounded-xl border border-blue-900/50 bg-blue-950/20 p-5 space-y-2">
            <div className="flex items-center gap-2 text-blue-400 font-bold font-mono">
              <Sparkles className="h-4 w-4" />
              <span>Consumer A: The Neural Reasoner (LLM)</span>
            </div>
            <p className="text-zinc-300 leading-relaxed">
              The LLM consumes Region 2 (CommonMark instructions). Because the prompt envelope contains natural language guidelines, decision protocols, and intent boundaries, the neural model understands <em>when</em> and <em>why</em> to invoke the skill, producing structured JSON parameters matching <code className="text-zinc-200 font-mono">input_schema</code>.
            </p>
          </div>

          <div className="rounded-xl border border-emerald-900/50 bg-emerald-950/20 p-5 space-y-2">
            <div className="flex items-center gap-2 text-emerald-400 font-bold font-mono">
              <Shield className="h-4 w-4" />
              <span>Consumer B: The Sandboxed ASL Host VM</span>
            </div>
            <p className="text-zinc-300 leading-relaxed">
              When the agent calls the skill, the host ASL engine validates arguments, creates an isolated memory sandbox, injects capability context handles (<code className="text-zinc-200 font-mono">ctx</code>), and executes Region 3 with guaranteed mathematical termination in finite fuel steps (100% compatible with the Starlark L1 runtime standard).
            </p>
          </div>
        </div>
      </section>

      {/* 3. Frontmatter Specification */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">3. Frontmatter Manifest Specification</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The frontmatter declares identity, contracts, I/O schemas, and capability limits:
        </p>

        <div className="overflow-x-auto rounded-lg border border-zinc-800">
          <table className="w-full text-left text-xs">
            <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
              <tr>
                <th className="p-3">Field</th>
                <th className="p-3">Type</th>
                <th className="p-3">Required</th>
                <th className="p-3">Description</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
              <tr>
                <td className="p-3 text-white">asl_version</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Must match <code className="text-zinc-200">&quot;3.0&quot;</code> or start with <code className="text-zinc-200">&quot;3.&quot;</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">name</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Unique identifier slug, e.g. <code className="text-zinc-200">&quot;git-commit-helper&quot;</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">version</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">SemVer string, e.g. <code className="text-zinc-200">&quot;1.2.0&quot;</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">description</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Natural language summary for discovery and indexing.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">interface.protocol</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Default: <code className="text-zinc-200">&quot;mcp-tool-v1&quot;</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">interface.entrypoint</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Function name in deterministic code invoked by host runtime.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">interface.input_schema</td>
                <td className="p-3 text-blue-400">object (JSON Schema)</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Strict JSON Schema for arguments passed to entrypoint.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">interface.output_schema</td>
                <td className="p-3 text-blue-400">object (JSON Schema)</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">JSON Schema validating return data structure.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">capabilities.fs.confined_read_roots</td>
                <td className="p-3 text-blue-400">string[]</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Whitelisted root directories for sandboxed read access.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">limits.max_fuel_opcodes</td>
                <td className="p-3 text-blue-400">integer</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Monotonic instruction limit (default: <code className="text-zinc-200">1,000,000</code>).</td>
              </tr>
              <tr>
                <td className="p-3 text-white">limits.max_heap_kib</td>
                <td className="p-3 text-blue-400">integer</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Memory allocation ceiling (default: <code className="text-zinc-200">8,192</code> = 8MB).</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      {/* 4. Complete Language Reference Directory */}
      <section className="space-y-4 border-t border-zinc-800 pt-8 pb-12">
        <h2 className="text-xl font-bold text-white tracking-tight">4. Complete Language Reference Manual</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          Deep-dive into the formal language specifications across dedicated reference chapters:
        </p>

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
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/cli"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← CLI &amp; Tooling Reference
        </Link>
        <Link
          href="/docs/types"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>Types &amp; Data Model</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
