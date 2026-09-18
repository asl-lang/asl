import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const VariablesSection: React.FC = () => {
  return (
    <DocsSection title="1. Variable Bindings & Scoping">
      <p className="text-sm text-zinc-300 leading-relaxed">
        Variables in ASL are strongly bound to their lexical scope. ASL enforces strict separation between module definitions and function execution heaps to guarantee 100% thread safety, deterministic outcomes, and zero cross-invocation state leakage.
      </p>

      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <h3 className="text-sm font-semibold font-mono text-white">1.1 Assignment & Shadowing</h3>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Variables are declared upon first assignment. Inside functions, assignments create or rebind local variables. Local variables may shadow module-level constants without mutating the outer identifier.
        </p>
        <AslCodeBlock
          lang="asl"
          code={`DEFAULT_RETRIES = 3   # Module-level constant

def execute(ctx, input):
    # Local variable declaration
    retries = input.get("retries", DEFAULT_RETRIES)
    
    # Augmented assignments supported: +=, -=, *=, //=, %=
    retries += 1
    
    # Multiple assignment / tuple unpacking
    status, code = ("success", 200)
    
    return {"retries": retries, "code": code, "status": status}`}
        />
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
    </DocsSection>
  );
};
