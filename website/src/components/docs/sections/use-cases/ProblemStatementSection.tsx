import React from "react";
import { Sparkles } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const ProblemStatementSection: React.FC = () => {
  return (
    <DocsSection
      title="The Core Architectural Problem ASL Solves"
      subtitle="Why building autonomous systems with raw Python/Bash or unconstrained prompts fails at enterprise scale."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
        <div className="rounded-xl border border-rose-900/40 bg-rose-950/15 p-5 space-y-2">
          <h3 className="font-bold text-rose-400 font-mono flex items-center gap-2">
            <span>Fatal Mode A: The Ambient Authority Trap (Python/Bash)</span>
          </h3>
          <p className="text-zinc-300 leading-relaxed">
            Giving an LLM access to a Python interpreter or shell script means giving it <strong>unbounded ambient authority</strong>. A confused or compromised model can run <code className="text-rose-300 font-mono">os.system(&quot;rm -rf /&quot;)</code>, leak cloud credentials, run infinite loops, or trigger non-deterministic math.
          </p>
        </div>

        <div className="rounded-xl border border-amber-900/40 bg-amber-950/15 p-5 space-y-2">
          <h3 className="font-bold text-amber-400 font-mono flex items-center gap-2">
            <span>Fatal Mode B: The Hallucination Hazard (Raw Prompts)</span>
          </h3>
          <p className="text-zinc-300 leading-relaxed">
            Relying solely on natural language prompts has zero mathematical guarantees. The model can hallucinate discounts, skip mandatory compliance checks, output malformed JSON, and blow millions of tokens re-parsing prompt context.
          </p>
        </div>
      </div>

      <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 text-xs text-zinc-300 flex items-center gap-3">
        <Sparkles className="h-5 w-5 text-emerald-400 shrink-0" />
        <div>
          <strong className="text-white font-mono">The ASL Synthesis: </strong>
          The LLM provides the creative semantic reasoning (Region 2), while the sandboxed ASL runtime enforces mathematical invariants, strict schemas, gas budgets, and cryptographic audit trails (Regions 1 &amp; 3).
        </div>
      </div>
    </DocsSection>
  );
};
