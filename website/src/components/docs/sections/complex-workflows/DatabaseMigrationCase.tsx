import React from "react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const DatabaseMigrationCase: React.FC = () => {
  const skillCode = `---
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

\`\`\`asl
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

\`\`\`asl
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

\`\`\`asl
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
\`\`\``;

  return (
    <DocsSection
      title="2. Enterprise Case Study: Database Migration Safety Gate"
      subtitle="An end-to-end mission-critical skill that analyzes SQL migration scripts, calculates table locks, enforces zero-downtime rules, and signs an audit certificate."
      badge="Full Skill Artifact"
      badgeColor="emerald"
    >
      <AslCodeBlock lang="asl" code={skillCode} />
    </DocsSection>
  );
};
