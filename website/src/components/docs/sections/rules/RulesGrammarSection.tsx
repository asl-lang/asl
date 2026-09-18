import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const RulesGrammarSection: React.FC = () => {
  return (
    <DocsSection
      title="1. Formal Grammar & Clause Primitives"
      subtitle="Syntax for fail-fast precondition assertions and pattern-matching matrices in ASL."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs font-mono">
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <span className="text-emerald-400 font-bold">1. Guard Clauses (Preconditions)</span>
          <p className="text-zinc-400 font-sans">Evaluated sequentially before matching begins. Rejects invalid requests immediately.</p>
          <AslCodeBlock
            lang="asl"
            code={`guard:
  input.payload is not empty else reject("Empty payload")
  input.amount > 0 else reject("Invalid amount")
  input.verified is true else reject("Unverified account")`}
          />
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <span className="text-blue-400 font-bold">2. Pattern Match Matrix</span>
          <p className="text-zinc-400 font-sans">Multi-pattern disjunction with captured alias bindings for prefixes and substrings.</p>
          <AslCodeBlock
            lang="asl"
            code={`match input.target:
  when starts_with any(["prod-", "us-east-"]) as region:
    accept(status="routed", region=region)
  when contains any(["[urgent]", "[hotfix]"]):
    accept(status="expedited", priority=1)
  otherwise:
    reject("No routing policy matched")`}
          />
        </div>
      </div>
    </DocsSection>
  );
};
