import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const TranspilationPipelineSection: React.FC = () => {
  return (
    <DocsSection
      title="3. AOT In-Memory Transpilation Pipeline"
      subtitle="How the ASL compiler turns declarative rules into deterministic execution bytecode."
    >
      <p className="text-sm text-zinc-300 leading-relaxed">
        The <code className="text-zinc-200 font-mono">asl-parser</code> crate compiles rules blocks ahead-of-time directly into deterministic execution bytecode (fully compatible with the Starlark L1 runtime standard):
      </p>

      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2 text-xs text-zinc-400 font-mono">
        <p><strong className="text-white">1. Defensive Path Resolution:</strong> Generates nested <code className="text-zinc-300">_asl_get(input, [&quot;a&quot;, &quot;b&quot;])</code> calls, completely eliminating runtime <code className="text-rose-400">KeyError</code> panics.</p>
        <p><strong className="text-white">2. Pure Function Dispatch:</strong> Emits pure Pythonic branch trees with exact variable bindings (<code className="text-zinc-300">_asl_starts_with_any</code>) running in $O(N)$ bounded opcodes.</p>
        <p><strong className="text-white">3. Zero-Allocation Mapping:</strong> Rejection payloads and acceptance structs map directly into native JSON responses matching <code className="text-zinc-300 font-mono">output_schema</code>.</p>
      </div>
    </DocsSection>
  );
};
