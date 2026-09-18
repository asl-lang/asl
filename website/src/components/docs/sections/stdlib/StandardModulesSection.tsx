import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const StandardModulesSection: React.FC = () => {
  return (
    <DocsSection
      title="4. Standard Library Modules"
      subtitle="Pre-registered library extensions for serialization and records in ASL."
    >
      {/* JSON */}
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-base font-bold font-mono text-white">json</span>
            <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
              JSON Serialization Extension
            </span>
          </div>
        </div>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Fast, deterministic encoding and decoding of JSON text into ASL native structures.
        </p>
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    data = json.decode('{"task": "lint", "count": 42}')
    task_name = data["task"]   # "lint"

    payload = {"status": "success", "processed": [1, 2, 3]}
    raw_json = json.encode(payload)
    return {"task": task_name, "raw_json": raw_json}`}
        />
      </div>

      {/* Struct */}
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-base font-bold font-mono text-white">struct</span>
            <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
              Record Type Extension
            </span>
          </div>
        </div>
        <p className="text-xs text-zinc-300 leading-relaxed">
          Constructs immutable record objects with dot-accessible properties in ASL.
        </p>
        <AslCodeBlock
          lang="asl:deterministic"
          code={`def execute(ctx, input):
    point = struct(x=10, y=25, label="origin")
    distance = point.x + point.y   # 35
    return {"x": point.x, "y": point.y, "distance": distance}`}
        />
      </div>
    </DocsSection>
  );
};
