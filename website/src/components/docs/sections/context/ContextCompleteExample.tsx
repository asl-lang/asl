import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const ContextCompleteExample: React.FC = () => {
  const completeSkill = `---
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

\`\`\`asl
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
\`\`\``;

  return (
    <DocsSection title="3. Complete End-to-End Example">
      <p className="text-sm text-zinc-300 leading-relaxed">
        The following skill reads and checksums a file within a confined root directory, dynamically monitoring its fuel budget:
      </p>

      <AslCodeBlock
        lang="asl"
        filename="sandboxed-file-hasher.skill"
        code={completeSkill}
      />
    </DocsSection>
  );
};
