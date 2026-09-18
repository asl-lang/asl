import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

export const StringMethodsSection: React.FC = () => {
  const methods = [
    { name: "s.strip([chars]) / lstrip / rstrip", desc: "Strips leading/trailing whitespace or specified characters.", ex: '" hello ".strip() # "hello"' },
    { name: "s.startswith(prefix) / endswith", desc: "Checks prefix or suffix match; accepts string or tuple.", ex: '"v1.2.0".startswith("v1") # True' },
    { name: "s.split(sep=None, maxsplit=-1)", desc: "Splits string into a list of substrings by delimiter.", ex: '"a,b,c".split(",") # ["a", "b", "c"]' },
    { name: "s.join(iterable)", desc: "Concatenates an iterable of strings using s as delimiter.", ex: '"-".join(["a", "b"]) # "a-b"' },
    { name: "s.replace(old, new[, count])", desc: "Replaces occurrences of substring with replacement string.", ex: '"foo bar".replace("bar", "baz")' },
    { name: "s.lower() / upper() / capitalize()", desc: "Case conversion with full Unicode support.", ex: '"ASL".lower() # "asl"' },
    { name: "s.find(sub) / rfind / index / rindex", desc: "Returns index of substring or -1 if not found.", ex: '"abcd".find("bc") # 1' },
    { name: "s.isdigit() / isalpha() / isalnum()", desc: "Character class classification predicates.", ex: '"12345".isdigit() # True' },
  ];

  return (
    <DocsSection title="2. String Methods">
      <p className="text-sm text-zinc-300 leading-relaxed">
        Strings in ASL are immutable UTF-8 sequences equipped with comprehensive inspection, formatting, and transformation methods:
      </p>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs font-mono">
        {methods.map((m, i) => (
          <div key={i} className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">{m.name}</span>
            <p className="text-zinc-400 font-sans">{m.desc}</p>
            <code className="text-zinc-300 block">{m.ex}</code>
          </div>
        ))}
      </div>
    </DocsSection>
  );
};
