import React from "react";

export const AxiomsListSection: React.FC = () => {
  const axioms = [
    {
      num: 1,
      title: "File Atomicity",
      desc: "Each skill, tool, or ASL specification exists as a single self-contained atomic file. Companion scripts, auxiliary virtual environments, or unversioned subfolders are strictly prohibited.",
    },
    {
      num: 2,
      title: "Zero External Runtime Dependencies",
      desc: "The core runtime (libasl and asl-cli) is written in pure Rust. Execution requires zero system interpreters—no Python, pip, Node.js, npm, Ruby, or JVM required on the host.",
    },
    {
      num: 3,
      title: "Object-Capability (OCap) Confinement",
      desc: "Zero Ambient Authority. Skills receive explicitly passed capability handles (ctx.fs, ctx.crypto, ctx.env). Symlink traversal escapes and ungranted sockets are blocked by construction.",
    },
    {
      num: 4,
      title: "Hexagonal Decoupling",
      desc: "Pure domain traits in asl-spec and asl-core-traits. Adapters (Starlark, Wasm, MCP, HTTP) never import each other, ensuring modular swapability and zero cyclic dependencies.",
    },
    {
      num: 5,
      title: "Deterministic Termination via Fuel",
      desc: "Execution is bounded by a monotonically decreasing fuel counter. Infinite loops, recursive explosions, and adversarial regex stalls terminate safely before exhausting host resources.",
    },
    {
      num: 6,
      id: "axiom-6",
      title: "Static Invariant Prefix & 100% KV-Cache",
      desc: "The semantic frontmatter and prompt instructions maintain byte-for-byte canonical ordering. Hash digests guarantee 100% KV-Cache hit rate across GPU inference clusters.",
    },
    {
      num: 7,
      title: "Cognitive Context Window Constraint (< 450 lines)",
      desc: "All code modules, specification files, and architectural ADRs are strictly bounded to fewer than 450 lines, ensuring optimal comprehension for both human architects and autonomous agent context windows.",
    },
  ];

  return (
    <div className="space-y-4">
      <p>
        Every release, crate, and feature in ASL is formally verified against the <strong>Seven Axioms</strong>. Any proposed pull request or runtime modification that violates an axiom is automatically rejected by the automated CI guardrails (<code className="text-zinc-200">./scripts/guardrail_check.sh</code>).
      </p>

      <div className="space-y-4 my-6">
        {axioms.map((ax) => (
          <div
            key={ax.num}
            id={ax.id}
            className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2 glow-card"
          >
            <div className="flex items-center gap-3">
              <div className="flex h-6 w-6 items-center justify-center rounded bg-zinc-800 border border-zinc-700 font-mono text-xs font-bold text-white">
                {ax.num}
              </div>
              <h3 className="text-sm font-bold text-white tracking-tight">
                Axiom {ax.num}: {ax.title}
              </h3>
            </div>
            <p className="text-xs text-zinc-400 pl-9 leading-relaxed">
              {ax.desc}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
};
