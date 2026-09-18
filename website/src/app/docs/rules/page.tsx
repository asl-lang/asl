import React from "react";
import Link from "next/link";
import { ArrowRight, Sparkles, ShieldCheck, Cpu, Code2, AlertOctagon, Landmark, CheckCircle2 } from "lucide-react";

export default function RulesDocsPage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Language Reference • Section 5
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Declarative Rules (<code className="text-blue-400 font-mono">asl:rules</code>)
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          Grammar specification and enterprise architectures for declarative policy engines, financial AML risk scoring, and zero-trust security firewalls in native ASL syntax (compatible with the Starlark L1 runtime).
        </p>
      </div>

      {/* 1. Grammar & Clause Specification */}
      <section className="space-y-6 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">1. Formal Grammar &amp; Clause Primitives</h2>
          <p className="text-sm text-zinc-400 mt-1">Syntax for fail-fast precondition assertions and pattern-matching matrices in ASL.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs font-mono">
          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
            <span className="text-emerald-400 font-bold">1. Guard Clauses (Preconditions)</span>
            <p className="text-zinc-400 font-sans">Evaluated sequentially before matching begins. Rejects invalid requests immediately.</p>
            <pre className="text-zinc-300 bg-black p-2.5 rounded border border-zinc-850 overflow-x-auto">{`\`\`\`asl:rules
guard:
  input.payload is not empty else reject("Empty payload")
  input.amount > 0 else reject("Invalid amount")
  input.verified is true else reject("Unverified account")
\`\`\``}</pre>
          </div>

          <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2">
            <span className="text-blue-400 font-bold">2. Pattern Match Matrix</span>
            <p className="text-zinc-400 font-sans">Multi-pattern disjunction with captured alias bindings for prefixes and substrings.</p>
            <pre className="text-zinc-300 bg-black p-2.5 rounded border border-zinc-850 overflow-x-auto">{`\`\`\`asl:rules
match input.target:
  when starts_with any(["prod-", "us-east-"]) as region:
    accept(status="routed", region=region)
  when contains any(["[urgent]", "[hotfix]"]):
    accept(status="expedited", priority=1)
  otherwise:
    reject("No routing policy matched")
\`\`\``}</pre>
          </div>
        </div>
      </section>

      {/* 2. Enterprise Architecture 1: Financial AML Risk Engine */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            <Landmark className="h-5 w-5 text-amber-400" />
            <span>Enterprise Case 1: Financial Settlement &amp; AML Risk Engine</span>
          </h2>
          <span className="text-[10px] font-mono rounded bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 text-amber-400">
            Mission-Critical
          </span>
        </div>
        <p className="text-sm text-zinc-300 leading-relaxed">
          High-value cross-border settlements require strict compliance checking: currency whitelists, sanction lists (OFAC), velocity thresholds, and automatic Suspicious Activity Report (SAR) tagging.
        </p>

        <div className="rounded-xl border border-zinc-800 bg-[#0c0c0c] p-4 font-mono text-xs text-zinc-300 overflow-x-auto leading-relaxed">
          <pre>{`\`\`\`asl:rules
# --- 1. PRE-SETTLEMENT GUARDS (FAIL-FAST COMPLIANCE) ---
guard:
  input.source_account is not empty else reject("ERR_SOURCE_MISSING: Origin account required.")
  input.destination_iban is not empty else reject("ERR_DEST_MISSING: Destination IBAN required.")
  input.amount_cents > 0 else reject("ERR_INVALID_AMOUNT: Transaction amount must be positive.")
  input.kyc_tier >= 2 else reject("ERR_KYC_INSUFFICIENT: Settlement requires KYC Tier 2 or above.")

# --- 2. MULTI-TIER SANCTION & VELOCITY MATRIX ---
match input.destination_iban:
  # Block sanctioned country codes immediately (OFAC compliance)
  when starts_with any(["IR", "KP", "SY", "CU"]) as sanctioned_code:
    reject("ERR_SANCTION_BLOCK: Transfers to jurisdiction " + sanctioned_code + " are prohibited by law.")

  # Cross-border high-value threshold (FinCEN CTR threshold: >= $10,000.00 USD / 1,000,000 cents)
  when input.amount_cents >= 1000000:
    accept(
      status="MANUAL_REVIEW",
      risk_score=95,
      requires_compliance_signoff=True,
      fincen_ctr_flag=True,
      clearing_channel="FEDWIRE_HEAVY"
    )

  # High-risk SWIFT corridor
  when starts_with any(["RU", "BY", "MM"]) as high_risk_corridor:
    accept(
      status="ENHANCED_DUE_DILIGENCE",
      risk_score=75,
      requires_compliance_signoff=True,
      fincen_ctr_flag=False,
      clearing_channel="SWIFT_EDD"
    )

  # SEPA and Domestic Instant Clearing
  when starts_with any(["US", "GB", "DE", "FR", "BR"]):
    accept(
      status="AUTO_APPROVED",
      risk_score=5,
      requires_compliance_signoff=False,
      fincen_ctr_flag=False,
      clearing_channel="INSTANT_CLEARING"
    )

  otherwise:
    accept(
      status="STANDARD_CLEARING",
      risk_score=25,
      requires_compliance_signoff=False,
      fincen_ctr_flag=False,
      clearing_channel="ACH_BATCH"
    )
\`\`\``}</pre>
        </div>
      </section>

      {/* 3. Enterprise Architecture 2: Zero-Trust Security Gateway */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <div className="flex items-center justify-between">
          <h2 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            <AlertOctagon className="h-5 w-5 text-rose-400" />
            <span>Enterprise Case 2: Autonomous Zero-Trust API Firewall</span>
          </h2>
          <span className="text-[10px] font-mono rounded bg-rose-500/10 border border-rose-500/20 px-2 py-0.5 text-rose-400">
            Security Hardened
          </span>
        </div>
        <p className="text-sm text-zinc-300 leading-relaxed">
          Protects LLM tool endpoints against prompt injection attacks, unauthorized role escalation, and tenant boundary hopping before payload reaches internal databases:
        </p>

        <div className="rounded-xl border border-zinc-800 bg-[#0c0c0c] p-4 font-mono text-xs text-zinc-300 overflow-x-auto leading-relaxed">
          <pre>{`\`\`\`asl:rules
# --- 1. PERIMETER AUTH & TENANT ISOLATION GUARDS ---
guard:
  input.tenant_id is not empty else reject("SEC_001: Missing tenant identifier.")
  input.auth_token is not null else reject("SEC_002: Bearer authorization token required.")
  input.actor_role is not empty else reject("SEC_003: RBAC actor role required.")

# --- 2. ATTACK VECTOR & ROUTING DISPATCH MATRIX ---
match input.query_payload:
  # Defense-in-depth: Prompt Injection & Jailbreak Heuristic Signatures
  when contains any(["ignore previous instructions", "system prompt", "DAN mode", "bypass rules"]):
    reject("SEC_ATTACK_DETECTED: Prompt injection attempt logged and reported to SOC.")

  # SQL Injection & Destructive Query Signatures
  when contains any(["DROP TABLE", "UNION SELECT", ";--", "OR 1=1"]):
    reject("SEC_SQLI_DETECTED: Malicious SQL tokens identified in query payload.")

  # Privileged Management Endpoints (Requires 'cluster_admin' role)
  when starts_with any(["/admin", "/v1/cluster", "/v1/keys"]):
    accept(
      authorized=input.actor_role == "cluster_admin",
      security_zone="AIR_GAPPED_CORE",
      audit_rate=1.0
    )

  # Read-Only Metrics & Observability Endpoints
  when starts_with any(["/metrics", "/healthz", "/ready"]):
    accept(
      authorized=True,
      security_zone="PUBLIC_PROBE",
      audit_rate=0.01
    )

  otherwise:
    accept(
      authorized=input.actor_role in ["cluster_admin", "developer", "operator"],
      security_zone="STANDARD_VPC",
      audit_rate=0.1
    )
\`\`\``}</pre>
        </div>
      </section>

      {/* 4. Compilation Pipeline */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">4. AOT In-Memory Transpilation Pipeline</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          The <code className="text-zinc-200 font-mono">asl-parser</code> crate compiles rules blocks ahead-of-time directly into deterministic execution bytecode (fully compatible with the Starlark L1 runtime standard):
        </p>

        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-4 space-y-2 text-xs text-zinc-400 font-mono">
          <p><strong className="text-white">1. Defensive Path Resolution:</strong> Generates nested <code className="text-zinc-300">_asl_get(input, [&quot;a&quot;, &quot;b&quot;])</code> calls, completely eliminating runtime <code className="text-rose-400">KeyError</code> panics.</p>
          <p><strong className="text-white">2. Pure Function Dispatch:</strong> Emits pure Pythonic branch trees with exact variable bindings (<code className="text-zinc-300">_asl_starts_with_any</code>) running in $O(N)$ bounded opcodes.</p>
          <p><strong className="text-white">3. Zero-Allocation Mapping:</strong> Rejection payloads and acceptance structs map directly into native JSON responses matching <code className="text-zinc-300 font-mono">output_schema</code>.</p>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs/context"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Capability Context (ctx)
        </Link>
        <Link
          href="/docs/complex-workflows"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>Complex Multi-Stage Pipelines</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
