import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const PipelineModelSection: React.FC = () => {
  return (
    <DocsSection
      title="1. The Interleaved Pipeline Model"
      subtitle="How ASL combines natural language intent stages and deterministic execution blocks."
    >
      <p className="text-sm text-zinc-300 leading-relaxed">
        In real-world enterprise deployments (e.g. database migrations, CI/CD gates, financial reconciliations), a skill cannot be a simple 5-line script. It requires a <strong>multi-stage pipeline</strong> where the neural reasoner (LLM) and deterministic engine collaborate across progressive verification checkpoints.
      </p>
      <p className="text-sm text-zinc-300 leading-relaxed">
        The ASL parser natively supports interleaving multiple CommonMark prompt sections with multiple code blocks. During compilation:
      </p>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
          <span className="font-semibold text-emerald-400 font-mono">1. Code Assembly</span>
          <p className="text-zinc-400">All fenced blocks (<code className="text-zinc-300 font-mono">```asl:rules</code> and <code className="text-zinc-300 font-mono">```asl:deterministic</code>) are merged in source order into a single unified ASL execution module (compatible with the Starlark L1 runtime standard).</p>
        </div>
        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
          <span className="font-semibold text-blue-400 font-mono">2. Semantic Preservation</span>
          <p className="text-zinc-400">All Markdown headings, step protocols, and few-shot examples are preserved byte-for-byte in the prompt envelope for the LLM.</p>
        </div>
        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
          <span className="font-semibold text-purple-400 font-mono">3. Single-Turn Invariance</span>
          <p className="text-zinc-400">The entire multi-stage protocol resides in one atomic file, guaranteeing 100% KV-cache hit rate across turns.</p>
        </div>
      </div>
    </DocsSection>
  );
};
