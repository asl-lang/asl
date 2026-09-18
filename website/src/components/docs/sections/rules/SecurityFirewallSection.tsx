import React from "react";
import { AlertOctagon } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const SecurityFirewallSection: React.FC = () => {
  const firewallRules = `# --- 1. PERIMETER AUTH & TENANT ISOLATION GUARDS ---
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
    )`;

  return (
    <DocsSection
      title={
        <span className="flex items-center gap-2">
          <AlertOctagon className="h-5 w-5 text-rose-400" />
          <span>Enterprise Case 2: Autonomous Zero-Trust API Firewall</span>
        </span>
      }
      badge="Security Hardened"
      badgeColor="rose"
    >
      <p className="text-sm text-zinc-300 leading-relaxed">
        Protects LLM tool endpoints against prompt injection attacks, unauthorized role escalation, and tenant boundary hopping before payload reaches internal databases:
      </p>

      <AslCodeBlock lang="asl" code={firewallRules} />
    </DocsSection>
  );
};
