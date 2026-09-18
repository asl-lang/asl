import React from "react";
import { MathBlock } from "@/components/MathBlock";

export const DualConsumerAstSection: React.FC = () => {
  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        The ASL Solution: Dual-Consumer AST
      </h2>
      <p>
        Literate Programming (Knuth, 1984) united human prose with compiler code. ASL reinvents this for the AI era: an atomic skill unit is simultaneously consumed by two distinct entities:
      </p>

      <div className="rounded-xl border border-zinc-800 bg-black p-4 font-mono text-xs my-4 space-y-2">
        <div className="text-zinc-400">// Mathematical Definition of an ASL Unit:</div>
        <MathBlock block math={`\\mathcal{S} = \\langle \\mathcal{M}, \\mathcal{P}, \\mathcal{D} \\rangle`} />
        <ul className="list-disc pl-5 space-y-1 text-zinc-400">
          <li><span className="text-zinc-200 font-semibold">\mathcal&#123;M&#125; (Manifest)</span>: Immutable YAML frontmatter with capabilities, OCap bounds, and entrypoint signatures.</li>
          <li><span className="text-zinc-200 font-semibold">\mathcal&#123;P&#125; (Semantic Envelope)</span>: CommonMark prose parsed by the LLM (Consumer \alpha) for intent and context.</li>
          <li><span className="text-zinc-200 font-semibold">\mathcal&#123;D&#125; (Deterministic Block)</span>: Native ASL deterministic logic (evaluated by Consumer \beta, 100% compatible with Starlark L1 runtime).</li>
        </ul>
      </div>
    </div>
  );
};
