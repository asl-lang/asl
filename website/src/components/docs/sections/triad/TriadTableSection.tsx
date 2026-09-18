import React from "react";

export const TriadTableSection: React.FC = () => {
  return (
    <div className="space-y-4">
      <p>
        In traditional agent architectures, developers are forced to choose between incompatible formats: prompt files (<code className="text-zinc-200">.prompt.md</code>), agent descriptions (<code className="text-zinc-200">.agent</code>), or Python scripts. ASL replaces this fragmentation with the <strong>Canonical Triad</strong>:
      </p>

      <div className="rounded-xl border border-zinc-800 bg-zinc-950 overflow-hidden font-mono text-xs">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-zinc-800 bg-zinc-900/60 text-zinc-400">
              <th className="p-3">Extension</th>
              <th className="p-3">Role & Purpose</th>
              <th className="p-3">Shadow Projection (.md)</th>
              <th className="p-3">Primary Engine</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-850 text-zinc-300">
            <tr>
              <td className="p-3 font-bold text-blue-400">.skill</td>
              <td className="p-3">Autonomous agent capability with semantic instructions & rules</td>
              <td className="p-3 text-emerald-400 font-semibold">✅ Enabled (.skill.md)</td>
              <td className="p-3 text-zinc-400">asl-vm-starlark</td>
            </tr>
            <tr>
              <td className="p-3 font-bold text-emerald-400">.tool</td>
              <td className="p-3">Direct deterministic tool call (MCP stdio & SSE compatible)</td>
              <td className="p-3 text-zinc-500">❌ Clean (Atomic File)</td>
              <td className="p-3 text-zinc-400">asl-vm-starlark / C-ABI</td>
            </tr>
            <tr>
              <td className="p-3 font-bold text-purple-400">.asl</td>
              <td className="p-3">Native Agent Skill Language full specification unit</td>
              <td className="p-3 text-zinc-500">❌ Clean (Atomic File)</td>
              <td className="p-3 text-zinc-400">asl-core / Wasm</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );
};
