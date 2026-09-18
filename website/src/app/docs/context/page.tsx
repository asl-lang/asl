import React from "react";
import Link from "next/link";
import { ArrowRight, Shield, Lock, FileCode, CheckCircle2, ShieldAlert, Cpu } from "lucide-react";

export default function CapabilityContextPage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Section 4
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Capability Context (<code className="text-blue-400 font-mono">ctx</code>)
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Complete reference for the Object Capability (OCap) host interface, sandboxed filesystem access, cryptographic primitives, and fuel monitoring.
        </p>
      </div>

      {/* 1. The OCap Security Model */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. Object Capability (OCap) Security Model</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          Standard programming languages (Python, Node.js, Bash) operate on an <strong>ambient authority</strong> model: any library, dependency, or LLM-generated script can unilaterally invoke system calls, read arbitrary files (<code className="text-zinc-200 font-mono">/etc/passwd</code>, <code className="text-zinc-200 font-mono">~/.ssh/id_rsa</code>), or open outbound sockets.
        </p>
        <p className="text-sm text-zinc-300 leading-relaxed">
          ASL implements Axiom 3 (Object Capability Security). The execution engine is completely air-gapped. Deterministic logic has <strong>zero ambient authority</strong>: it cannot import external modules, execute raw syscalls, or inspect host state. Every interaction with the outside world must flow through the explicit <code className="text-zinc-200 font-mono">ctx</code> parameter passed by the host runtime into the entrypoint.
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-emerald-400 font-mono">Explicit Grants</span>
            <p className="text-zinc-400">Capabilities are granted only if explicitly declared in the frontmatter manifest under <code className="text-zinc-300 font-mono">capabilities</code>.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-blue-400 font-mono">Confined Paths</span>
            <p className="text-zinc-400">Filesystem operations are strictly chrooted. Canonical path sanitization prevents directory traversal (<code className="text-zinc-300 font-mono">../../</code>).</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-purple-400 font-mono">Auditable Invariants</span>
            <p className="text-zinc-400">Every capability invocation increments monotonic fuel meters and produces structured audit logs in the host runner.</p>
          </div>
        </div>
      </section>

      {/* 2. Context API Specification */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">2. Capability Context API Specification</h2>
          <p className="text-sm text-zinc-400 mt-1">Detailed methods available on the <code className="text-zinc-200 font-mono">ctx</code> struct parameter.</p>
        </div>

        {/* Filesystem */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">ctx.fs</span>
              <span className="text-[10px] font-mono rounded bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 text-amber-400">
                Requires capabilities.fs
              </span>
            </div>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Provides sandboxed filesystem operations strictly bounded by <code className="text-zinc-200 font-mono">confined_read_roots</code>.
          </p>

          <div className="overflow-x-auto rounded-lg border border-zinc-850">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
                <tr>
                  <th className="p-2.5">Method</th>
                  <th className="p-2.5">Returns</th>
                  <th className="p-2.5 font-sans">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-850 text-zinc-300">
                <tr>
                  <td className="p-2.5 text-blue-400">ctx.fs.read(path: string)</td>
                  <td className="p-2.5 text-zinc-400">string | None</td>
                  <td className="p-2.5 font-sans">Reads text content. Returns <code className="text-zinc-200">None</code> if file is missing. Raises capability error if path escapes root.</td>
                </tr>
                <tr>
                  <td className="p-2.5 text-blue-400">ctx.fs.exists(path: string)</td>
                  <td className="p-2.5 text-zinc-400">bool</td>
                  <td className="p-2.5 font-sans">Returns boolean indicating whether file exists within whitelisted roots.</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        {/* Crypto */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">ctx.crypto</span>
              <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
                Pure Deterministic (Zero Grant Needed)
              </span>
            </div>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Cryptographic primitives implemented in native Rust for high performance and strict mathematical reproducibility.
          </p>

          <div className="overflow-x-auto rounded-lg border border-zinc-850">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
                <tr>
                  <th className="p-2.5">Method</th>
                  <th className="p-2.5">Returns</th>
                  <th className="p-2.5 font-sans">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-850 text-zinc-300">
                <tr>
                  <td className="p-2.5 text-blue-400">ctx.crypto.sha256(data: string)</td>
                  <td className="p-2.5 text-zinc-400">string</td>
                  <td className="p-2.5 font-sans">Computes 64-character lowercase hexadecimal SHA-256 checksum.</td>
                </tr>
                <tr>
                  <td className="p-2.5 text-blue-400">ctx.crypto.verify_ed25519(msg, sig, pubkey)</td>
                  <td className="p-2.5 text-zinc-400">bool</td>
                  <td className="p-2.5 font-sans">Validates Ed25519 digital signature against public key.</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        {/* Fuel & Execution Metering */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="text-base font-bold font-mono text-white">ctx.fuel</span>
              <span className="text-[10px] font-mono rounded bg-purple-500/10 border border-purple-500/20 px-2 py-0.5 text-purple-400">
                Axiom 5 Metering
              </span>
            </div>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Enables code to inspect its own resource consumption and gracefully terminate or downgrade workloads before quotas are exceeded.
          </p>

          <div className="overflow-x-auto rounded-lg border border-zinc-850">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-zinc-900/60 border-b border-zinc-850 text-zinc-400">
                <tr>
                  <th className="p-2.5">Method</th>
                  <th className="p-2.5">Returns</th>
                  <th className="p-2.5 font-sans">Description</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-zinc-850 text-zinc-300">
                <tr>
                  <td className="p-2.5 text-blue-400">ctx.fuel.consumed()</td>
                  <td className="p-2.5 text-zinc-400">int</td>
                  <td className="p-2.5 font-sans">Returns total opcode execution units consumed since invocation start.</td>
                </tr>
                <tr>
                  <td className="p-2.5 text-blue-400">ctx.fuel.remaining()</td>
                  <td className="p-2.5 text-zinc-400">int</td>
                  <td className="p-2.5 font-sans">Returns remaining gas before execution deadline is forcibly triggered.</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>

      {/* 3. Comprehensive Code Example */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">3. Complete End-to-End Example</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The following skill reads and checksums a file within a confined root directory, dynamically monitoring its fuel budget:
        </p>

        <div className="rounded-lg border border-zinc-850 bg-black p-4 font-mono text-xs text-zinc-300 overflow-x-auto">
          <pre>{`---
asl_version: "3.0"
name: "sandboxed-file-hasher"
version: "1.0.0"
description: "Confined file reader with fuel-aware cryptographic hashing"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "execute"
  input_schema:
    type: "object"
    required: ["filepath"]
    properties:
      filepath: { type: "string" }
capabilities:
  fs:
    confined_read_roots: ["./data", "./logs"]
limits:
  max_fuel_opcodes: 100000
---

# 1. Intent
Reads and cryptographically audits target files under sandboxed roots.

---

\`\`\`asl:deterministic
def execute(ctx, input):
    filepath = input.get("filepath")
    
    # 1. Defensive verification of path argument
    if not filepath:
        return {"ok": False, "error": "Argument 'filepath' is required."}

    # 2. Check remaining fuel quota before heavy processing
    if ctx.fuel.remaining() < 5000:
        return {"ok": False, "error": "Insufficient execution fuel budget."}

    # 3. Read file via sandboxed capability
    content = ctx.fs.read(filepath)
    if content == None:
        return {"ok": False, "error": "Target file does not exist."}

    # 4. Compute cryptographic digest
    sha = ctx.crypto.sha256(content)

    return {
        "ok": True,
        "filepath": filepath,
        "bytes_read": len(content),
        "sha256": sha,
        "opcodes_used": ctx.fuel.consumed()
    }
\`\`\``}</pre>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/stdlib"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Standard Library &amp; Builtins
        </Link>
        <Link
          href="/docs/rules"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>Declarative Rules (asl:rules)</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
