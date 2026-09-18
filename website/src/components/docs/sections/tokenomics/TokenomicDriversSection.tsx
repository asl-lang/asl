import React from "react";

export const TokenomicDriversSection: React.FC = () => {
  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        4. The Three Drivers of Tokenomic Efficiency
      </h2>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <div className="text-xs font-bold text-blue-400 font-mono">1. AOT Contract Binding</div>
          <p className="text-xs text-zinc-400">
            Grammar masks (GBNF/Regex) eliminate stochastic format errors before decoding finishes. Re-prompt rate drops to 0.0%.
          </p>
        </div>
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <div className="text-xs font-bold text-emerald-400 font-mono">2. Zero Terminal Loops</div>
          <p className="text-xs text-zinc-400">
            The agent never runs exploratory subshell commands (<code className="text-zinc-200">ls</code>, <code className="text-zinc-200">grep</code>, <code className="text-zinc-200">which python</code>) to locate runtime binaries.
          </p>
        </div>
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <div className="text-xs font-bold text-purple-400 font-mono">3. 100% KV-Cache Invariance</div>
          <p className="text-xs text-zinc-400">
            Axiom 6 guarantees canonical serialization with frozen prefix hashes, ensuring 100% prompt cache reuse on GPU clusters.
          </p>
        </div>
      </div>
    </div>
  );
};
