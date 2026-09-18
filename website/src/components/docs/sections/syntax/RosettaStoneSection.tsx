"use client";

import React, { useState } from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";
import { Terminal, Code, Cpu, BookOpen } from "lucide-react";

export const RosettaStoneSection: React.FC = () => {
  const [activeTab, setActiveTab] = useState<"bash" | "python" | "starlark">("bash");

  const bashAslExample = `---
asl_version: "3.0"
name: "ping-service"
version: "1.0.0"
description: "High-speed command router written in pure ASL"
interface:
  entrypoint: "route_command"
---
# Ping Service
Routes operational commands with zero shell injection risk.

\`\`\`asl
# Pure ASL replaces fragile Bash scripts (curl, jq, grep)
match input.cmd:
  when "ping":
    return {"reply": "pong", "status": "healthy"}
  when "status":
    return {"reply": "active", "fuel": ctx.fuel.consumed()}
  otherwise:
    return {"error": "unknown command"}
\`\`\``;

  const pythonAslExample = `---
asl_version: "3.0"
name: "crypto-auditor"
version: "1.0.0"
description: "Cryptographic hash validator in pure ASL"
interface:
  entrypoint: "run"
---
# Crypto Auditor
Audit document hashes deterministically with fuel protection.

\`\`\`asl
# Pure ASL replaces Python sys/hashlib scripts
def run(ctx, input):
    text = input.get("text", "")
    if not text:
        return {"error": "text is required"}
    
    # Capability-confined host call
    digest = ctx.crypto.sha256(text)
    
    return {
        "hash": digest,
        "length": len(text),
        "fuel_used": ctx.fuel.consumed()
    }
\`\`\``;

  const starlarkUnderTheHoodExample = `# --- UNDER THE HOOD: COMPILED TO HERMETIC STARLARK (IN-MEMORY AOT) ---
# Source: examples/git-conventional-commit.skill
# Runtime Engine: Starlark L1 Bytecode / RAM Execution

def _asl_eval_rule_0(ctx, input):
    _v_intent = input.get("intent", "")
    if _v_intent == "":
        return {"is_valid": False, "error": "Commit intent cannot be empty"}
    return None

def _asl_eval_rule_1(ctx, input):
    _v_intent = input.get("intent", "")
    if _v_intent.startswith("fix") or _v_intent.startswith("bug"):
        return {"commit_type": "fix", "is_valid": True}
    return None

def run(ctx, input):
    _res = _asl_eval_rule_0(ctx, input)
    if _res != None:
        return _res
    _res = _asl_eval_rule_1(ctx, input)
    if _res != None:
        return _res
    return {"commit_type": "chore", "is_valid": True}`;

  return (
    <DocsSection
      title="3. Canonical ASL Rosetta Stone"
      subtitle="How to do anything in pure ASL (\`\`\`asl) — migrating from Bash, Python, and understanding Starlark under the hood."
    >
      <div className="space-y-4">
        {/* Philosophy Callout */}
        <div className="rounded-xl border border-blue-900/40 bg-blue-950/20 p-4 text-xs text-blue-200 leading-relaxed flex items-start gap-3">
          <BookOpen className="h-5 w-5 text-blue-400 shrink-0 mt-0.5" />
          <div>
            <span className="font-semibold text-white">The Core Philosophy: &ldquo;Programar igual escrevendo um Markdown&rdquo;</span>
            <p className="text-zinc-300 mt-1">
              In ASL, the document <em>is</em> the program. Neural instructions are plain CommonMark prose for humans and AI agents, while deterministic logic lives in concise <code className="text-zinc-200 font-mono">\`\`\`asl</code> code blocks. There are no separate languages or disjoint tags—only one unified, token-efficient ASL language.
            </p>
          </div>
        </div>

        {/* Tab Selection */}
        <div className="flex border-b border-zinc-800 bg-zinc-900/40 rounded-t-xl overflow-hidden font-mono text-xs">
          <button
            onClick={() => setActiveTab("bash")}
            className={`flex items-center gap-2 px-4 py-2.5 transition-colors ${
              activeTab === "bash"
                ? "bg-zinc-800 text-white font-semibold border-b-2 border-emerald-400"
                : "text-zinc-400 hover:text-zinc-200 hover:bg-zinc-850"
            }`}
          >
            <Terminal className="h-3.5 w-3.5 text-emerald-400" />
            <span>Bash to ASL</span>
          </button>
          <button
            onClick={() => setActiveTab("python")}
            className={`flex items-center gap-2 px-4 py-2.5 transition-colors ${
              activeTab === "python"
                ? "bg-zinc-800 text-white font-semibold border-b-2 border-blue-400"
                : "text-zinc-400 hover:text-zinc-200 hover:bg-zinc-850"
            }`}
          >
            <Code className="h-3.5 w-3.5 text-blue-400" />
            <span>Python to ASL</span>
          </button>
          <button
            onClick={() => setActiveTab("starlark")}
            className={`flex items-center gap-2 px-4 py-2.5 transition-colors ${
              activeTab === "starlark"
                ? "bg-zinc-800 text-white font-semibold border-b-2 border-purple-400"
                : "text-zinc-400 hover:text-zinc-200 hover:bg-zinc-850"
            }`}
          >
            <Cpu className="h-3.5 w-3.5 text-purple-400" />
            <span>Starlark Under the Hood</span>
          </button>
        </div>

        {/* Tab Content: Bash */}
        {activeTab === "bash" && (
          <div className="space-y-4">
            <div className="rounded-xl border border-zinc-800 bg-zinc-950 overflow-hidden font-mono text-xs">
              <table className="w-full text-left border-collapse">
                <thead>
                  <tr className="border-b border-zinc-800 bg-zinc-900/60 text-zinc-400">
                    <th className="p-3">Bash Operation</th>
                    <th className="p-3">Pure ASL Equivalent (<code className="text-zinc-200">\`\`\`asl</code>)</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-zinc-850 text-zinc-300">
                  <tr>
                    <td className="p-3 text-rose-300">cat file.txt</td>
                    <td className="p-3 text-emerald-300">content = ctx.fs.read(&quot;file.txt&quot;)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">echo &quot;$data&quot; &gt; file.txt</td>
                    <td className="p-3 text-emerald-300">ctx.fs.write(&quot;file.txt&quot;, data)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">curl -s $URL</td>
                    <td className="p-3 text-emerald-300">resp = ctx.http.get(url)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">curl -d &quot;$b&quot; $URL</td>
                    <td className="p-3 text-emerald-300">resp = ctx.http.post(url, json=payload)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">if [ &quot;$x&quot; = &quot;ok&quot; ]</td>
                    <td className="p-3 text-emerald-300">match input.x: when &quot;ok&quot;: return &#123;&quot;ok&quot;: True&#125;</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">echo $s | grep needle</td>
                    <td className="p-3 text-emerald-300">&quot;needle&quot; in text</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">$MY_VAR</td>
                    <td className="p-3 text-emerald-300">val = ctx.env.get(&quot;MY_VAR&quot;)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">exit 1</td>
                    <td className="p-3 text-emerald-300">reject(&quot;Operation failed&quot;) or return &#123;&quot;error&quot;: &quot;...&quot;&#125;</td>
                  </tr>
                </tbody>
              </table>
            </div>

            <AslCodeBlock lang="asl" filename="ping-service.skill" code={bashAslExample} />
          </div>
        )}

        {/* Tab Content: Python */}
        {activeTab === "python" && (
          <div className="space-y-4">
            <div className="rounded-xl border border-zinc-800 bg-zinc-950 overflow-hidden font-mono text-xs">
              <table className="w-full text-left border-collapse">
                <thead>
                  <tr className="border-b border-zinc-800 bg-zinc-900/60 text-zinc-400">
                    <th className="p-3">Python Pattern</th>
                    <th className="p-3">Pure ASL Equivalent (<code className="text-zinc-200">\`\`\`asl</code>)</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-zinc-850 text-zinc-300">
                  <tr>
                    <td className="p-3 text-rose-300">import json; json.loads(s)</td>
                    <td className="p-3 text-emerald-300">data = json.decode(s) (built-in linear parser)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">import json; json.dumps(o)</td>
                    <td className="p-3 text-emerald-300">s = json.encode(o)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">import hashlib; sha256()</td>
                    <td className="p-3 text-emerald-300">digest = ctx.crypto.sha256(data)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">import os; os.environ[&quot;K&quot;]</td>
                    <td className="p-3 text-emerald-300">val = ctx.env.get(&quot;K&quot;)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">dict.get(key, default)</td>
                    <td className="p-3 text-emerald-300">input.get(key, default)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">[x * 2 for x in items]</td>
                    <td className="p-3 text-emerald-300">[x * 2 for x in items] (deterministic comprehensions)</td>
                  </tr>
                  <tr>
                    <td className="p-3 text-rose-300">def main(): sys.exit(0)</td>
                    <td className="p-3 text-emerald-300">def run(ctx, input): return &#123;&quot;status&quot;: &quot;ok&quot;&#125;</td>
                  </tr>
                </tbody>
              </table>
            </div>

            <AslCodeBlock lang="asl" filename="crypto-auditor.skill" code={pythonAslExample} />
          </div>
        )}

        {/* Tab Content: Starlark Under the Hood */}
        {activeTab === "starlark" && (
          <div className="space-y-4">
            <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2 text-xs text-zinc-300 leading-relaxed">
              <h4 className="font-semibold text-white font-mono">Writing in ASL vs. Execution in Starlark</h4>
              <p>
                When authoring skills, tools, or specifications, you write <strong>strictly in ASL</strong> using the single universal code tag <code className="text-zinc-200 font-mono">\`\`\`asl</code>.
              </p>
              <p>
                Under the hood, the ASL compiler transpiles your code Ahead-of-Time (AOT) directly in-memory into hermetic Starlark. The Starlark engine guarantees:
              </p>
              <ul className="list-disc pl-5 space-y-1 text-zinc-400">
                <li><strong className="text-zinc-200">Physical Fuel Metering:</strong> Every opcode decrements monotonic fuel counter. Infinite loops are halted.</li>
                <li><strong className="text-zinc-200">Capability Confinement:</strong> All host interactions pass through the explicit <code className="text-zinc-300 font-mono">ctx</code> struct.</li>
                <li><strong className="text-zinc-200">Zero Ambient Authority:</strong> No random numbers, system clocks, or network calls without explicit capability grants.</li>
              </ul>
              <p className="text-zinc-400">
                Developers can run <code className="text-zinc-200 font-mono">asl expand &lt;file&gt;</code> at any time to inspect the exact Starlark code generated under the hood.
              </p>
            </div>

            <AslCodeBlock lang="asl" filename="git-conventional-commit.expanded.star" code={starlarkUnderTheHoodExample} />
          </div>
        )}
      </div>
    </DocsSection>
  );
};
