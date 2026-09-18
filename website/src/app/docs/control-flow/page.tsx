import React from "react";
import Link from "next/link";
import { ArrowRight, ShieldAlert, CheckCircle2, Lock, Flame, Ban, Check } from "lucide-react";

export default function ControlFlowReferencePage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Section 2
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Variables, Loops &amp; Functions
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Complete operational reference for variable bindings, lexical scoping, bounded iteration, and function definitions in Agent Skill Language (ASL 3.0) under the Axiom 5 Bounded Termination model.
        </p>
      </div>

      {/* 1. Variables & Bindings */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. Variable Bindings &amp; Scoping</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          Variables in ASL are strongly bound to their lexical scope. ASL enforces strict separation between module definitions and function execution heaps to guarantee 100% thread safety, deterministic outcomes, and zero cross-invocation state leakage.
        </p>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white">1.1 Assignment &amp; Shadowing</h3>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Variables are declared upon first assignment. Inside functions, assignments create or rebind local variables. Local variables may shadow module-level constants without mutating the outer identifier.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Variable Bindings & Scope
DEFAULT_RETRIES = 3   # Module-level constant

def execute(ctx, input):
    # Local variable declaration
    retries = input.get("retries", DEFAULT_RETRIES)
    
    # Augmented assignments supported: +=, -=, *=, //=, %=
    retries += 1
    
    # Multiple assignment / tuple unpacking
    status, code = ("success", 200)
    
    return {"retries": retries, "code": code, "status": status}
\`\`\``}</pre>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-emerald-400 font-mono">Isolated Ephemeral Heap</span>
            <p className="text-zinc-400">Each invocation runs in a clean memory sandbox. When <code className="text-zinc-300">execute()</code> returns, the heap is dropped immediately. No dirty state survives between calls.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-rose-400 font-mono">No Global Mutability</span>
            <p className="text-zinc-400">ASL disallows <code className="text-zinc-300">global</code> or <code className="text-zinc-300">nonlocal</code> mutation keywords. Pure code cannot introduce hidden side-channels or data races.</p>
          </div>
        </div>
      </section>

      {/* 2. Control Flow */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">2. Conditional Branching</h2>
          <p className="text-sm text-zinc-400 mt-1">Deterministic branch execution and short-circuit evaluation in ASL.</p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white">2.1 If / Elif / Else Statements</h3>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Standard indented syntax (4 spaces). Conditions evaluate with short-circuit boolean logic (<code className="text-zinc-200 font-mono">and</code>, <code className="text-zinc-200 font-mono">or</code>, <code className="text-zinc-200 font-mono">not</code>).
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Conditionals & Branching
def categorize_risk(score, mode):
    if score >= 90:
        grade = "CRITICAL"
    elif score >= 70:
        grade = "HIGH"
    elif score >= 40:
        grade = "MEDIUM"
    else:
        grade = "LOW"
        
    # Ternary Conditional Expression:
    threshold = 100 if mode == "strict" else 50
    return {"grade": grade, "threshold": threshold}
\`\`\``}</pre>
          </div>
        </div>
      </section>

      {/* 3. Bounded Iteration & Halting Proof */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">3. Bounded Loops &amp; Halting Proofs</h2>
          <p className="text-sm text-zinc-400 mt-1">Enforcing finite execution time via mathematical termination constraints (Axiom 5).</p>
        </div>

        {/* For loop */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold font-mono text-white">3.1 Bounded For Loops</h3>
            <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
              Guaranteed Termination
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            All iteration in ASL must be bounded over a finite iterable sequence (<code className="text-zinc-200 font-mono">list</code>, <code className="text-zinc-200 font-mono">dict</code>, <code className="text-zinc-200 font-mono">tuple</code>, <code className="text-zinc-200 font-mono">string</code>, or <code className="text-zinc-200 font-mono">range()</code>). The <code className="text-zinc-200 font-mono">break</code> and <code className="text-zinc-200 font-mono">continue</code> statements are fully supported.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Bounded Iteration & Comprehensions
def process_records(records):
    sanitized = []
    for r in records:
        if not r.get("active"):
            continue
        sanitized.append(r["name"].strip())
        if len(sanitized) >= 100:
            break  # Bounded early exit
            
    # List and Dict comprehensions:
    tags = [r.get("tag", "general") for r in records if "tag" in r]
    return {"sanitized": sanitized, "tags": tags}
\`\`\``}</pre>
          </div>
        </div>

        {/* The Strict Bans */}
        <div className="rounded-xl border border-rose-900/50 bg-rose-950/20 p-5 space-y-4">
          <div className="flex items-center gap-2 text-rose-400">
            <Ban className="h-4 w-4" />
            <h3 className="text-sm font-bold font-mono uppercase tracking-wide">
              Strictly Forbidden: Why while Loops and Recursion are Banned
            </h3>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            In general-purpose languages like Python, C, and Rust, unrestricted <code className="text-rose-300 font-mono">while</code> loops and recursive calls make the <em>Halting Problem</em> undecidable. An autonomous agent executing untrusted or LLM-generated code could enter an infinite loop or blow the call stack.
          </p>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs">
            <div className="rounded-lg border border-zinc-800 bg-black p-3.5 space-y-1.5">
              <span className="font-bold text-rose-400 font-mono">1. No While Loops</span>
              <p className="text-zinc-400">
                The <code className="text-zinc-200 font-mono">while</code> keyword is rejected at the parser tokenization stage. Loops must iterate strictly over finite collections whose length is known prior to loop execution.
              </p>
            </div>
            <div className="rounded-lg border border-zinc-850 bg-black p-3.5 space-y-1.5">
              <span className="font-bold text-rose-400 font-mono">2. No Recursion</span>
              <p className="text-zinc-400">
                Functions cannot call themselves directly or transitively. The call graph must form a Directed Acyclic Graph (DAG). Stack overflow is mathematically impossible.
              </p>
            </div>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3 text-xs text-zinc-400 flex items-center gap-3">
            <Flame className="h-5 w-5 text-amber-400 shrink-0" />
            <div>
              <strong className="text-white">Axiom 5 Bounded Termination: </strong>
              Every execution has an explicit monotonic fuel limit <code className="text-zinc-200 font-mono">limits.max_fuel_opcodes</code>. Compatible with the hermetic Starlark L1 runtime standard, execution is proven to terminate in finite steps <span className="font-mono text-zinc-200">O(F)</span>.
            </div>
          </div>
        </div>
      </section>

      {/* 4. Functions & Entrypoint */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">4. Functions &amp; Entrypoint Signatures</h2>
          <p className="text-sm text-zinc-400 mt-1">Defining procedures and the canonical capability entrypoint in ASL.</p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white">4.1 Function Declaration</h3>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Functions are declared with <code className="text-zinc-200 font-mono">def</code>. Parameters support positional arguments, default values, and keyword arguments.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# Pure helper procedure in ASL
def sanitize_token(token, uppercase=False):
    cleaned = token.strip().replace(" ", "_")
    return cleaned.upper() if uppercase else cleaned.lower()

# Multiple return values (tuple packing)
def split_name(full_name):
    parts = full_name.split(" ", 1)
    if len(parts) == 2:
        return parts[0], parts[1]
    return parts[0], ""
\`\`\``}</pre>
          </div>
        </div>

        {/* Entrypoint Signature */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-semibold font-mono text-white">4.2 The Capability Entrypoint</h3>
            <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
              Runtime Invocation Root
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            The function designated in <code className="text-zinc-200 font-mono">interface.entrypoint</code> (by default <code className="text-zinc-200 font-mono">execute</code>) accepts exactly two parameters:
          </p>
          <ul className="text-xs text-zinc-400 space-y-1.5 list-disc pl-5">
            <li><code className="text-white font-mono">ctx</code>: The capability context struct providing sandboxed host capabilities (<code className="text-zinc-300">ctx.fs</code>, <code className="text-zinc-300">ctx.crypto</code>, <code className="text-zinc-300">ctx.fuel</code>).</li>
            <li><code className="text-white font-mono">input</code>: The input dictionary validated against <code className="text-zinc-300">interface.input_schema</code>.</li>
          </ul>

          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
def execute(ctx, input):
    # 1. Access validated input fields
    filename = input.get("filename")
    sha_only = input.get("sha_only", False)

    # 2. Invoke sandboxed capability
    content = ctx.fs.read(filename)
    if content == None:
        return {"success": False, "error": "File not found"}

    # 3. Compute cryptographic hash
    digest = ctx.crypto.sha256(content)

    # 4. Return structured dictionary matching output_schema
    return {
        "success": True,
        "digest": digest,
        "fuel_used": ctx.fuel.consumed(),
    }
\`\`\``}</pre>
          </div>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/types"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Types &amp; Data Model
        </Link>
        <Link
          href="/docs/stdlib"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>Standard Library &amp; Builtins</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
