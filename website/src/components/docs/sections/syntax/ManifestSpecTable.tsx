import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const ManifestSpecTable: React.FC = () => {
  return (
    <DocsSection
      title="3. Frontmatter Manifest Specification"
      subtitle="The strict YAML contract declaring metadata, interfaces, capabilities, and execution limits."
    >
      <div className="overflow-x-auto rounded-lg border border-zinc-800">
        <table className="w-full text-left text-xs">
          <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
            <tr>
              <th className="p-3">Field</th>
              <th className="p-3">Type</th>
              <th className="p-3">Required</th>
              <th className="p-3">Description</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
            <tr>
              <td className="p-3 text-white">asl_version</td>
              <td className="p-3 text-blue-400">string</td>
              <td className="p-3 text-emerald-400">Yes</td>
              <td className="p-3 font-sans">Must match <code className="text-zinc-200">&quot;3.0&quot;</code> or start with <code className="text-zinc-200">&quot;3.&quot;</code>.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">name</td>
              <td className="p-3 text-blue-400">string</td>
              <td className="p-3 text-emerald-400">Yes</td>
              <td className="p-3 font-sans">Unique identifier slug, e.g. <code className="text-zinc-200">&quot;git-commit-helper&quot;</code>.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">version</td>
              <td className="p-3 text-blue-400">string</td>
              <td className="p-3 text-zinc-500">No</td>
              <td className="p-3 font-sans">SemVer string, e.g. <code className="text-zinc-200">&quot;1.2.0&quot;</code>.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">description</td>
              <td className="p-3 text-blue-400">string</td>
              <td className="p-3 text-emerald-400">Yes</td>
              <td className="p-3 font-sans">Natural language summary for discovery and indexing.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">interface.protocol</td>
              <td className="p-3 text-blue-400">string</td>
              <td className="p-3 text-zinc-500">No</td>
              <td className="p-3 font-sans">Default: <code className="text-zinc-200">&quot;mcp-tool-v1&quot;</code>.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">interface.entrypoint</td>
              <td className="p-3 text-blue-400">string</td>
              <td className="p-3 text-emerald-400">Yes</td>
              <td className="p-3 font-sans">Function name in deterministic code invoked by host runtime.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">interface.input_schema</td>
              <td className="p-3 text-blue-400">object (JSON Schema)</td>
              <td className="p-3 text-emerald-400">Yes</td>
              <td className="p-3 font-sans">Strict JSON Schema for arguments passed to entrypoint.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">interface.output_schema</td>
              <td className="p-3 text-blue-400">object (JSON Schema)</td>
              <td className="p-3 text-zinc-500">No</td>
              <td className="p-3 font-sans">JSON Schema validating return data structure.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">capabilities.fs.confined_read_roots</td>
              <td className="p-3 text-blue-400">string[]</td>
              <td className="p-3 text-zinc-500">No</td>
              <td className="p-3 font-sans">Whitelisted root directories for sandboxed read access.</td>
            </tr>
            <tr>
              <td className="p-3 text-white">limits.max_fuel_opcodes</td>
              <td className="p-3 text-blue-400">integer</td>
              <td className="p-3 text-zinc-500">No</td>
              <td className="p-3 font-sans">Monotonic instruction limit (default: <code className="text-zinc-200">1,000,000</code>).</td>
            </tr>
            <tr>
              <td className="p-3 text-white">limits.max_heap_kib</td>
              <td className="p-3 text-blue-400">integer</td>
              <td className="p-3 text-zinc-500">No</td>
              <td className="p-3 font-sans">Memory allocation ceiling (default: <code className="text-zinc-200">8,192</code> = 8MB).</td>
            </tr>
          </tbody>
        </table>
      </div>
    </DocsSection>
  );
};
