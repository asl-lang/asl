import React from "react";
import Link from "next/link";
import { ArrowRight, Code, Shield, Layers, Terminal, Check } from "lucide-react";

export default function SyntaxReferencePage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-zinc-500 uppercase tracking-wider">
          Language Specification
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Complete Syntax Reference
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Formal grammar, structure, and type specifications for Agent Skill Language (ASL 3.0).
          Every ASL document is an atomic file containing structured metadata, a semantic prompt envelope, and a deterministic execution block.
        </p>
      </div>

      {/* Anatomy Overview */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. File Structure &amp; Dual-Consumer AST</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          An ASL file is divided into three distinct syntactic regions delimited by standard YAML markers (<code className="text-zinc-200">---</code>):
        </p>

        <div className="rounded-lg border border-zinc-800 bg-[#0d0d0d] p-4 font-mono text-xs text-zinc-300 overflow-x-auto">
          <pre>{`---
# REGION 1: Frontmatter Manifest (YAML)
asl_version: "3.0"
name: "example-skill"
version: "1.0.0"
description: "Description of what this skill accomplishes"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "execute"
  input_schema: { ... }
  output_schema: { ... }
capabilities: { ... }
limits: { ... }
---

# REGION 2: Semantic Prompt Envelope (CommonMark Markdown)
## 1. Intent
Instructions consumed directly by the LLM reasoning loop.

## 2. Activation Criteria
When the model should invoke this unit.

---

\`\`\`asl:deterministic
# REGION 3: Deterministic Logic Block (Starlark L1 or asl:rules)
def execute(ctx, input):
    return {"status": "ok", "result": input.get("query")}
\`\`\``}</pre>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-3 pt-2 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="font-semibold text-white font-mono">1. Frontmatter</span>
            <p className="text-zinc-400">Strict YAML parsed into <code className="text-zinc-300">SkillManifest</code>. Validates schemas, capabilities, and execution limits before any bytecode runs.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="font-semibold text-white font-mono">2. Semantic Envelope</span>
            <p className="text-zinc-400">Pure CommonMark consumed by neural models. Kept byte-for-byte immutable across turns for 100% KV-cache invariance.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="font-semibold text-white font-mono">3. Deterministic Block</span>
            <p className="text-zinc-400">Hermetic Starlark L1 code or declarative <code className="text-zinc-300">asl:rules</code> evaluated in a sandboxed runtime with fuel limits.</p>
          </div>
        </div>
      </section>

      {/* Frontmatter Specification */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">2. Frontmatter Manifest Specification</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The frontmatter defines the metadata, capability grants, I/O schemas, and runtime resource quotas.
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
                <td className="p-3 font-sans">Must match <code className="text-zinc-200">"3.0"</code> or start with <code className="text-zinc-200">"3."</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">name</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Unique identifier (slug), e.g. <code className="text-zinc-200">"git-commit-helper"</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">version</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">SemVer release string, e.g. <code className="text-zinc-200">"1.2.0"</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">description</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">One-line natural language summary for discovery.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">license</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">SPDX license identifier, e.g. <code className="text-zinc-200">"MIT"</code> or <code className="text-zinc-200">"Apache-2.0"</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">interface.protocol</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Default: <code className="text-zinc-200">"mcp-tool-v1"</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">interface.entrypoint</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 text-emerald-400">Yes</td>
                <td className="p-3 font-sans">Function name in deterministic code to call upon invocation.</td>
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
                <td className="p-3 font-sans">JSON Schema validating the returned data structure.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">capabilities.fs.confined_read_roots</td>
                <td className="p-3 text-blue-400">string[]</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Whitelisted directory paths for read-only sandboxed access.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">capabilities.fs.allow_write</td>
                <td className="p-3 text-blue-400">string[]</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Whitelisted directory paths for sandboxed writing.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">capabilities.net.allow_domains</td>
                <td className="p-3 text-blue-400">string[]</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Whitelisted fully qualified domains for HTTP requests.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">limits.max_fuel_opcodes</td>
                <td className="p-3 text-blue-400">integer</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Instruction limit (default: <code className="text-zinc-200">1,000,000</code>). Prevents infinite execution.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">limits.max_heap_kib</td>
                <td className="p-3 text-blue-400">integer</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Maximum memory allocation in KiB (default: <code className="text-zinc-200">8,192</code> = 8MB).</td>
              </tr>
              <tr>
                <td className="p-3 text-white">limits.wall_clock_timeout_ms</td>
                <td className="p-3 text-blue-400">integer</td>
                <td className="p-3 text-zinc-500">No</td>
                <td className="p-3 font-sans">Wall-clock deadline in milliseconds (default: <code className="text-zinc-200">1,000</code>).</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      {/* Deterministic Starlark L1 */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold text-white tracking-tight">3. Deterministic Logic Syntax (<code className="text-blue-400">```asl:deterministic</code>)</h2>
          <span className="text-[11px] font-mono rounded bg-zinc-800 px-2 py-0.5 text-zinc-300">Starlark L1 Dialect</span>
        </div>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The deterministic block is written in a strict, hermetic dialect of Starlark (Google&apos;s deterministic Python dialect).
          It has <strong>zero ambient authority</strong>: it cannot import <code className="text-zinc-200">os</code>, <code className="text-zinc-200">sys</code>, or spawn subprocesses.
        </p>

        {/* Function signature */}
        <div className="space-y-2">
          <h3 className="text-sm font-semibold text-white font-mono">3.1 Entrypoint Signature</h3>
          <div className="rounded-lg border border-zinc-800 bg-[#0d0d0d] p-4 font-mono text-xs text-zinc-300">
            <pre>{`def entrypoint_name(ctx, input):
    # ctx   : Host capability interface (Context Object)
    # input : Validated dictionary conforming to input_schema
    return { ... } # Return value validating against output_schema`}</pre>
          </div>
        </div>

        {/* Context methods */}
        <div className="space-y-2 pt-2">
          <h3 className="text-sm font-semibold text-white font-mono">3.2 The Host Context Object (<code className="text-blue-400">ctx</code>)</h3>
          <p className="text-xs text-zinc-400">
            I/O operations are available only through explicitly granted capability handles on the <code className="text-zinc-200">ctx</code> parameter:
          </p>

          <div className="overflow-x-auto rounded-lg border border-zinc-800">
            <table className="w-full text-left text-xs">
              <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
                <tr>
                  <th className="p-3">Method</th>
                  <th className="p-3">Required Capability</th>
                  <th className="p-3">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
                <tr>
                  <td className="p-3 text-white">ctx.fs.read(path)</td>
                  <td className="p-3 text-amber-400">capabilities.fs.confined_read_roots</td>
                  <td className="p-3 font-sans">Reads file content as a string. Path must reside within whitelisted roots.</td>
                </tr>
                <tr>
                  <td className="p-3 text-white">ctx.fs.exists(path)</td>
                  <td className="p-3 text-amber-400">capabilities.fs.confined_read_roots</td>
                  <td className="p-3 font-sans">Returns boolean indicating whether file exists.</td>
                </tr>
                <tr>
                  <td className="p-3 text-white">ctx.crypto.sha256(data)</td>
                  <td className="p-3 text-emerald-400">None (Pure)</td>
                  <td className="p-3 font-sans">Computes SHA-256 hexadecimal digest of input string or bytes.</td>
                </tr>
                <tr>
                  <td className="p-3 text-white">ctx.crypto.verify_ed25519(msg, sig, pubkey)</td>
                  <td className="p-3 text-emerald-400">None (Pure)</td>
                  <td className="p-3 font-sans">Verifies cryptographic Ed25519 signature.</td>
                </tr>
                <tr>
                  <td className="p-3 text-white">ctx.env.get(var_name)</td>
                  <td className="p-3 text-amber-400">Explicit env grant</td>
                  <td className="p-3 font-sans">Reads a whitelisted environment variable.</td>
                </tr>
                <tr>
                  <td className="p-3 text-white">ctx.fail(reason_string)</td>
                  <td className="p-3 text-emerald-400">None</td>
                  <td className="p-3 font-sans">Terminates execution immediately with an error diagnostic.</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        {/* Language constraints */}
        <div className="space-y-2 pt-2">
          <h3 className="text-sm font-semibold text-white font-mono">3.3 Invariant Termination Constraints</h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
            <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
              <span className="font-semibold text-emerald-400">Allowed Syntax</span>
              <ul className="space-y-1 text-zinc-400 list-disc list-inside">
                <li><code className="text-zinc-300">for item in collection:</code> (bounded loop)</li>
                <li><code className="text-zinc-300">if / elif / else</code> branching</li>
                <li>List comprehensions: <code className="text-zinc-300">[x * 2 for x in items]</code></li>
                <li>Dictionaries, lists, tuples, strings, ints, booleans</li>
                <li>Builtins: <code className="text-zinc-300">len, range, str, int, min, max, sorted</code></li>
              </ul>
            </div>
            <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
              <span className="font-semibold text-rose-400">Strictly Forbidden</span>
              <ul className="space-y-1 text-zinc-400 list-disc list-inside">
                <li><code className="text-rose-300">while True:</code> (unbounded loops rejected at parse time)</li>
                <li>Recursion (functions calling themselves cause compile error)</li>
                <li><code className="text-rose-300">import os, sys, subprocess</code> (no ambient authority)</li>
                <li>Global mutable state across invocations</li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      {/* Declarative Rules asl:rules */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold text-white tracking-tight">4. Declarative Rules Syntax (<code className="text-blue-400">```asl:rules</code>)</h2>
          <span className="text-[11px] font-mono rounded bg-zinc-800 px-2 py-0.5 text-zinc-300">AOT Transpiled</span>
        </div>
        <p className="text-sm text-zinc-300 leading-relaxed">
          For schema-driven tasks (commit message formatting, safety validation, data hygiene), ASL provides the <code className="text-zinc-200">asl:rules</code> DSL.
          Rules are compiled ahead-of-time (AOT) in memory directly into deterministic validation logic.
        </p>

        <div className="rounded-lg border border-zinc-800 bg-[#0d0d0d] p-4 font-mono text-xs text-zinc-300 overflow-x-auto">
          <pre>{`\`\`\`asl:rules
# 1. Precondition Guards (fail-fast assertions)
guard:
  input.payload is not empty else reject("Payload cannot be empty.")
  input.diff_stat is not null else reject("Missing diff_stat parameter.")

# 2. Pattern Matching Matrix
match input.intent:
  when starts_with any(["feat", "fix", "docs", "refactor"]) as prefix:
    accept(is_valid=true, commit_type=prefix, formatted=input.intent)
  when contains any(["bug", "error", "patch"]):
    accept(is_valid=true, commit_type="fix", formatted="fix: " + input.intent)
  otherwise:
    accept(is_valid=true, commit_type="chore", formatted="chore: " + input.intent)
\`\`\``}</pre>
        </div>

        <div className="space-y-2 pt-2 text-xs">
          <h3 className="text-sm font-semibold text-white font-mono">4.1 Rules Keywords &amp; Operators</h3>
          <ul className="space-y-1.5 text-zinc-300">
            <li><code className="text-blue-400 font-mono">guard:</code> — List of invariant preconditions evaluated before matching begins.</li>
            <li><code className="text-blue-400 font-mono">is not empty / is not null</code> — Nullability and non-zero length predicates.</li>
            <li><code className="text-blue-400 font-mono">else reject(&quot;reason&quot;)</code> — Action executed if guard condition evaluates to false.</li>
            <li><code className="text-blue-400 font-mono">match &lt;expression&gt;:</code> — Target expression for subsequent pattern clauses.</li>
            <li><code className="text-blue-400 font-mono">when starts_with / ends_with / contains / equals</code> — String and token matching primitives.</li>
            <li><code className="text-blue-400 font-mono">any([&quot;a&quot;, &quot;b&quot;]) as &lt;var&gt;</code> — Multi-pattern disjunction with captured match variable.</li>
            <li><code className="text-blue-400 font-mono">accept(field=value, ...)</code> — Success termination returning structured payload.</li>
            <li><code className="text-blue-400 font-mono">otherwise:</code> — Fallback case if no preceding pattern matched.</li>
          </ul>
        </div>
      </section>

      {/* The Canonical Formats */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">5. The Triad Format Extensions</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          ASL supports three canonical file extensions representing different operational roles in the AI agent lifecycle:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
            <div className="flex items-center justify-between">
              <span className="font-mono font-bold text-base text-white">.skill</span>
              <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
                Shadow Projected
              </span>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              Full-featured autonomous agent skill. Automatically projects a companion <code className="text-zinc-300 font-mono">.skill.md</code> shadow file with stripped executable blocks for legacy agent compatibility (Cursor, Claude Code).
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
            <div className="flex items-center justify-between">
              <span className="font-mono font-bold text-base text-white">.tool</span>
              <span className="text-[10px] font-mono rounded bg-zinc-800 border border-zinc-700 px-2 py-0.5 text-zinc-300">
                No Shadow
              </span>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              Atomic Model Context Protocol (MCP) tool. Optimized for direct programmatic registration without Markdown shadow projection overhead.
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
            <div className="flex items-center justify-between">
              <span className="font-mono font-bold text-base text-white">.asl</span>
              <span className="text-[10px] font-mono rounded bg-zinc-800 border border-zinc-700 px-2 py-0.5 text-zinc-300">
                Native Specification
              </span>
            </div>
            <p className="text-xs text-zinc-400 leading-relaxed">
              Native root language unit. Represents pure standalone deterministic algorithms and composite specifications in the ASL runtime.
            </p>
          </div>
        </div>
      </section>

      {/* CLI Reference */}
      <section className="space-y-4 border-t border-zinc-800 pt-8 pb-12">
        <h2 className="text-xl font-bold text-white tracking-tight">6. CLI Syntax &amp; Execution</h2>
        <div className="space-y-3 font-mono text-xs">
          <div className="rounded-lg border border-zinc-800 bg-black p-3 text-zinc-300 flex items-center justify-between">
            <code>asl run &lt;file.skill|tool|asl&gt; --input &#39;&#123;&quot;key&quot;: &quot;value&quot;&#125;&#39;</code>
            <span className="text-zinc-500 font-sans text-[11px]">Execute skill</span>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-black p-3 text-zinc-300 flex items-center justify-between">
            <code>asl check &lt;file&gt;</code>
            <span className="text-zinc-500 font-sans text-[11px]">Validate manifest &amp; syntax</span>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-black p-3 text-zinc-300 flex items-center justify-between">
            <code>asl digest &lt;file&gt;</code>
            <span className="text-zinc-500 font-sans text-[11px]">Calculate SHA-256 digest</span>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-black p-3 text-zinc-300 flex items-center justify-between">
            <code>asl shadow-project &lt;file.skill&gt;</code>
            <span className="text-zinc-500 font-sans text-[11px]">Generate .skill.md shadow</span>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-black p-3 text-zinc-300 flex items-center justify-between">
            <code>asl serve --mcp</code>
            <span className="text-zinc-500 font-sans text-[11px]">Start Model Context Protocol server</span>
          </div>
        </div>

        <div className="pt-6 flex items-center justify-between border-t border-zinc-800">
          <Link
            href="/docs"
            className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
          >
            ← Overview
          </Link>
          <Link
            href="/docs/triad"
            className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
          >
            <span>The Triad (.skill, .tool, .asl)</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
        </div>
      </section>
    </div>
  );
}
