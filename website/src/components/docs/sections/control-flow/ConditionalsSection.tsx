import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const ConditionalsSection: React.FC = () => {
  return (
    <DocsSection
      title="2. Conditional Branching"
      subtitle="Deterministic branch execution and short-circuit evaluation in ASL."
    >
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <h3 className="text-sm font-semibold font-mono text-white">2.1 If / Elif / Else Statements</h3>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Standard indented syntax (4 spaces). Conditions evaluate with short-circuit boolean logic (<code className="text-zinc-200 font-mono">and</code>, <code className="text-zinc-200 font-mono">or</code>, <code className="text-zinc-200 font-mono">not</code>).
        </p>
        <AslCodeBlock
          lang="asl"
          code={`def categorize_risk(score, mode):
    if score >= 90:
        grade = "CRITICAL"
    elif score >= 70:
        grade = "HIGH"
    elif score >= 40:
        grade = "MEDIUM"
    else:
        grade = "LOW"
        
    # Ternary Conditional Expression:
    threshold = 100 if mode == "strict" else 50
    return {"grade": grade, "threshold": threshold}`}
        />
      </div>
    </DocsSection>
  );
};
