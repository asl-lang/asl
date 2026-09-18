import React from "react";
import { CheckCircle2 } from "lucide-react";

export const CoreTenetsSection: React.FC = () => {
  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        Core Tenets &amp; Guarantees
      </h2>
      <div className="space-y-3">
        <div className="flex items-start gap-3">
          <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
          <div>
            <strong className="text-white">Atomic Single-File Distribution:</strong> Each skill lives in a single atomic file (<code className="text-zinc-200 font-mono">.skill</code>, <code className="text-zinc-200 font-mono">.tool</code>, <code className="text-zinc-200 font-mono">.asl</code>). No subdirectories, loose helper scripts, or companion virtual environments.
          </div>
        </div>
        <div className="flex items-start gap-3">
          <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
          <div>
            <strong className="text-white">Pure Rust Runtime:</strong> Zero external interpreter requirements (no Python, Node.js, or JVM needed on the host system).
          </div>
        </div>
        <div className="flex items-start gap-3">
          <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
          <div>
            <strong className="text-white">Guaranteed Termination via Fuel:</strong> Monotonically decreasing opcode counters prevent infinite loops or Denial-of-Service stalls.
          </div>
        </div>
        <div className="flex items-start gap-3">
          <CheckCircle2 className="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
          <div>
            <strong className="text-white">93.2% Token Reduction:</strong> AOT schema validation eliminates ReAct exploratory turns and stochastic JSON repair retries.
          </div>
        </div>
      </div>
    </div>
  );
};
