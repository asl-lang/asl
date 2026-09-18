import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const OcapPrinciplesSection: React.FC = () => {
  return (
    <DocsSection title="1. Object Capability (OCap) Security Model">
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
    </DocsSection>
  );
};
