import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const FileAnatomySection: React.FC = () => {
  const anatomyCode = `---
# REGION 1: Frontmatter Manifest (Strict YAML)
asl_version: "3.0"
name: "git-commit-helper"
version: "1.0.0"
description: "Generates semantic Conventional Commits from git status"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "execute"
  input_schema:
    type: "object"
    required: ["intent"]
    properties:
      intent: { type: "string" }
capabilities:
  fs:
    confined_read_roots: ["./"]
limits:
  max_fuel_opcodes: 100000
---

# REGION 2: Semantic Prompt Envelope (CommonMark Markdown)
## 1. Intent
Generates conventional commit messages based on user intent and workspace diffs.

## 2. Decision Protocol
Invoke this skill when the user asks to summarize staged changes or prepare a commit.

---

\`\`\`asl:deterministic
# REGION 3: ASL Deterministic Block (asl:deterministic or asl:rules)
def execute(ctx, input):
    intent = input.get("intent", "").strip()
    if not intent:
        return {"valid": False, "error": "Intent is mandatory."}
    return {
        "valid": True,
        "formatted": "chore: " + intent,
        "fuel_used": ctx.fuel.consumed()
    }
\`\`\``;

  return (
    <DocsSection
      title="1. The Tri-Partite File Anatomy"
      subtitle="How an ASL skill file unites machine metadata, neural prompts, and deterministic code."
    >
      <AslCodeBlock lang="asl" code={anatomyCode} />

      <div className="grid grid-cols-1 md:grid-cols-3 gap-3 pt-2 text-xs">
        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
          <span className="font-semibold text-white font-mono">1. Frontmatter Manifest</span>
          <p className="text-zinc-400">Strict YAML parsed into <code className="text-zinc-300 font-mono">SkillManifest</code>. Validates JSON schemas, capabilities, and fuel limits ahead of execution.</p>
        </div>
        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
          <span className="font-semibold text-white font-mono">2. Semantic Envelope</span>
          <p className="text-zinc-400">Pure CommonMark consumed by the LLM reasoning loop. Kept byte-for-byte immutable across agent turns for 100% KV-cache invariance.</p>
        </div>
        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
          <span className="font-semibold text-white font-mono">3. Deterministic Block</span>
          <p className="text-zinc-400">Native ASL procedural code (<code className="text-zinc-300 font-mono">asl:deterministic</code>) or declarative rules (<code className="text-zinc-300 font-mono">asl:rules</code>) executed in the ASL sandboxed VM (compatible with Starlark L1).</p>
        </div>
      </div>
    </DocsSection>
  );
};
