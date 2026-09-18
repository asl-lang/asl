import React from "react";
import { Ban, Flame } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const BoundedLoopsSection: React.FC = () => {
  return (
    <DocsSection
      title="3. Bounded Loops & Halting Proofs"
      subtitle="Enforcing finite execution time via mathematical termination constraints (Axiom 5)."
    >
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
        <AslCodeBlock
          lang="asl"
          code={`def process_records(records):
    sanitized = []
    for r in records:
        if not r.get("active"):
            continue
        sanitized.append(r["name"].strip())
        if len(sanitized) >= 100:
            break  # Bounded early exit
            
    # List and Dict comprehensions:
    tags = [r.get("tag", "general") for r in records if "tag" in r]
    return {"sanitized": sanitized, "tags": tags}`}
        />
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
            Every execution has an explicit monotonic fuel limit <code className="text-zinc-200 font-mono">limits.max_fuel_opcodes</code>. Transpiled AOT into hermetic Starlark in RAM, execution is proven to terminate in finite steps <span className="font-mono text-zinc-200">O(F)</span>.
          </div>
        </div>
      </div>
    </DocsSection>
  );
};
