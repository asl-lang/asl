import React from "react";
import Link from "next/link";
import { ArrowRight, Terminal, BookOpen, Code2, Braces, Sparkles } from "lucide-react";

export default function StdlibReferencePage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Section 3
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Standard Library &amp; Builtins
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Complete API specification for all built-in functions, string methods, list methods, dict methods, and standard modules in Agent Skill Language (ASL 3.0).
        </p>
      </div>

      {/* 1. Global Builtin Functions */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. Global Builtin Functions</h2>
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
              <tr>
                <td className="p-3 text-white">len(x)</td>
                <td className="p-3 text-blue-400">int</td>
                <td className="p-3 font-sans">Returns the number of elements in string, list, dict, or tuple. <code className="text-zinc-200">len([1, 2, 3]) == 3</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">range(stop)<br />range(start, stop[, step])</td>
                <td className="p-3 text-blue-400">range</td>
                <td className="p-3 font-sans">Generates an immutable arithmetic progression. <code className="text-zinc-200">range(1, 5) # [1, 2, 3, 4]</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">min(*args, key=None)<br />max(*args, key=None)</td>
                <td className="p-3 text-blue-400">T</td>
                <td className="p-3 font-sans">Returns smallest or largest item. <code className="text-zinc-200">max(10, 5, 20) == 20</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">abs(x)</td>
                <td className="p-3 text-blue-400">int</td>
                <td className="p-3 font-sans">Returns the absolute value of an integer. <code className="text-zinc-200">abs(-42) == 42</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">sorted(iter, key=None, reverse=False)</td>
                <td className="p-3 text-blue-400">list</td>
                <td className="p-3 font-sans">Returns a new sorted list. <code className="text-zinc-200">sorted([3, 1, 2]) == [1, 2, 3]</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">reversed(sequence)</td>
                <td className="p-3 text-blue-400">iterator</td>
                <td className="p-3 font-sans">Returns a reverse iterator over the sequence.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">enumerate(iter, start=0)</td>
                <td className="p-3 text-blue-400">iterator</td>
                <td className="p-3 font-sans">Yields <code className="text-zinc-200">(index, item)</code> pairs during loop iteration.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">zip(*iterables)</td>
                <td className="p-3 text-blue-400">iterator</td>
                <td className="p-3 font-sans">Aggregates corresponding elements from multiple iterables into tuples.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">any(iter)<br />all(iter)</td>
                <td className="p-3 text-blue-400">bool</td>
                <td className="p-3 font-sans">Tests truth value of iterable elements. <code className="text-zinc-200">any([False, True]) == True</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">type(x)</td>
                <td className="p-3 text-blue-400">string</td>
                <td className="p-3 font-sans">Returns type name string: <code className="text-zinc-200">&quot;int&quot;, &quot;string&quot;, &quot;list&quot;, &quot;dict&quot;</code>.</td>
              </tr>
              <tr>
                <td className="p-3 text-white">fail(msg)</td>
                <td className="p-3 text-rose-400">noreturn</td>
                <td className="p-3 font-sans">Aborts deterministic execution immediately with an unrecoverable error message.</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      {/* 2. String Methods */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">2. String Methods</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          Strings in ASL are immutable UTF-8 sequences equipped with comprehensive inspection, formatting, and transformation methods:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs font-mono">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.strip([chars]) / lstrip / rstrip</span>
            <p className="text-zinc-400 font-sans">Strips leading/trailing whitespace or specified characters.</p>
            <code className="text-zinc-300 block">&quot; hello &quot;.strip() # &quot;hello&quot;</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.startswith(prefix) / endswith</span>
            <p className="text-zinc-400 font-sans">Checks prefix or suffix match; accepts string or tuple.</p>
            <code className="text-zinc-300 block">&quot;v1.2.0&quot;.startswith(&quot;v1&quot;) # True</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.split(sep=None, maxsplit=-1)</span>
            <p className="text-zinc-400 font-sans">Splits string into a list of substrings by delimiter.</p>
            <code className="text-zinc-300 block">&quot;a,b,c&quot;.split(&quot;,&quot;) # [&quot;a&quot;, &quot;b&quot;, &quot;c&quot;]</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.join(iterable)</span>
            <p className="text-zinc-400 font-sans">Concatenates an iterable of strings using <code className="text-zinc-200">s</code> as delimiter.</p>
            <code className="text-zinc-300 block">&quot;-&quot;.join([&quot;a&quot;, &quot;b&quot;]) # &quot;a-b&quot;</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.replace(old, new[, count])</span>
            <p className="text-zinc-400 font-sans">Replaces occurrences of substring with replacement string.</p>
            <code className="text-zinc-300 block">&quot;foo bar&quot;.replace(&quot;bar&quot;, &quot;baz&quot;)</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.lower() / upper() / capitalize()</span>
            <p className="text-zinc-400 font-sans">Case conversion with full Unicode support.</p>
            <code className="text-zinc-300 block">&quot;ASL&quot;.lower() # &quot;asl&quot;</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.find(sub) / rfind / index / rindex</span>
            <p className="text-zinc-400 font-sans">Returns index of substring or -1 if not found.</p>
            <code className="text-zinc-300 block">&quot;abcd&quot;.find(&quot;bc&quot;) # 1</code>
          </div>

          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-3.5 space-y-1">
            <span className="text-blue-400 font-bold">s.isdigit() / isalpha() / isalnum()</span>
            <p className="text-zinc-400 font-sans">Character class classification predicates.</p>
            <code className="text-zinc-300 block">&quot;12345&quot;.isdigit() # True</code>
          </div>
        </div>
      </section>

      {/* 3. List and Dict Methods */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">3. Collection Methods</h2>
          <p className="text-sm text-zinc-400 mt-1">Manipulation methods for mutable <code className="text-zinc-200 font-mono">list</code> and <code className="text-zinc-200 font-mono">dict</code> instances.</p>
        </div>

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
      </section>

      {/* 4. Standard Modules */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">4. Standard Library Modules</h2>
          <p className="text-sm text-zinc-400 mt-1">Pre-registered library extensions for serialization and records.</p>
        </div>

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
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL JSON Serialization in execute()
def execute(ctx, input):
    data = json.decode('{"task": "lint", "count": 42}')
    task_name = data["task"]   # "lint"

    payload = {"status": "success", "processed": [1, 2, 3]}
    raw_json = json.encode(payload)
    return {"task": task_name, "raw_json": raw_json}
\`\`\``}</pre>
          </div>
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
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-300 overflow-x-auto">
            <pre>{`\`\`\`asl:deterministic
# ASL Immutable Struct Records
def execute(ctx, input):
    point = struct(x=10, y=25, label="origin")
    distance = point.x + point.y   # 35
    return {"x": point.x, "y": point.y, "distance": distance}
\`\`\``}</pre>
          </div>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/control-flow"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Variables, Loops &amp; Functions
        </Link>
        <Link
          href="/docs/context"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>Capability Context (ctx)</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
