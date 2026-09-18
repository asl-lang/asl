import React from "react";
import { PlaygroundSimulator } from "@/components/PlaygroundSimulator";
import { Cpu, Sparkles, Shield, Layers } from "lucide-react";

export default function PlaygroundPage() {
  return (
    <div className="mx-auto max-w-6xl px-4 py-12 sm:px-6 lg:px-8 space-y-8">
      <div>
        <div className="text-xs font-mono font-medium text-purple-400 uppercase tracking-wider">
          Interactive Environment
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          ASL Interactive Playground & Simulator
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-2xl">
          Edit ASL documents in real-time. Experience semantic <code className="text-zinc-200">```asl</code> in-memory transpilation, deterministic execution, and tokenomics analysis directly in your browser.
        </p>
      </div>

      <PlaygroundSimulator />

      {/* Feature cards below simulator */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-5 pt-4">
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <div className="flex items-center gap-2 text-xs font-semibold text-blue-400 font-mono">
            <Cpu className="h-4 w-4" />
            In-Memory Transpiler
          </div>
          <p className="text-xs text-zinc-400 leading-relaxed">
            High-level declarative rules in pure <code className="text-zinc-300 font-mono">```asl</code> are compiled Ahead-of-Time to hermetic ASL VM deterministic functions without needing an external compiler.
          </p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <div className="flex items-center gap-2 text-xs font-semibold text-emerald-400 font-mono">
            <Sparkles className="h-4 w-4" />
            KV-Cache Prefix Metering
          </div>
          <p className="text-xs text-zinc-400 leading-relaxed">
            Every document maintains an invariant prefix hash (Axiom 6), ensuring zero KV-Cache invalidations across multi-tenant GPU inference clusters.
          </p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
          <div className="flex items-center gap-2 text-xs font-semibold text-purple-400 font-mono">
            <Shield className="h-4 w-4" />
            Bounded Fuel Execution
          </div>
          <p className="text-xs text-zinc-400 leading-relaxed">
            Every execution is guaranteed to terminate deterministically, preventing infinite loops or denial-of-service stalls.
          </p>
        </div>
      </div>
    </div>
  );
}
