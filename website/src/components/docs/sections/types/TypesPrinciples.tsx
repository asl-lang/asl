import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const TypesPrinciples: React.FC = () => {
  return (
    <DocsSection title="1. Architectural Principles">
      <p className="text-sm text-zinc-300 leading-relaxed">
        ASL operates on a <strong>strongly-typed, hermetically-scoped semantic runtime model</strong>. The type system is designed to eliminate undefined behavior, null-pointer dereferences, and floating-point non-determinism across disparate CPU architectures, authored in pure ASL and evaluated deterministically by the ASL VM (transpiled AOT to hermetic Starlark in-memory).
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
    </DocsSection>
  );
};
