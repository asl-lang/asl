import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const BuiltinsSection: React.FC = () => {
  const builtins = [
    { sig: "len(x)", ret: "int", desc: 'Returns the number of elements in string, list, dict, or tuple. len([1, 2, 3]) == 3.' },
    { sig: "range(stop)\nrange(start, stop[, step])", ret: "range", desc: 'Generates an immutable arithmetic progression. range(1, 5) # [1, 2, 3, 4].' },
    { sig: "min(*args, key=None)\nmax(*args, key=None)", ret: "T", desc: 'Returns smallest or largest item. max(10, 5, 20) == 20.' },
    { sig: "abs(x)", ret: "int", desc: 'Returns the absolute value of an integer. abs(-42) == 42.' },
    { sig: "sorted(iter, key=None, reverse=False)", ret: "list", desc: 'Returns a new sorted list. sorted([3, 1, 2]) == [1, 2, 3].' },
    { sig: "reversed(sequence)", ret: "iterator", desc: 'Returns a reverse iterator over the sequence.' },
    { sig: "enumerate(iter, start=0)", ret: "iterator", desc: 'Yields (index, item) pairs during loop iteration.' },
    { sig: "zip(*iterables)", ret: "iterator", desc: 'Aggregates corresponding elements from multiple iterables into tuples.' },
    { sig: "any(iter)\nall(iter)", ret: "bool", desc: 'Tests truth value of iterable elements. any([False, True]) == True.' },
    { sig: "type(x)", ret: "string", desc: 'Returns type name string: "int", "string", "list", "dict".' },
    { sig: "fail(msg)", ret: "noreturn", desc: 'Aborts deterministic execution immediately with an unrecoverable error message.' },
  ];

  return (
    <DocsSection title="1. Global Builtin Functions">
      <p className="text-sm text-zinc-300 leading-relaxed">
        The following functions are globally available in all ASL execution blocks without requiring any imports:
      </p>

      <div className="overflow-x-auto rounded-lg border border-zinc-800">
        <table className="w-full text-left text-xs">
          <thead className="bg-zinc-900 border-b border-zinc-800 text-zinc-300 font-mono uppercase text-[11px]">
            <tr>
              <th className="p-3">Function Signature</th>
              <th className="p-3">Return Type</th>
              <th className="p-3">Description &amp; Example</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800/80 bg-zinc-950 font-mono text-zinc-400">
            {builtins.map((b, i) => (
              <tr key={i}>
                <td className="p-3 text-white whitespace-pre-line">{b.sig}</td>
                <td className={`p-3 ${b.ret === "noreturn" ? "text-rose-400" : "text-blue-400"}`}>{b.ret}</td>
                <td className="p-3 font-sans">{b.desc}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </DocsSection>
  );
};
