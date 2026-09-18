import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const CompositeTypes: React.FC = () => {
  return (
    <DocsSection
      title="3. Composite Data Structures"
      subtitle="Containers and structured records for data manipulation in ASL."
    >
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
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    items = ["git", "docker", "rust"]
    items.append("asl")
    first = items.pop(0)          # "git"

    # Comprehensions:
    squares = [x * x for x in range(5) if x % 2 == 0]  # [0, 4, 16]
    return {"items": items, "first": first, "squares": squares}`}
        />
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
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    config = {
        "version": 3,
        "enabled": True,
        "tags": ["core", "prod"]
    }

    env = config.get("env", "development")
    is_prod = "prod" in config.get("tags", [])
    
    return {"env": env, "is_prod": is_prod}`}
        />
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
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    point = (10, 20)
    x, y = point               # Destructuring / Unpacking
    matrix = {(0, 1): "cell_value"} # Valid as dict key
    
    return {"x": x, "y": y, "val": matrix[(0, 1)]}`}
        />
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
          Created via the ASL builtin <code className="text-zinc-200 font-mono">struct(key=val)</code> constructor. Provides dot-accessible attributes and strict immutability.
        </p>
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    user = struct(id=42, name="alice", role="admin")
    return {"user_name": user.name, "role": user.role}`}
        />
      </div>
    </DocsSection>
  );
};
