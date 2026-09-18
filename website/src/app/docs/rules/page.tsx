import React from "react";
import Link from "next/link";
import { ArrowRight, Sparkles, ShieldCheck, Cpu, Code2, CheckCircle2 } from "lucide-react";

export default function RulesDocsPage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Section 5
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Declarative Rules (<code className="text-blue-400 font-mono">asl:rules</code>)
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Complete grammar specification and compilation reference for high-level semantic rules, pattern-matching matrices, and in-memory ahead-of-time (AOT) transpilation.
        </p>
      </div>

      {/* 1. Philosophy & Purpose */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. Semantic DSL Architecture</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          While ASL allows writing imperative Starlark code directly in <code className="text-zinc-200 font-mono">```asl:deterministic</code> blocks, human engineers and LLMs reason most effectively about intent, guards, and validation matrices in declarative format.
        </p>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The <code className="text-zinc-200 font-mono">```asl:rules</code> domain-specific language (DSL) provides formal constructs for precondition verification and pattern dispatch. The ASL parser transpiles this block in memory ahead-of-time directly into hermetic Starlark L1 bytecode before VM execution.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-emerald-400 font-mono">Fail-Fast Guards</span>
            <p className="text-zinc-400">Preconditions reject invalid inputs before any pattern matching or resource-consuming logic runs.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-blue-400 font-mono">Pattern Matrix</span>
            <p className="text-zinc-400">Multi-pattern disjunction with captured alias bindings for prefixes, substrings, and suffixes.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-purple-400 font-mono">Zero Overhead</span>
            <p className="text-zinc-400">Compiles into optimized pure branch trees with zero external runtime or interpreter overhead.</p>
          </div>
        </div>
      </section>

      {/* 2. Grammar Specification */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">2. Grammar &amp; Clause Specification</h2>
          <p className="text-sm text-zinc-400 mt-1">Formal syntax for guards, match clauses, and terminal actions.</p>
        </div>

        {/* Guard Clauses */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white">2.1 Guard Clauses (<code className="text-blue-400">guard:</code>)</h3>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Evaluated sequentially before match clauses. If any guard predicate fails, execution terminates immediately with the specified rejection message.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`guard:
  input.payload is not empty else reject("Payload cannot be empty.")
  input.actor is not null else reject("Actor identifier is required.")
  input.retries >= 0 else reject("Retries count cannot be negative.")`}</pre>
          </div>
          <div className="overflow-x-auto rounded-lg border border-zinc-850">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
                <tr>
                  <th className="p-2.5">Predicate</th>
                  <th className="p-2.5 font-sans">Semantics</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-850 text-zinc-300">
                <tr>
                  <td className="p-2.5 text-blue-400">is not empty / is empty</td>
                  <td className="p-2.5 font-sans">Checks that string or list has non-zero length after trimming.</td>
                </tr>
                <tr>
                  <td className="p-2.5 text-blue-400">is not null / is null</td>
                  <td className="p-2.5 font-sans">Checks non-nullness against <code className="text-zinc-200">None</code>.</td>
                </tr>
                <tr>
                  <td className="p-2.5 text-blue-400">is true / is false</td>
                  <td className="p-2.5 font-sans">Strict boolean equivalence assertion.</td>
                </tr>
                <tr>
                  <td className="p-2.5 text-blue-400">&gt;=, &lt;=, ==, !=</td>
                  <td className="p-2.5 font-sans">Relational comparison against scalar literal values.</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        {/* Pattern Matching */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white">2.2 Pattern Matching (<code className="text-blue-400">match:</code>)</h3>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Evaluates candidate conditions against an expression target. The first matching <code className="text-zinc-200 font-mono">when</code> branch executes and terminates evaluation.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`match input.commit_message:
  when starts_with any(["feat", "fix", "docs", "chore"]) as prefix:
    accept(valid=True, type=prefix, clean_msg=input.commit_message)

  when contains any(["[skip ci]", "[wip]"]):
    reject("WIP or skip commits are barred from release pipeline.")

  when equals "initial commit":
    accept(valid=True, type="init", clean_msg="initial commit")

  otherwise:
    reject("Commit message does not adhere to Conventional Commits format.")`}</pre>
          </div>
        </div>

        {/* Match Primitives */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white">2.3 Pattern Primitives &amp; Disjunctions</h3>
          <ul className="text-xs text-zinc-300 space-y-2 font-mono">
            <li><strong className="text-white">starts_with any([...]) [as &lt;var&gt;]:</strong> Matches if target string starts with any prefix in the array. Optionally binds matching prefix to variable.</li>
            <li><strong className="text-white">ends_with any([...]) [as &lt;var&gt;]:</strong> Matches if target ends with any suffix.</li>
            <li><strong className="text-white">contains any([...]):</strong> Matches if target contains any substring.</li>
            <li><strong className="text-white">matches_regex(&quot;...&quot;):</strong> Matches against regex pattern.</li>
            <li><strong className="text-white">equals &lt;expr&gt;:</strong> Exact value equality comparison.</li>
            <li><strong className="text-white">accept(key=val, ...):</strong> Emits success dictionary conforming to <code className="text-zinc-400">output_schema</code>.</li>
            <li><strong className="text-white">reject(&quot;message&quot;):</strong> Emits structured rejection dictionary with diagnostic error string.</li>
            <li><strong className="text-white">otherwise:</strong> Default fallback branch when no preceding patterns match.</li>
          </ul>
        </div>
      </section>

      {/* 3. AOT Transpilation Pipeline */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">3. In-Memory AOT Transpilation Pipeline</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The <code className="text-zinc-200 font-mono">asl-parser</code> crate processes the rules block during document ingestion:
        </p>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="space-y-2 text-xs text-zinc-400 font-sans">
            <div className="flex items-start gap-2">
              <span className="font-mono text-emerald-400 font-bold">Step 1:</span>
              <span><strong>Lexical Analysis:</strong> Scans tokens and validates syntax indentation.</span>
            </div>
            <div className="flex items-start gap-2">
              <span className="font-mono text-emerald-400 font-bold">Step 2:</span>
              <span><strong>AST Construction:</strong> Builds strongly-typed <code className="text-zinc-200 font-mono">RulesBlock</code> with validated expressions.</span>
            </div>
            <div className="flex items-start gap-2">
              <span className="font-mono text-emerald-400 font-bold">Step 3:</span>
              <span><strong>Starlark Emission:</strong> Generates hermetic Python-dialect code wrapped in the entrypoint function, injecting safe key access (<code className="text-zinc-200 font-mono">_asl_get</code>) and pattern helpers (<code className="text-zinc-200 font-mono">_asl_starts_with_any</code>).</span>
            </div>
            <div className="flex items-start gap-2">
              <span className="font-mono text-emerald-400 font-bold">Step 4:</span>
              <span><strong>AOT Syntax Verification:</strong> Emitted code is verified with <code className="text-zinc-200 font-mono">AstModule::parse</code> before writing, ensuring syntax bugs can never reach production runtime.</span>
            </div>
          </div>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/context"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Capability Context (ctx)
        </Link>
        <Link
          href="/docs/triad"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>The Triad (.skill, .tool, .asl)</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
