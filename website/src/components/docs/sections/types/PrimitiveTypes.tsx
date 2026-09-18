import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const PrimitiveTypes: React.FC = () => {
  return (
    <DocsSection
      title="2. Primitive Types"
      subtitle="Fundamental atomic value types provided by the ASL runtime."
    >
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
          Represents exact, unbounded mathematical integers without 32 or 64-bit overflow hazards. Supports decimal, hexadecimal (<code className="text-zinc-200 font-mono">0xFF</code>), binary (<code className="text-zinc-200 font-mono">0b1010</code>), and octal (<code className="text-zinc-200 font-mono">0o755</code>) literals.
        </p>
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    count = 42
    hex_mask = 0xFF00
    binary_flag = 0b1011

    # Integer division and modulo:
    quotient = 10 // 3   # 3 (floor division)
    remainder = 10 % 3  # 1

    return {"count": count, "quotient": quotient, "remainder": remainder}`}
        />
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
          Contains exactly two values: <code className="text-zinc-200 font-mono">True</code> and <code className="text-zinc-200 font-mono">False</code>. Falsy values are <code className="text-zinc-200 font-mono">0</code>, <code className="text-zinc-200 font-mono">&quot;&quot;</code>, <code className="text-zinc-200 font-mono">[]</code>, <code className="text-zinc-200 font-mono">&#123;&#125;</code>, and <code className="text-zinc-200 font-mono">None</code>.
        </p>
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    is_ready = input.get("ready", True)
    has_errors = False

    status = "operational" if (not has_errors and is_ready) else "blocked"
    return {"status": status, "is_ready": is_ready}`}
        />
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
          Immutable sequences of valid UTF-8 characters. Indexed by character position (0-indexed). Supports slicing <code className="text-zinc-200 font-mono">[start:end:step]</code> and template formatting.
        </p>
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    name = input.get("agent_name", "asl-executor")
    prefix = name[:3]            # "asl"
    reversed_str = name[::-1]
    formatted = "Active agent: {}".format(name)
    
    return {"prefix": prefix, "formatted": formatted, "reversed": reversed_str}`}
        />
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
          The singleton value <code className="text-zinc-200 font-mono">None</code> denotes the absence of a value. Functions without an explicit return statement implicitly return <code className="text-zinc-200 font-mono">None</code>. Check with <code className="text-zinc-200 font-mono">x is None</code> or <code className="text-zinc-200 font-mono">x == None</code>.
        </p>
      </div>
    </DocsSection>
  );
};
