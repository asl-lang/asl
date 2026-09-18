"use client";

import React, { useState } from "react";
import { Copy, Check, Terminal, FileCode, Shield, Layers } from "lucide-react";

interface CodeExample {
  extension: "skill" | "tool" | "asl";
  filename: string;
  badge: string;
  badgeColor: string;
  shadowNote: string;
  code: string;
}

const EXAMPLES: Record<string, CodeExample> = {
  skill: {
    extension: "skill",
    filename: "conventional-commit.skill",
    badge: "Autonomous Skill",
    badgeColor: "bg-zinc-900 text-zinc-300 border-zinc-800",
    shadowNote: "✨ Automatic Shadow Projection: Projects conventional-commit.skill.md for human / legacy inspection",
    code: `---
asl_version: "3.0"
name: "git-conventional-commit"
description: "Generates atomic, standardized conventional commits"
interface:
  entrypoint: "validate_and_format"
  input_schema:
    type: "object"
    properties:
      intent: { type: "string" }
      diff_stat: { type: "string" }
    required: ["intent"]
capabilities:
  fs:
    confined_read_roots: ["."]
limits:
  max_fuel_opcodes: 500000
---
# Semantic Prompt Instructions
Evaluate git staged status and generate compliant conventional commit.

\`\`\`asl:rules
rule "infer_type_from_intent":
  when:
    input.intent matches "(?i)^(corrigir|fix|bug|patch)"
  then:
    set commit_type = "fix"

rule "require_non_empty_intent":
  when:
    input.intent == ""
  then:
    set is_valid = false
    set error = "Commit intent cannot be empty"
\`\`\`
`,
  },
  tool: {
    extension: "tool",
    filename: "security-validator.tool",
    badge: "Direct MCP Tool",
    badgeColor: "bg-zinc-900 text-zinc-300 border-zinc-800",
    shadowNote: "🛡️ Atomic Tool File: Zero shadow markdown generated. Clean single-file MCP contract.",
    code: `---
asl_version: "3.0"
name: "security-validator"
description: "Confined AST sanitizer preventing prompt injections"
interface:
  entrypoint: "validate_prompt"
  input_schema:
    type: "object"
    properties:
      prompt_text: { type: "string" }
    required: ["prompt_text"]
capabilities:
  fs:
    confined_read_roots: []
limits:
  max_fuel_opcodes: 100000
---
# Instructions
Validate prompt safety boundaries before invoking external subagents.

\`\`\`asl:rules
rule "block_eval_injection":
  when:
    input.prompt_text matches "(?i)(ignore previous instructions|system:)"
  then:
    set is_safe = false
    set flag = "PROMPT_INJECTION_DETECTED"
\`\`\`
`,
  },
  asl: {
    extension: "asl",
    filename: "summarizer.asl",
    badge: "Native ASL Specification",
    badgeColor: "bg-zinc-900 text-zinc-300 border-zinc-800",
    shadowNote: "⚡ Pure ASL Unit: Zero shadow markdown. Evaluated at high throughput by the Rust VM.",
    code: `---
asl_version: "3.0"
name: "text-summarizer"
description: "Token-optimal AST text summarization engine"
interface:
  entrypoint: "format_prompt"
  input_schema:
    type: "object"
    properties:
      content: { type: "string" }
      max_sentences: { type: "integer" }
    required: ["content"]
capabilities:
  fs:
    confined_read_roots: []
limits:
  max_fuel_opcodes: 250000
---
# Semantic Prompt
Extract high-saliency declarative tokens preserving causal intent.

\`\`\`asl:deterministic
def format_prompt(ctx, input):
    raw_content = input.get("content", "")
    max_sentences = input.get("max_sentences", 3)
    
    if len(raw_content) == 0:
        return {"error": "Content cannot be empty", "summary": ""}
        
    return {
        "status": "ready",
        "processed_length": len(raw_content),
        "limit": max_sentences
    }
\`\`\`
`,
  },
};

export const CodeSwitcher: React.FC = () => {
  const [activeTab, setActiveTab] = useState<"skill" | "tool" | "asl">("skill");
  const [copied, setCopied] = useState(false);

  const active = EXAMPLES[activeTab];

  const handleCopy = () => {
    navigator.clipboard.writeText(active.code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="w-full rounded-xl border border-zinc-800 bg-zinc-950/80 shadow-2xl overflow-hidden">
      {/* Tab bar */}
      <div className="flex flex-wrap items-center justify-between border-b border-zinc-800 bg-zinc-900/60 px-4 py-2.5 gap-2">
        <div className="flex items-center gap-1.5">
          <button
            onClick={() => setActiveTab("skill")}
            className={`flex items-center gap-2 rounded-lg px-3 py-1.5 text-xs font-mono font-medium transition-all ${
              activeTab === "skill"
                ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                : "text-zinc-400 hover:bg-zinc-800/40 hover:text-zinc-200"
            }`}
          >
            <span className="h-1.5 w-1.5 rounded-full bg-zinc-400" />
            .skill
          </button>
          <button
            onClick={() => setActiveTab("tool")}
            className={`flex items-center gap-2 rounded-lg px-3 py-1.5 text-xs font-mono font-medium transition-all ${
              activeTab === "tool"
                ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                : "text-zinc-400 hover:bg-zinc-800/40 hover:text-zinc-200"
            }`}
          >
            <span className="h-1.5 w-1.5 rounded-full bg-zinc-400" />
            .tool
          </button>
          <button
            onClick={() => setActiveTab("asl")}
            className={`flex items-center gap-2 rounded-lg px-3 py-1.5 text-xs font-mono font-medium transition-all ${
              activeTab === "asl"
                ? "bg-zinc-800 text-white shadow-sm border border-zinc-700"
                : "text-zinc-400 hover:bg-zinc-800/40 hover:text-zinc-200"
            }`}
          >
            <span className="h-1.5 w-1.5 rounded-full bg-zinc-400" />
            .asl
          </button>
        </div>

        <div className="flex items-center gap-2.5">
          <span className={`text-[10px] sm:text-[11px] font-mono px-2 py-0.5 rounded border ${active.badgeColor}`}>
            {active.badge}
          </span>
          <button
            onClick={handleCopy}
            className="flex items-center gap-1.5 rounded border border-zinc-700/60 bg-zinc-800/70 px-2.5 py-1 text-[11px] text-zinc-300 hover:bg-zinc-700 hover:text-white transition-colors"
          >
            {copied ? <Check className="h-3 w-3 text-white" /> : <Copy className="h-3 w-3 text-zinc-400" />}
            <span>{copied ? "Copied!" : "Copy"}</span>
          </button>
        </div>
      </div>

      {/* Shadow Projection status indicator */}
      <div className="border-b border-zinc-800 bg-zinc-900/30 px-4 py-2 text-[11px] font-mono text-zinc-400 flex flex-col sm:flex-row sm:items-center justify-between gap-1">
        <span className="text-zinc-300 font-semibold">{active.filename}</span>
        <span className="text-[10px] sm:text-[11px] text-zinc-400">{active.shadowNote}</span>
      </div>

      {/* Code body */}
      <div className="relative overflow-x-auto p-4 font-mono text-xs leading-relaxed text-zinc-300">
        <pre className="selection:bg-zinc-700 selection:text-white">
          <code>{active.code}</code>
        </pre>
      </div>
    </div>
  );
};
