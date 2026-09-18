import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const JsonSchemaMatrix: React.FC = () => {
  const mapping = [
    { schema: '"string"', asl: "string", engine: "ASL VM", notes: "Strict UTF-8 encoded string." },
    { schema: '"integer"', asl: "int", engine: "ASL VM", notes: "Zero-loss unbounded integer." },
    { schema: '"boolean"', asl: "bool", engine: "ASL VM", notes: "Strict boolean (true / false)." },
    { schema: '"array"', asl: "list", engine: "ASL VM", notes: "Zero-indexed JSON array." },
    { schema: '"object"', asl: "dict", engine: "ASL VM", notes: "String-keyed JSON object map." },
    { schema: '"null"', asl: "NoneType (None)", engine: "ASL VM", notes: "JSON literal null." },
  ];

  return (
    <DocsSection title="4. JSON Schema Mapping Matrix">
      <p className="text-sm text-zinc-300 leading-relaxed">
        When skills are executed, arguments conforming to <code className="text-zinc-200 font-mono">input_schema</code> are deserialized into ASL native types, and return values are verified against <code className="text-zinc-200 font-mono">output_schema</code>:
      </p>

      <div className="overflow-x-auto rounded-lg border border-zinc-800">
        <table className="w-full text-left text-xs">
          <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
            <tr>
              <th className="p-3">JSON Schema Type</th>
              <th className="p-3">ASL Runtime Type</th>
              <th className="p-3">Runtime Engine Mapping</th>
              <th className="p-3">Serialization Behavior</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
            {mapping.map((row) => (
              <tr key={row.schema}>
                <td className="p-3 text-blue-400">{row.schema}</td>
                <td className="p-3 text-white">{row.asl}</td>
                <td className="p-3 text-zinc-400">{row.engine}</td>
                <td className="p-3 font-sans">{row.notes}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </DocsSection>
  );
};
