import React from "react";
import Link from "next/link";
import { ArrowRight, CheckCircle2, ShieldAlert, Code2, Layers, Cpu } from "lucide-react";

export default function TypesReferencePage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Section 1
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Types &amp; Data Model
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Formal data type specifications, memory representation, immutability semantics, and JSON Schema boundary serialization in Agent Skill Language (ASL 3.0).
        </p>
      </div>

      {/* 1. Core Type System Architecture */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. Architectural Principles</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          ASL operates on a <strong>strongly-typed, hermetically-scoped semantic runtime model</strong>. The type system is designed to eliminate undefined behavior, null-pointer dereferences, and floating-point non-determinism across disparate CPU architectures, while maintaining full binary compatibility with the Starlark L1 execution standard.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-emerald-400 font-mono">Hermetic Boundaries</span>
            <p className="text-zinc-400">Values entering and exiting ASL are strictly validated against JSON Schemas declared in the skill frontmatter before code executes.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-blue-400 font-mono">Zero Pointer Hazards</span>
            <p className="text-zinc-400">No raw pointers, no circular references, and no ambient global state. Execution memory is collected safely in an isolated per-invocation arena.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-purple-400 font-mono">Architecture Invariance</span>
            <p className="text-zinc-400">All arithmetic and string operations produce byte-identical results across x86_64, ARM64, and WebAssembly targets.</p>
          </div>
        </div>
      </section>

      {/* 2. Primitive Types */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">2. Primitive Types</h2>
          <p className="text-sm text-zinc-400 mt-1">Fundamental atomic value types provided by the ASL runtime.</p>
        </div>

        {/* Integer */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">int</span>
              <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
                Arbitrary-Precision Integer
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;int&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Represents exact, unbounded mathematical integers. ASL integers do not overflow or wrap around at 32 or 64 bits. Supports decimal, hexadecimal (<code className="text-zinc-200 font-mono">0xFF</code>), binary (<code className="text-zinc-200 font-mono">0b1010</code>), and octal (<code className="text-zinc-200 font-mono">0o755</code>) literals.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Integer Arithmetic
def execute(ctx, input):
    count = 42
    hex_mask = 0xFF00
    binary_flag = 0b1011
    large_val = 1000000000000000000000000  # Exact precision, zero overflow risk

    # Integer division and modulo:
    quotient = 10 // 3   # 3 (floor division)
    remainder = 10 % 3  # 1

    return {"count": count, "quotient": quotient, "remainder": remainder}
\`\`\``}</pre>
          </div>
        </div>

        {/* Boolean */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">bool</span>
              <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
                Boolean Truth Value
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;bool&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Contains exactly two values: <code className="text-zinc-200 font-mono">True</code> and <code className="text-zinc-200 font-mono">False</code>. Truth-value testing considers <code className="text-zinc-200 font-mono">0</code>, <code className="text-zinc-200 font-mono">&quot;&quot;</code>, <code className="text-zinc-200 font-mono">[]</code>, <code className="text-zinc-200 font-mono">&#123;&#125;</code>, and <code className="text-zinc-200 font-mono">None</code> as falsy. All other values evaluate to truthy.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Boolean Branching
def execute(ctx, input):
    is_ready = input.get("ready", True)
    has_errors = False

    status = "operational" if (not has_errors and is_ready) else "blocked"
    return {"status": status, "is_ready": is_ready}
\`\`\``}</pre>
          </div>
        </div>

        {/* String */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">string</span>
              <span className="text-[10px] font-mono rounded bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 text-amber-400">
                Immutable UTF-8 Sequence
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;string&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Immutable sequences of valid UTF-8 characters. Indexed by character position (0-indexed). Supports slicing <code className="text-zinc-200 font-mono">[start:end:step]</code>, concatenation with <code className="text-zinc-200 font-mono">+</code>, repetition with <code className="text-zinc-200 font-mono">*</code>, and template formatting.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL UTF-8 String Processing
def execute(ctx, input):
    name = input.get("agent_name", "asl-executor")
    first_char = name[0]         # "a"
    prefix = name[:3]            # "asl"
    reversed_str = name[::-1]
    formatted = "Active agent: {}".format(name)
    
    return {"prefix": prefix, "formatted": formatted, "reversed": reversed_str}
\`\`\``}</pre>
          </div>
        </div>

        {/* NoneType */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">NoneType (None)</span>
              <span className="text-[10px] font-mono rounded bg-zinc-800 border border-zinc-700 px-2 py-0.5 text-zinc-400">
                Unit / Null Value
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;NoneType&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            The singleton value <code className="text-zinc-200 font-mono">None</code> denotes the absence of a value. Functions without an explicit return statement implicitly return <code className="text-zinc-200 font-mono">None</code>. Check using the identity comparison <code className="text-zinc-200 font-mono">x == None</code> or <code className="text-zinc-200 font-mono">x is None</code>.
          </p>
        </div>
      </section>

      {/* 3. Composite & Data Structures */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">3. Composite Data Structures</h2>
          <p className="text-sm text-zinc-400 mt-1">Containers and structured records for data manipulation in ASL.</p>
        </div>

        {/* List */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">list</span>
              <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
                Mutable Ordered Sequence
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;list&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Heterogeneous ordered sequences. Items can be mutated, appended, extended, and sliced inside function bodies.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL List Operations
def execute(ctx, input):
    items = ["git", "docker", "rust"]
    items.append("asl")
    first = items.pop(0)          # "git"

    # Comprehensions:
    squares = [x * x for x in range(5) if x % 2 == 0]  # [0, 4, 16]
    return {"items": items, "first": first, "squares": squares}
\`\`\``}</pre>
          </div>
        </div>

        {/* Dict */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">dict</span>
              <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
                Associative Hash Map
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;dict&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Key-value associative maps. Keys must be hashable immutable types (<code className="text-zinc-200 font-mono">string</code>, <code className="text-zinc-200 font-mono">int</code>, <code className="text-zinc-200 font-mono">bool</code>, <code className="text-zinc-200 font-mono">tuple</code>).
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Dictionary Operations
def execute(ctx, input):
    config = {
        "version": 3,
        "enabled": True,
        "tags": ["core", "prod"]
    }

    # Defensive access:
    env = config.get("env", "development")
    is_prod = "prod" in config.get("tags", [])
    
    return {"env": env, "is_prod": is_prod}
\`\`\``}</pre>
          </div>
        </div>

        {/* Tuple */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">tuple</span>
              <span className="text-[10px] font-mono rounded bg-purple-500/10 border border-purple-500/20 px-2 py-0.5 text-purple-400">
                Immutable Ordered Sequence
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;tuple&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Fixed-size immutable collections. Tuples can be used as dictionary keys because of their strict immutability.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Tuples & Destructuring
def execute(ctx, input):
    point = (10, 20)
    x, y = point               # Destructuring / Unpacking
    matrix = {(0, 1): "cell_value"} # Valid as dict key
    
    return {"x": x, "y": y, "val": matrix[(0, 1)]}
\`\`\``}</pre>
          </div>
        </div>

        {/* Struct */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">struct</span>
              <span className="text-[10px] font-mono rounded bg-cyan-500/10 border border-cyan-500/20 px-2 py-0.5 text-cyan-400">
                Immutable Record Object
              </span>
            </div>
            <code className="text-xs font-mono text-zinc-500">type(x) == &quot;struct&quot;</code>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Created via the ASL builtin <code className="text-zinc-200 font-mono">struct(key=val)</code> constructor. Provides dot-accessible attributes and strict immutability. Used by the ASL runtime to inject capability handles on the <code className="text-zinc-200 font-mono">ctx</code> object.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Struct Records
def execute(ctx, input):
    user = struct(id=42, name="alice", role="admin")
    
    # Dot access syntax
    return {"user_name": user.name, "role": user.role}
\`\`\``}</pre>
          </div>
        </div>
      </section>

      {/* 4. JSON Schema Mapping Matrix */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">4. JSON Schema Mapping Matrix</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          When skills are executed, arguments conforming to <code className="text-zinc-200 font-mono">input_schema</code> are deserialized into ASL native types, and return values are verified against <code className="text-zinc-200 font-mono">output_schema</code>:
        </p>

        <div className="overflow-x-auto rounded-lg border border-zinc-800">
          <table className="w-full text-left text-xs">
            <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
              <tr>
                <th className="p-3">JSON Schema Type</th>
                <th className="p-3">ASL Runtime Type</th>
                <th className="p-3">Runtime Engine Mapping</th>
                <th className="p-3">Serialization Behavior</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
              <tr>
                <td className="p-3 text-blue-400">&quot;string&quot;</td>
                <td className="p-3 text-white">string</td>
                <td className="p-3 text-zinc-400">ASL VM (Starlark L1 compatible)</td>
                <td className="p-3 font-sans">Strict UTF-8 encoded string.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;integer&quot;</td>
                <td className="p-3 text-white">int</td>
                <td className="p-3 text-zinc-400">ASL VM (Starlark L1 compatible)</td>
                <td className="p-3 font-sans">Zero-loss unbounded integer.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;boolean&quot;</td>
                <td className="p-3 text-white">bool</td>
                <td className="p-3 text-zinc-400">ASL VM (Starlark L1 compatible)</td>
                <td className="p-3 font-sans">Strict boolean (<code className="text-zinc-200">true</code> / <code className="text-zinc-200">false</code>).</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;array&quot;</td>
                <td className="p-3 text-white">list</td>
                <td className="p-3 text-zinc-400">ASL VM (Starlark L1 compatible)</td>
                <td className="p-3 font-sans">Zero-indexed JSON array.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;object&quot;</td>
                <td className="p-3 text-white">dict</td>
                <td className="p-3 text-zinc-400">ASL VM (Starlark L1 compatible)</td>
                <td className="p-3 font-sans">String-keyed JSON object map.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;null&quot;</td>
                <td className="p-3 text-white">NoneType (None)</td>
                <td className="p-3 text-zinc-400">ASL VM (Starlark L1 compatible)</td>
                <td className="p-3 font-sans">JSON literal <code className="text-zinc-200">null</code>.</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/syntax"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Syntax &amp; File Anatomy
        </Link>
        <Link
          href="/docs/control-flow"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>Variables, Loops &amp; Functions</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
