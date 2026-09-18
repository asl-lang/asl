import React from "react";
import cliSpec from "@/data/cli-spec.json";

export const CliCommandsSection: React.FC = () => {
  const commands = cliSpec.commands || [];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-bold text-white tracking-tight">
          Subcommands &amp; Flags
        </h2>
        <span className="text-xs font-mono text-zinc-500 bg-zinc-900 border border-zinc-800 px-2.5 py-1 rounded-full">
          SSOT v{cliSpec.version} • {commands.length} Commands
        </span>
      </div>

      <p className="text-xs text-zinc-400">
        All subcommands are generated deterministically from the Spec-Driven Documentation (SDD) manifest in <code className="text-zinc-300">docs/spec/</code>.
      </p>

      <div className="space-y-4 my-4">
        {commands.map((c: any) => (
          <div key={c.id} className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-3">
            <div className="flex items-center justify-between">
              <div className="font-mono text-xs font-bold text-emerald-400">
                {c.ai_primer?.usage ? c.ai_primer.usage : `asl ${c.name}`}
              </div>
              <span className="text-[10px] uppercase font-mono px-2 py-0.5 rounded border border-zinc-700 bg-zinc-900 text-zinc-400">
                {c.category}
              </span>
            </div>

            <p className="text-xs text-zinc-300">
              {c.description}
            </p>

            {c.flags && c.flags.length > 0 && (
              <div className="space-y-1.5 pt-1">
                <span className="text-[11px] font-semibold text-zinc-400 uppercase tracking-wider">Flags:</span>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-1.5">
                  {c.flags.map((f: any) => (
                    <div key={f.long} className="text-[11px] font-mono bg-zinc-900/80 border border-zinc-850 p-1.5 rounded text-zinc-300 flex items-baseline gap-2">
                      <span className="text-cyan-400 font-semibold">
                        {f.short ? `-${f.short}, ` : ""}--{f.long}
                      </span>
                      <span className="text-zinc-500 text-[10px] truncate">{f.description}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {c.examples && c.examples.length > 0 && (
              <div className="space-y-1 pt-1">
                {c.examples.map((ex: any, idx: number) => (
                  <div key={idx} className="rounded-lg border border-zinc-850 bg-black p-2.5 font-mono text-[11px] text-zinc-300">
                    <div className="text-[10px] text-zinc-500 mb-1"># {ex.desc}</div>
                    <code>$ {ex.cmd}</code>
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};
