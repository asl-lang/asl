import React from "react";
import Link from "next/link";
import { ArrowRight, Terminal, Server, Shield, Sparkles, Check } from "lucide-react";

export default function CliDocsPage() {
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
      desc: "Displays the in-memory transpiled Starlark L1 code generated from asl:rules blocks for developer inspection.",
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
    <div className="space-y-8">
      <div>
        <div className="text-xs font-mono font-medium text-blue-400 uppercase tracking-wider">
          Tooling • Track 5
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          CLI & MCP Server Reference
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed">
          Complete reference for the pure-Rust <code className="text-zinc-200 font-mono">asl</code> binary, subcommands, and Model Context Protocol server integration.
        </p>
      </div>

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <h2 className="text-xl font-bold text-white tracking-tight">
          Subcommands & Flags
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

        <h2 className="text-xl font-bold text-white tracking-tight pt-4">
          Model Context Protocol (MCP) Configuration
        </h2>
        <p>
          You can expose your ASL skills directly to Claude Desktop, Cursor, or Google Antigravity by registering <code className="text-zinc-200 font-mono">asl serve</code> in your MCP client configuration:
        </p>

        <div className="rounded-xl border border-zinc-800 bg-black p-4 font-mono text-xs text-zinc-300 leading-relaxed">
          <pre>
{`{
  "mcpServers": {
    "asl": {
      "command": "asl",
      "args": ["serve", "--transport", "stdio", "--dir", "/path/to/my-skills"]
    }
  }
}`}
          </pre>
        </div>

        <div className="pt-6 flex items-center justify-between border-t border-zinc-850">
          <Link
            href="/docs/axioms"
            className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200"
          >
            <span>← The 7 Axioms & Security</span>
          </Link>
          <Link
            href="/paper"
            className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300"
          >
            <span>The Formal Scientific Paper 🔬</span>
            <ArrowRight className="h-3.5 w-3.5" />
          </Link>
        </div>
      </div>
    </div>
  );
}
