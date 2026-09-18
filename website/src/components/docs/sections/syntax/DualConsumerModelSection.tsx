import React from "react";
import { Sparkles, Shield } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const DualConsumerModelSection: React.FC = () => {
  return (
    <DocsSection
      title="2. The Dual-Consumer Execution Model"
      subtitle="Why ASL targets both neural reasoners and deterministic virtual machines simultaneously."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
        <div className="rounded-xl border border-blue-900/50 bg-blue-950/20 p-5 space-y-2">
          <div className="flex items-center gap-2 text-blue-400 font-bold font-mono">
            <Sparkles className="h-4 w-4" />
            <span>Consumer A: The Neural Reasoner (LLM)</span>
          </div>
          <p className="text-zinc-300 leading-relaxed">
            The LLM consumes Region 2 (CommonMark instructions). Because the prompt envelope contains natural language guidelines, decision protocols, and intent boundaries, the neural model understands <em>when</em> and <em>why</em> to invoke the skill, producing structured JSON parameters matching <code className="text-zinc-200 font-mono">input_schema</code>.
          </p>
        </div>

        <div className="rounded-xl border border-emerald-900/50 bg-emerald-950/20 p-5 space-y-2">
          <div className="flex items-center gap-2 text-emerald-400 font-bold font-mono">
            <Shield className="h-4 w-4" />
            <span>Consumer B: The Sandboxed ASL Host VM</span>
          </div>
          <p className="text-zinc-300 leading-relaxed">
            When the agent calls the skill, the host ASL engine validates arguments, creates an isolated memory sandbox, injects capability context handles (<code className="text-zinc-200 font-mono">ctx</code>), and executes Region 3 with guaranteed mathematical termination in finite fuel steps (100% compatible with the Starlark L1 runtime standard).
          </p>
        </div>
      </div>
    </DocsSection>
  );
};
