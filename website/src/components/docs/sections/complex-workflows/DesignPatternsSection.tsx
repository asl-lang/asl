import React from "react";
import { Workflow, Flame, ShieldCheck, GitBranch } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const DesignPatternsSection: React.FC = () => {
  return (
    <DocsSection
      title="3. Enterprise Design Patterns"
      subtitle="Architectural principles for authoring fault-tolerant, auditable ASL pipelines."
    >
      <p className="text-sm text-zinc-300 leading-relaxed">
        When scaling skills to complex multi-step systems, adhere to these production patterns:
      </p>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <h3 className="font-bold text-white font-mono flex items-center gap-2">
            <Workflow className="h-4 w-4 text-blue-400" />
            <span>Modular Decomposition</span>
          </h3>
          <p className="text-zinc-400 leading-relaxed">
            Break complex logic into pure sub-routines (e.g. <code className="text-zinc-200">analyze_sql_risk</code>, <code className="text-zinc-200">calculate_blast_radius</code>) declared in separate code blocks adjacent to their corresponding semantic documentation.
          </p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <h3 className="font-bold text-white font-mono flex items-center gap-2">
            <Flame className="h-4 w-4 text-amber-400" />
            <span>Fuel Budget Checks</span>
          </h3>
          <p className="text-zinc-400 leading-relaxed">
            Always query <code className="text-zinc-200">ctx.fuel.remaining()</code> prior to traversing large ASTs or running string parsing loops. If gas is low, degrade gracefully rather than allowing a hard termination fault.
          </p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <h3 className="font-bold text-white font-mono flex items-center gap-2">
            <ShieldCheck className="h-4 w-4 text-emerald-400" />
            <span>Cryptographic Attestation</span>
          </h3>
          <p className="text-zinc-400 leading-relaxed">
            Generate SHA-256 audit certificates using <code className="text-zinc-200">ctx.crypto.sha256()</code> over the input parameters and decision payload. This provides an immutable paper trail for compliance audits (SOC2, ISO 27001).
          </p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <h3 className="font-bold text-white font-mono flex items-center gap-2">
            <GitBranch className="h-4 w-4 text-purple-400" />
            <span>Strict Fallback Action</span>
          </h3>
          <p className="text-zinc-400 leading-relaxed">
            Always return structured error dictionaries with <code className="text-zinc-200">&quot;approved&quot;: False</code> rather than relying on unhandled exceptions, so the agent can interpret the failure and self-correct.
          </p>
        </div>
      </div>
    </DocsSection>
  );
};
