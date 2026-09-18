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
          Formal data type specifications, memory representation, immutability semantics, and JSON Schema boundary serialization for Agent Skill Language (ASL 3.0).
        </p>
      </div>

      {/* 1. Core Type System Architecture */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. Architectural Principles</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          ASL deterministic execution blocks operate on a <strong>strongly-typed, hermetically-scoped</strong> runtime model derived from Google&apos;s Starlark dialect. The type system is designed to completely eliminate undefined behavior, null-pointer dereferences, and floating-point non-determinism across disparate CPU microarchitectures.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-emerald-400 font-mono">Hermetic Boundaries</span>
            <p className="text-zinc-400">Values entering and exiting the runtime are strictly validated against JSON Schemas declared in the skill frontmatter.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-blue-400 font-mono">Zero Pointer Hazards</span>
            <p className="text-zinc-400">No memory pointers, no circular references, and no ambient global references. Garbage collection runs safely in an isolated per-execution arena.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-purple-400 font-mono">Microarchitecture Invariance</span>
            <p className="text-zinc-400">All arithmetic operations produce byte-identical results on x86_64, ARM64, and WebAssembly engines without divergence.</p>
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
            <pre>{`count = 42
hex_mask = 0xFF00
binary_flag = 0b1011
large_val = 1000000000000000000000000  # Exact precision, zero overflow risk

# Integer division and modulo:
quotient = 10 // 3   # 3 (floor division)
remainder = 10 % 3  # 1`}</pre>
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
            <pre>{`is_ready = True
has_errors = False

# Truthiness testing
if not has_errors and is_ready:
    status = "operational"`}</pre>
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
            Immutable sequences of valid UTF-8 characters. Indexed by character position (0-indexed). Supports Python-style slicing <code className="text-zinc-200 font-mono">[start:end:step]</code>, concatenation with <code className="text-zinc-200 font-mono">+</code>, repetition with <code className="text-zinc-200 font-mono">*</code>, and template formatting.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`name = "agent-executor"
multiline = """Line 1
Line 2 with UTF-8: 🚀"""

first_char = name[0]         # "a"
prefix = name[:5]            # "agent"
reversed_str = name[::-1]    # "rotucexe-tnega"
interpolated = "Hello, {}!".format("world")`}</pre>
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
          <p className="text-sm text-zinc-400 mt-1">Containers and structured records for data manipulation.</p>
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
            <pre>{`items = ["git", "docker", "rust"]
items.append("wasm")          # ["git", "docker", "rust", "wasm"]
items[1] = "podman"           # ["git", "podman", "rust", "wasm"]
first = items.pop(0)          # "git", items becomes ["podman", "rust", "wasm"]

# List comprehensions:
squares = [x * x for x in range(5) if x % 2 == 0]  # [0, 4, 16]`}</pre>
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
            <pre>{`config = {
    "version": 3,
    "enabled": True,
    "tags": ["core", "prod"]
}

# Defensive access (safe against missing keys):
env = config.get("env", "development")

# Key inspection:
has_version = "version" in config   # True`}</pre>
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
            <pre>{`point = (10, 20)
x, y = point               # Destructuring / Unpacking
matrix_cell = {(0, 1): "val"} # Valid as dict key`}</pre>
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
            Created via the builtin <code className="text-zinc-200 font-mono">struct(key=val)</code> constructor. Provides dot-accessible attributes and strict immutability. Used by the ASL runtime to inject capability handles on the <code className="text-zinc-200 font-mono">ctx</code> object.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`# Creating a structured record
user = struct(id=42, name="alice", role="admin")

# Dot access
user_name = user.name      # "alice"
# user.name = "bob"        # COMPILE/RUNTIME ERROR: structs are immutable!`}</pre>
          </div>
        </div>
      </section>

      {/* 4. JSON Schema Mapping Matrix */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">4. JSON Schema Mapping Matrix</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          When skills are executed, arguments conforming to <code className="text-zinc-200 font-mono">input_schema</code> are deserialized into ASL types, and return values are verified against <code className="text-zinc-200 font-mono">output_schema</code>:
        </p>

        <div className="overflow-x-auto rounded-lg border border-zinc-800">
          <table className="w-full text-left text-xs">
            <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
              <tr>
                <th className="p-3">JSON Schema Type</th>
                <th className="p-3">ASL Runtime Type</th>
                <th className="p-3">Rust Internal Representation</th>
                <th className="p-3">Serialization Behavior</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
              <tr>
                <td className="p-3 text-blue-400">&quot;string&quot;</td>
                <td className="p-3 text-white">string</td>
                <td className="p-3 text-zinc-400">starlark::values::string::StarlarkStr</td>
                <td className="p-3 font-sans">Strict UTF-8 encoded string.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;integer&quot;</td>
                <td className="p-3 text-white">int</td>
                <td className="p-3 text-zinc-400">starlark::values::int::StarlarkInt</td>
                <td className="p-3 font-sans">Zero-loss unbounded integer.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;boolean&quot;</td>
                <td className="p-3 text-white">bool</td>
                <td className="p-3 text-zinc-400">starlark::values::bool::StarlarkBool</td>
                <td className="p-3 font-sans">Strict boolean (<code className="text-zinc-200">true</code> / <code className="text-zinc-200">false</code>).</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;array&quot;</td>
                <td className="p-3 text-white">list</td>
                <td className="p-3 text-zinc-400">starlark::values::list::List</td>
                <td className="p-3 font-sans">Zero-indexed JSON array.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;object&quot;</td>
                <td className="p-3 text-white">dict</td>
                <td className="p-3 text-zinc-400">starlark::values::dict::Dict</td>
                <td className="p-3 font-sans">String-keyed JSON object map.</td>
              </tr>
              <tr>
                <td className="p-3 text-blue-400">&quot;null&quot;</td>
                <td className="p-3 text-white">NoneType (None)</td>
                <td className="p-3 text-zinc-400">starlark::values::none::NoneType</td>
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
