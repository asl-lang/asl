import React from "react";

export const CliCommandsSection: React.FC = () => {
  const commands = [
    {
      cmd: "asl run <file> [--input '<json>']",
      desc: "Executes the deterministic entrypoint of a .skill, .tool, or .asl file with fuel metering and capability sandbox.",
      example: `asl run examples/git-conventional-commit.skill --input '{"intent": "fix: bug"}'`,
    },
    {
      cmd: "asl check <file>",
      desc: "Validates syntax, verifies canonical SHA256 digest, checks YAML frontmatter schema, and verifies OCap boundaries.",
      example: `asl check examples/security-validator.tool`,
    },
    {
      cmd: "asl expand <file>",
      desc: "Displays the in-memory compiled hermetic Starlark code generated from ASL blocks for developer inspection.",
      example: `asl expand examples/git-conventional-commit.skill`,
    },
    {
      cmd: "asl serve [--transport stdio|sse] [--port 8080] [--dir ./skills]",
      desc: "Starts a native Model Context Protocol (MCP) server exposing loaded skills as tools over stdio or HTTP/Server-Sent Events.",
      example: `asl serve --transport stdio --dir ./examples`,
    },
    {
      cmd: "asl analyze-prefix <file>",
      desc: "Calculates static invariant prefix length, estimates token footprint, and predicts inference KV-cache hit rate.",
      example: `asl analyze-prefix examples/summarizer.asl`,
    },
    {
      cmd: "asl sign <file> --key <priv_key>",
      desc: "Signs the canonical skill digest using Ed25519 private key, inserting cryptographic custody signature into the frontmatter.",
      example: `asl sign my-skill.skill --key ./author.ed25519`,
    },
  ];

  return (
    <div className="space-y-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        Subcommands &amp; Flags
      </h2>

      <div className="space-y-4 my-4">
        {commands.map((c, i) => (
          <div key={i} className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
            <div className="font-mono text-xs font-bold text-emerald-400">
              {c.cmd}
            </div>
            <p className="text-xs text-zinc-400">
              {c.desc}
            </p>
            <div className="rounded-lg border border-zinc-850 bg-black p-2.5 font-mono text-[11px] text-zinc-300">
              <code>$ {c.example}</code>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
