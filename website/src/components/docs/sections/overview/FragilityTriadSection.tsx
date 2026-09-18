import React from "react";

export const FragilityTriadSection: React.FC = () => {
  return (
    <div className="space-y-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        The Problem: The Fragility Triad of Legacy Agents
      </h2>
      <p>
        Current AI agent frameworks (such as raw Claude Code tools, OpenAI Operator actions, or general-purpose MCP servers) execute unconfined scripts in Python, Bash, or Node.js. In production, this architecture suffers from three catastrophic failure modes:
      </p>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 my-6">
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <div className="text-xs font-bold text-red-400 font-mono">1. Ambient Authority</div>
          <p className="text-xs text-zinc-400">
            Unconfined scripts execute with full OS root/user privileges, allowing indirect prompt injections to execute remote code (RCE) or exfiltrate environment secrets.
          </p>
        </div>
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <div className="text-xs font-bold text-amber-400 font-mono">2. Environment Drift</div>
          <p className="text-xs text-zinc-400">
            Scripts depend on global interpreters, virtual environments, and package managers. Breakages force LLMs into 10+ turn repair loops wasting thousands of tokens.
          </p>
        </div>
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
          <div className="text-xs font-bold text-blue-400 font-mono">3. Token Bloat &amp; Cache Loss</div>
          <p className="text-xs text-zinc-400">
            Exploratory terminal loops (<code className="text-zinc-200">cat</code>, <code className="text-zinc-200">ls</code>, stderr tracebacks) consume ~2,100 tokens per action and destroy inference engine KV-cache prefixes.
          </p>
        </div>
      </div>
    </div>
  );
};
