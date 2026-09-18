import React from "react";

export const ShadowProjectionSection: React.FC = () => {
  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        Shadow Markdown Projection: Architectural Rationale
      </h2>
      <p>
        One of the core design achievements in ADR-0009 and ADR-0011 is <strong>Selective Shadow Projection</strong>:
      </p>
      <ul className="list-disc pl-5 space-y-2 text-zinc-400">
        <li>
          <strong className="text-zinc-200">Why only .skill projects a shadow:</strong> Skills frequently interface with human developers in IDEs, GitHub web readers, and non-ASL LLM harness tools. The projection generates an isolated <code className="text-zinc-300 font-mono">&lt;file&gt;.skill.md</code> reflecting the human-readable Markdown view with cryptographic synchronization.
        </li>
        <li>
          <strong className="text-zinc-200">Why .tool and .asl remain clean:</strong> Direct tools and language units are machine-evaluated. Generating companion markdown files for every single tool would clutter developer workspaces, increase token overhead during directory scans, and violate Axiom 1 (Strict File Atomicity).
        </li>
      </ul>
    </div>
  );
};
