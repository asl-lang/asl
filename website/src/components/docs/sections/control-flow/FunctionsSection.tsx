import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const FunctionsSection: React.FC = () => {
  return (
    <DocsSection
      title="4. Functions & Entrypoint Signatures"
      subtitle="Defining procedures and the canonical capability entrypoint in ASL."
    >
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <h3 className="text-sm font-semibold font-mono text-white">4.1 Function Declaration</h3>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Functions are declared with <code className="text-zinc-200 font-mono">def</code>. Parameters support positional arguments, default values, and keyword arguments.
        </p>
        <AslCodeBlock
          lang="asl"
          code={`# Pure helper procedure in ASL
def sanitize_token(token, uppercase=False):
    cleaned = token.strip().replace(" ", "_")
    return cleaned.upper() if uppercase else cleaned.lower()

# Multiple return values (tuple packing)
def split_name(full_name):
    parts = full_name.split(" ", 1)
    if len(parts) == 2:
        return parts[0], parts[1]
    return parts[0], ""`}
        />
      </div>

      {/* Entrypoint Signature */}
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <div className="flex items-center justify-between">
          <h3 className="text-sm font-semibold font-mono text-white">4.2 The Capability Entrypoint</h3>
          <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
            Runtime Invocation Root
          </span>
        </div>
        <p className="text-xs text-zinc-300 leading-relaxed">
          The function designated in <code className="text-zinc-200 font-mono">interface.entrypoint</code> (by default <code className="text-zinc-200 font-mono">execute</code>) accepts exactly two parameters:
        </p>
        <ul className="text-xs text-zinc-400 space-y-1.5 list-disc pl-5">
          <li><code className="text-white font-mono">ctx</code>: The capability context struct providing sandboxed host capabilities (<code className="text-zinc-300">ctx.fs</code>, <code className="text-zinc-300">ctx.crypto</code>, <code className="text-zinc-300">ctx.fuel</code>).</li>
          <li><code className="text-white font-mono">input</code>: The input dictionary validated against <code className="text-zinc-300">interface.input_schema</code>.</li>
        </ul>

        <AslCodeBlock
          lang="asl"
          code={`def execute(ctx, input):
    # 1. Access validated input fields
    filename = input.get("filename")
    sha_only = input.get("sha_only", False)

    # 2. Invoke sandboxed capability
    content = ctx.fs.read(filename)
    if content == None:
        return {"success": False, "error": "File not found"}

    # 3. Compute cryptographic hash
    digest = ctx.crypto.sha256(content)

    # 4. Return structured dictionary matching output_schema
    return {
        "success": True,
        "digest": digest,
        "fuel_used": ctx.fuel.consumed(),
    }`}
        />
      </div>
    </DocsSection>
  );
};
