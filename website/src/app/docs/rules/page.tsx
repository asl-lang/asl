import React from "react";
import Link from "next/link";
import { ArrowRight, BookOpen, Code2, Sparkles, CheckCircle2 } from "lucide-react";

export default function RulesDocsPage() {
  return (
    <div className="space-y-8">
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language • Track 2
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Declarative Semantic Rules (asl:rules)
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed">
          High-level, expressive rule semantics that transpile in memory to hermetic Starlark bytecode with zero runtime overhead.
        </p>
      </div>

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <p>
          While ASL supports writing raw deterministic code blocks via <code className="text-zinc-200 font-mono">```asl:deterministic</code> in Starlark, human authors and LLMs often express domain policies more reliably as declarative rules. The <code className="text-zinc-200 font-mono">```asl:rules</code> block enables writing business rules that are automatically transpiled into deterministic Starlark before execution.
        </p>

        <h2 className="text-xl font-bold text-white tracking-tight pt-2">
          Rule Syntax Specification
        </h2>
        <p>
          Each rule consists of a unique name, a <code className="text-zinc-200 font-mono">when:</code> condition block, and a <code className="text-zinc-200 font-mono">then:</code> mutation block:
        </p>

        <div className="rounded-xl border border-zinc-800 bg-black p-4 font-mono text-xs text-zinc-300 leading-relaxed">
          <pre>
{`\`\`\`asl:rules
rule "check_bug_fix":
  when:
    input.intent matches "(?i)^(corrigir|fix|bug)"
  then:
    set is_valid = true
    set commit_type = "fix"
    set formatted_message = "fix: " + input.intent

rule "enforce_non_empty":
  when:
    input.intent == ""
  then:
    set is_valid = false
    set commit_type = "unknown"
    set error = "Commit intent is mandatory"
\`\`\``}
          </pre>
        </div>

        <h2 className="text-xl font-bold text-white tracking-tight pt-4">
          In-Memory Transpilation to Strict Starlark L1
        </h2>
        <p>
          During parsing, <code className="text-zinc-200">asl-parser</code> parses the AST and generates pure, type-safe Starlark code. The transpiler guarantees:
        </p>
        <ul className="list-disc pl-5 space-y-2 text-zinc-400">
          <li><strong className="text-zinc-200">Regex Optimization:</strong> Strings matched with <code className="text-zinc-200">matches</code> are evaluated via the hermetic <code className="text-zinc-200">ctx.regex.is_match()</code> capability.</li>
          <li><strong className="text-zinc-200">Safe Key Access:</strong> Input fields are accessed defensively using <code className="text-zinc-200">input.get("field", "")</code> to prevent unhandled KeyError exceptions.</li>
          <li><strong className="text-zinc-200">Immutable Scope:</strong> Rules cannot leak state between runs or tamper with capability handles.</li>
        </ul>

        <div className="pt-6 flex items-center justify-between border-t border-zinc-850">
          <Link
            href="/docs/triad"
            className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200"
          >
            <span>← The Canonical Triad</span>
          </Link>
          <Link
            href="/docs/axioms"
            className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300"
          >
            <span>The 7 Axioms & Security</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
        </div>
      </div>
    </div>
  );
}
