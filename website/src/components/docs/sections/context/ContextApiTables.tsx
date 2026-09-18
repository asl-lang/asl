import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const ContextApiTables: React.FC = () => {
  return (
    <DocsSection
      title="2. Capability Context API Specification"
      subtitle="Detailed methods available on the ctx struct parameter in ASL."
    >
      {/* Filesystem */}
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-base font-bold font-mono text-white">ctx.fs</span>
            <span className="text-[10px] font-mono rounded bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 text-amber-400">
              Requires capabilities.fs
            </span>
          </div>
        </div>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Provides sandboxed filesystem operations strictly bounded by <code className="text-zinc-200 font-mono">confined_read_roots</code>.
        </p>

        <div className="overflow-x-auto rounded-lg border border-zinc-850">
          <table className="w-full text-left text-xs font-mono">
            <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
              <tr>
                <th className="p-2.5">Method</th>
                <th className="p-2.5">Returns</th>
                <th className="p-2.5 font-sans">Description</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-850 text-zinc-300">
              <tr>
                <td className="p-2.5 text-blue-400">ctx.fs.read(path: string)</td>
                <td className="p-2.5 text-zinc-400">string | None</td>
                <td className="p-2.5 font-sans">Reads text content. Returns <code className="text-zinc-200">None</code> if file is missing. Raises capability error if path escapes root.</td>
              </tr>
              <tr>
                <td className="p-2.5 text-blue-400">ctx.fs.exists(path: string)</td>
                <td className="p-2.5 text-zinc-400">bool</td>
                <td className="p-2.5 font-sans">Returns boolean indicating whether file exists within whitelisted roots.</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      {/* Crypto */}
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-base font-bold font-mono text-white">ctx.crypto</span>
            <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
              Pure Deterministic (Zero Grant Needed)
            </span>
          </div>
        </div>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Cryptographic primitives implemented in native Rust for high performance and strict mathematical reproducibility.
        </p>

        <div className="overflow-x-auto rounded-lg border border-zinc-850">
          <table className="w-full text-left text-xs font-mono">
            <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
              <tr>
                <th className="p-2.5">Method</th>
                <th className="p-2.5">Returns</th>
                <th className="p-2.5 font-sans">Description</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-850 text-zinc-300">
              <tr>
                <td className="p-2.5 text-blue-400">ctx.crypto.sha256(data: string)</td>
                <td className="p-2.5 text-zinc-400">string</td>
                <td className="p-2.5 font-sans">Computes 64-character lowercase hexadecimal SHA-256 checksum.</td>
              </tr>
              <tr>
                <td className="p-2.5 text-blue-400">ctx.crypto.verify_ed25519(msg, sig, pubkey)</td>
                <td className="p-2.5 text-zinc-400">bool</td>
                <td className="p-2.5 font-sans">Validates Ed25519 digital signature against public key.</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      {/* Fuel & Execution Metering */}
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-base font-bold font-mono text-white">ctx.fuel</span>
            <span className="text-[10px] font-mono rounded bg-purple-500/10 border border-purple-500/20 px-2 py-0.5 text-purple-400">
              Axiom 5 Metering
            </span>
          </div>
        </div>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Enables code to inspect its own resource consumption and gracefully terminate or downgrade workloads before quotas are exceeded.
        </p>

        <div className="overflow-x-auto rounded-lg border border-zinc-850">
          <table className="w-full text-left text-xs font-mono">
            <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
              <tr>
                <th className="p-2.5">Method</th>
                <th className="p-2.5">Returns</th>
                <th className="p-2.5 font-sans">Description</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-850 text-zinc-300">
              <tr>
                <td className="p-2.5 text-blue-400">ctx.fuel.consumed()</td>
                <td className="p-2.5 text-zinc-400">int</td>
                <td className="p-2.5 font-sans">Returns total opcode execution units consumed since invocation start.</td>
              </tr>
              <tr>
                <td className="p-2.5 text-blue-400">ctx.fuel.remaining()</td>
                <td className="p-2.5 text-zinc-400">int</td>
                <td className="p-2.5 font-sans">Returns remaining gas before execution deadline is forcibly triggered.</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </DocsSection>
  );
};
