import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const CollectionsSection: React.FC = () => {
  return (
    <DocsSection
      title="3. Collection Methods"
      subtitle="Manipulation methods for mutable list and dict instances in ASL."
    >
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
        {/* List methods */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white flex items-center gap-2">
            <span className="text-emerald-400">list</span> Methods
          </h3>
          <ul className="space-y-2 text-zinc-300 font-mono">
            <li><strong className="text-white">l.append(x):</strong> Appends element <code className="text-zinc-400">x</code> to the end.</li>
            <li><strong className="text-white">l.extend(iter):</strong> Appends all items from iterable.</li>
            <li><strong className="text-white">l.insert(i, x):</strong> Inserts <code className="text-zinc-400">x</code> at index <code className="text-zinc-400">i</code>.</li>
            <li><strong className="text-white">l.pop([i]):</strong> Removes and returns item at index (default: -1).</li>
            <li><strong className="text-white">l.remove(x):</strong> Removes first occurrence of value <code className="text-zinc-400">x</code>.</li>
            <li><strong className="text-white">l.clear():</strong> Removes all items from the list.</li>
            <li><strong className="text-white">l.index(x):</strong> Returns zero-based index of item.</li>
          </ul>
        </div>

        {/* Dict methods */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-3">
          <h3 className="text-sm font-semibold font-mono text-white flex items-center gap-2">
            <span className="text-blue-400">dict</span> Methods
          </h3>
          <ul className="space-y-2 text-zinc-300 font-mono">
            <li><strong className="text-white">d.get(key, default):</strong> Returns value for key, or fallback default.</li>
            <li><strong className="text-white">d.keys():</strong> Returns list of dictionary keys.</li>
            <li><strong className="text-white">d.values():</strong> Returns list of dictionary values.</li>
            <li><strong className="text-white">d.items():</strong> Returns list of <code className="text-zinc-400">(key, value)</code> tuples.</li>
            <li><strong className="text-white">d.update(other):</strong> Merges key-values from another dict.</li>
            <li><strong className="text-white">d.pop(key[, default]):</strong> Removes key and returns its value.</li>
            <li><strong className="text-white">d.clear():</strong> Removes all keys and values.</li>
          </ul>
        </div>
      </div>
    </DocsSection>
  );
};
