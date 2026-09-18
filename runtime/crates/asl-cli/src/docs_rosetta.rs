//! Canonical ASL Rosetta Stone: Migrating from Bash, Python, and Starlark to Pure ASL.

pub fn get_migration_docs() -> &'static str {
    r#"========================================================================
📖 ASL ROSETTA STONE: HOW TO DO ANYTHING IN PURE ASL (```asl)
========================================================================

Agent Skill Language (ASL 3.0) is a 100% pure, self-contained semantic
and deterministic programming language. All executable code blocks use
the single unified tag: ```asl

Quick Navigation:
  asl docs bash       - Shell/Bash to ASL migration cheat sheet
  asl docs python     - Python to ASL migration cheat sheet
  asl docs starlark   - Starlark to ASL migration cheat sheet

Key Principles:
1. One Universal Tag: Only ```asl is recognized. No multiple tags needed.
2. Declarative & Procedural: Write rules (match/guard) or functions (def run).
3. Hermetic Sandbox: All I/O occurs via the explicit `ctx` capability context.
4. Fuel Metering: Infinite loops and runaway execution are physically halted.
5. Typed JSON Return: Skills return structured dictionaries, not raw stdout."#
}

pub fn get_bash_migration_docs() -> &'static str {
    r#"### Bash to ASL Migration Cheat Sheet

| Bash Operation | Pure ASL Equivalent (```asl) |
|:---|:---|
| Read file: cat file.txt | content = ctx.fs.read("file.txt") |
| Write file: echo "$d" > f | ctx.fs.write("f", data) |
| HTTP GET: curl -s $URL | resp = ctx.http.get(url) |
| HTTP POST: curl -d "$b" $URL | resp = ctx.http.post(url, json=payload) |
| Condition: if [ "$x" = "ok" ] | match input.x:\n  when "ok":\n    return {"ok": True} |
| Grep: echo $s \| grep needle | "needle" in text (or pattern rules) |
| Pipe: step1 \| step2 | res1 = step1(input)\nreturn step2(res1) |
| Env var: $MY_VAR | val = ctx.env.get("MY_VAR") |
| Exit with error: exit 1 | reject("Operation failed") or return {"error": "..."} |

Example Bash Script:
  if [ "$1" = "ping" ]; then
    echo '{"reply": "pong"}'
  else
    exit 1
  fi

Canonical Pure ASL Skill:
```asl
match input.cmd:
  when "ping":
    return {"reply": "pong"}
  otherwise:
    return {"error": "unknown command"}
```"#
}

pub fn get_python_migration_docs() -> &'static str {
    r#"### Python to ASL Migration Cheat Sheet

| Python Pattern | Pure ASL Equivalent (```asl) |
|:---|:---|
| import json; json.loads(s) | data = json.decode(s) (built-in linear parser) |
| import json; json.dumps(o) | s = json.encode(o) |
| import hashlib; sha256() | digest = ctx.crypto.sha256(data) |
| import os; os.environ["K"] | val = ctx.env.get("K") |
| import re; re.match(pat, s) | match text:\n  when matches(pat): |
| dict.get(key, default) | input.get(key, default) |
| [x * 2 for x in items] | [x * 2 for x in items] (deterministic comprehensions) |
| def main(): sys.exit(0) | def run(ctx, input): return {"status": "ok"} |

Example Python Script:
  import hashlib
  def run(input_data):
      h = hashlib.sha256(input_data["text"].encode()).hexdigest()
      return {"hash": h}

Canonical Pure ASL Skill:
```asl
def run(ctx, input):
    text = input.get("text", "")
    h = ctx.crypto.sha256(text)
    return {"hash": h}
```"#
}

pub fn get_starlark_migration_docs() -> &'static str {
    r#"### Writing in ASL vs. Execution in Starlark Under the Hood

1. Authoring in Pure ASL:
   When authoring a .skill, .tool, or .asl file, you write strictly in ASL
   using the universal code tag: ```asl
   No foreign language tags (```starlark, ```asl:rules, ```asl:deterministic) are needed.

2. How ASL Works Under the Hood:
   Under the hood, the ASL engine parses your ```asl block and transpiles
   it Ahead-of-Time (AOT) directly in-memory into hermetic Starlark.
   The Starlark VM then executes the code with:
   • Physical fuel metering (guaranteed termination, zero runaway loops)
   • Strict capability confinement (OCap via explicit `ctx` handles)
   • Sandboxed memory without ambient authority or external OS imports

3. Declarative Rules vs Procedural Functions:
   Both are written in ```asl:
   • Declarative: match input.field: when "value": return ...
   • Procedural:  def run(ctx, input): return ...
   Both compile into verified Starlark L1 bytecode in RAM.

4. Inspecting the Generated Code:
   Run `asl expand <file>` to view the exact hermetic Starlark generated
   under the hood from your ASL skill."#
}
