import React from "react";
import { Landmark } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const FinancialAmlSection: React.FC = () => {
  const amlRules = `# --- 1. PRE-SETTLEMENT GUARDS (FAIL-FAST COMPLIANCE) ---
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
    )`;

  return (
    <DocsSection
      title={
        <span className="flex items-center gap-2">
          <Landmark className="h-5 w-5 text-amber-400" />
          <span>Enterprise Case 1: Financial Settlement &amp; AML Risk Engine</span>
        </span>
      }
      badge="Mission-Critical"
      badgeColor="amber"
    >
      <p className="text-sm text-zinc-300 leading-relaxed">
        High-value cross-border settlements require strict compliance checking: currency whitelists, sanction lists (OFAC), velocity thresholds, and automatic Suspicious Activity Report (SAR) tagging.
      </p>

      <AslCodeBlock lang="asl" code={amlRules} />
    </DocsSection>
  );
};
