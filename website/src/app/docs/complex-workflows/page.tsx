import React from "react";
import Link from "next/link";
import { ArrowRight, Layers, Workflow, ShieldCheck, Flame, Cpu, CheckCircle2, GitBranch } from "lucide-react";

export default function ComplexWorkflowsPage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Guide
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Complex Multi-Stage Pipelines
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Architectural guide for authoring mission-critical, enterprise-grade ASL skills combining interleaved semantic intent stages, modular deterministic functions, and cryptographic audit trails.
        </p>
      </div>

      {/* 1. Multi-Stage Architecture */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">1. The Interleaved Pipeline Model</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          In real-world enterprise deployments (e.g. database migrations, CI/CD gates, financial reconciliations), a skill cannot be a simple 5-line script. It requires a <strong>multi-stage pipeline</strong> where the neural reasoner (LLM) and deterministic engine collaborate across progressive verification checkpoints.
        </p>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The ASL parser natively supports interleaving multiple CommonMark prompt sections with multiple code blocks. During compilation:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-emerald-400 font-mono">1. Code Assembly</span>
            <p className="text-zinc-400">All fenced blocks (<code className="text-zinc-300 font-mono">```asl:rules</code> and <code className="text-zinc-300 font-mono">```asl:deterministic</code>) are merged in source order into a single unified Starlark module.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-blue-400 font-mono">2. Semantic Preservation</span>
            <p className="text-zinc-400">All Markdown headings, step protocols, and few-shot examples are preserved byte-for-byte in the prompt envelope for the LLM.</p>
          </div>
          <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 space-y-1.5">
            <span className="font-semibold text-purple-400 font-mono">3. Single-Turn Invariance</span>
            <p className="text-zinc-400">The entire multi-stage protocol resides in one atomic file, guaranteeing 100% KV-cache hit rate across turns.</p>
          </div>
        </div>
      </section>

      {/* 2. End-to-End Enterprise Case Study */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">2. Enterprise Case Study: Database Migration Safety Gate</h2>
          <p className="text-sm text-zinc-400 mt-1">
            An end-to-end mission-critical skill that analyzes SQL migration scripts, calculates table locks, enforces zero-downtime rules, and signs an audit certificate.
          </p>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-[#0c0c0c] p-5 font-mono text-xs text-zinc-300 overflow-x-auto leading-relaxed">
          <pre>{`---
asl_version: "3.0"
name: "db-migration-safety-gate"
version: "2.1.0"
description: "Zero-downtime safety gate for production database schema migrations"
interface:
  protocol: "mcp-tool-v1"
  entrypoint: "evaluate_migration_pipeline"
  input_schema:
    type: "object"
    required: ["migration_file", "environment", "target_database"]
    properties:
      migration_file: { type: "string" }
      environment: { type: "string", enum: ["staging", "production"] }
      target_database: { type: "string" }
      max_allowed_lock_ms: { type: "integer", default: 100 }
capabilities:
  fs:
    confined_read_roots: ["./migrations", "./schema"]
limits:
  max_fuel_opcodes: 500000
  max_heap_kib: 16384
---

# STAGE 1: Migration Intake & Intent Extraction
## 1.1 Trigger Criteria
Invoke this skill whenever a pull request touches SQL files in \`./migrations/\`.
Do NOT run DDL migrations directly without passing this automated gate.

## 1.2 Agent Decision Tree
1. Verify that the migration filename starts with a timestamp (e.g. \`20260918_add_users.sql\`).
2. Pass the file path and targeted environment to the deterministic pipeline.
3. If \`is_safe\` is false, block deployment and present the exact violation diagnostics to the developer.

---

\`\`\`asl:deterministic
# --- SUB-ROUTINE 1: AST / DDL Static Risk Analyzer ---
DANGEROUS_PATTERNS = [
    ("ALTER TABLE", "ADD COLUMN", "NOT NULL DEFAULT", "Adding NOT NULL without concurrent backfill locks the table."),
    ("DROP TABLE", "", "", "DROP TABLE destroys historical records. Requires manual DBA bypass."),
    ("DROP COLUMN", "", "", "DROP COLUMN causes breaking changes for running API replicas."),
    ("CREATE INDEX", "", "", "CREATE INDEX must use CONCURRENTLY in production to avoid write locks."),
]

def analyze_sql_risk(sql_text, environment):
    upper_sql = sql_text.upper()
    violations = []
    
    for term1, term2, term3, reason in DANGEROUS_PATTERNS:
        matches_all = True
        for term in [term1, term2, term3]:
            if term != "" and term not in upper_sql:
                matches_all = False
                break
        if matches_all:
            if environment == "production" or "DROP" in term1:
                violations.append({
                    "pattern": term1 + (" " + term2 if term2 else ""),
                    "severity": "CRITICAL" if "DROP" in term1 else "HIGH",
                    "reason": reason
                })
    return violations
\`\`\`

# STAGE 2: Table Dependency & Blast Radius Calculation
The agent checks whether targeted tables are high-throughput hot tables.
Hot tables (\`orders\`, \`payments\`, \`users\`) cannot sustain exclusive table locks.

\`\`\`asl:deterministic
# --- SUB-ROUTINE 2: Blast Radius Calculation ---
HOT_TABLES = ["users", "accounts", "orders", "payments", "ledger"]

def calculate_blast_radius(sql_text):
    upper_sql = sql_text.upper()
    impacted_hot_tables = []
    for table in HOT_TABLES:
        if (" " + table.upper() + " ") in upper_sql or (" " + table.upper() + ";") in upper_sql:
            impacted_hot_tables.append(table)
    
    risk_level = "LOW"
    if len(impacted_hot_tables) >= 2:
        risk_level = "CRITICAL"
    elif len(impacted_hot_tables) == 1:
        risk_level = "MEDIUM"
        
    return {
        "impacted_hot_tables": impacted_hot_tables,
        "risk_level": risk_level
    }
\`\`\`

# STAGE 3: Final Gate Evaluation & Cryptographic Attestation

\`\`\`asl:deterministic
# --- SUB-ROUTINE 3: Main Orchestration Entrypoint ---
def evaluate_migration_pipeline(ctx, input):
    # 1. Fuel quota pre-flight inspection
    if ctx.fuel.remaining() < 10000:
        return {"approved": False, "error": "Insufficient fuel budget for migration analysis."}

    filepath = input.get("migration_file")
    environment = input.get("environment")

    # 2. Confined file read via capability sandbox
    sql_content = ctx.fs.read(filepath)
    if sql_content == None:
        return {"approved": False, "error": "Migration file not found in ./migrations"}

    # 3. Step 1: Static DDL linting
    violations = analyze_sql_risk(sql_content, environment)

    # 4. Step 2: Blast radius analysis
    blast_radius = calculate_blast_radius(sql_content)

    # 5. Determine approval status
    is_approved = (len(violations) == 0) and (blast_radius["risk_level"] != "CRITICAL")
    
    # 6. Generate cryptographic audit certificate
    audit_payload = filepath + ":" + environment + ":" + str(is_approved)
    audit_digest = ctx.crypto.sha256(audit_payload)

    return {
        "approved": is_approved,
        "environment": environment,
        "file": filepath,
        "file_sha256": ctx.crypto.sha256(sql_content),
        "violations": violations,
        "blast_radius": blast_radius,
        "audit_certificate": audit_digest,
        "fuel_consumed": ctx.fuel.consumed()
    }
\`\`\``}</pre>
        </div>
      </section>

      {/* 3. Architectural Best Practices */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">3. Enterprise Design Patterns</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          When scaling skills to complex multi-step systems, adhere to these production patterns:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
            <h3 className="font-bold text-white font-mono flex items-center gap-2">
              <Workflow className="h-4 w-4 text-blue-400" />
              <span>Modular Decomposition</span>
            </h3>
            <p className="text-zinc-400 leading-relaxed">
              Break complex logic into pure sub-routines (e.g. <code className="text-zinc-200">analyze_sql_risk</code>, <code className="text-zinc-200">calculate_blast_radius</code>) declared in separate code blocks adjacent to their corresponding semantic documentation.
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
            <h3 className="font-bold text-white font-mono flex items-center gap-2">
              <Flame className="h-4 w-4 text-amber-400" />
              <span>Fuel Budget Checks</span>
            </h3>
            <p className="text-zinc-400 leading-relaxed">
              Always query <code className="text-zinc-200">ctx.fuel.remaining()</code> prior to traversing large ASTs or running string parsing loops. If gas is low, degrade gracefully rather than allowing a hard termination fault.
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
            <h3 className="font-bold text-white font-mono flex items-center gap-2">
              <ShieldCheck className="h-4 w-4 text-emerald-400" />
              <span>Cryptographic Attestation</span>
            </h3>
            <p className="text-zinc-400 leading-relaxed">
              Generate SHA-256 audit certificates using <code className="text-zinc-200">ctx.crypto.sha256()</code> over the input parameters and decision payload. This provides an immutable paper trail for compliance audits (SOC2, ISO 27001).
            </p>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-5 space-y-2">
            <h3 className="font-bold text-white font-mono flex items-center gap-2">
              <GitBranch className="h-4 w-4 text-purple-400" />
              <span>Strict Fallback Action</span>
            </h3>
            <p className="text-zinc-400 leading-relaxed">
              Always return structured error dictionaries with <code className="text-zinc-200">&quot;approved&quot;: False</code> rather than relying on unhandled exceptions, so the agent can interpret the failure and self-correct.
            </p>
          </div>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/rules"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Declarative Rules (asl:rules)
        </Link>
        <Link
          href="/docs/triad"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>The Canonical Triad</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
