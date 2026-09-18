import React from "react";
import { GitPullRequest, ShieldCheck, Landmark, Server, Workflow, Cpu } from "lucide-react";
import { DocsSection } from "@/components/docs/ui/DocsSection";

interface BlueprintCardProps {
  icon: React.ReactNode;
  title: string;
  category: string;
  badge: string;
  badgeColor?: "blue" | "rose" | "emerald" | "purple" | "amber" | "cyan";
  description: string;
  footerTag: string;
  footerContent: React.ReactNode;
}

const badgeColorClasses = {
  blue: "bg-blue-500/10 border-blue-500/20 text-blue-400",
  rose: "bg-rose-500/10 border-rose-500/20 text-rose-400",
  emerald: "bg-emerald-500/10 border-emerald-500/20 text-emerald-400",
  purple: "bg-purple-500/10 border-purple-500/20 text-purple-400",
  amber: "bg-amber-500/10 border-amber-500/20 text-amber-400",
  cyan: "bg-cyan-500/10 border-cyan-500/20 text-cyan-400",
};

const BlueprintCard: React.FC<BlueprintCardProps> = ({
  icon,
  title,
  category,
  badge,
  badgeColor = "blue",
  description,
  footerTag,
  footerContent,
}) => (
  <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-3">
        <div className={`rounded-lg p-2 border ${badgeColorClasses[badgeColor]}`}>
          {icon}
        </div>
        <div>
          <h3 className="text-base font-bold text-white">{title}</h3>
          <span className="text-xs text-zinc-500 font-mono">{category}</span>
        </div>
      </div>
      <span className={`text-[10px] font-mono rounded border px-2 py-0.5 ${badgeColorClasses[badgeColor]}`}>
        {badge}
      </span>
    </div>
    <p className="text-xs text-zinc-300 leading-relaxed">{description}</p>
    <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
      <span className="text-zinc-500">{footerTag}</span> {footerContent}
    </div>
  </div>
);

export const BlueprintsSection: React.FC = () => {
  return (
    <DocsSection
      title="6 End-to-End Enterprise Project Blueprints"
      subtitle="Real architectural blueprints where entire production subsystems are built natively on ASL."
    >
      <div className="space-y-6">
        <BlueprintCard
          icon={<GitPullRequest className="h-5 w-5" />}
          title="1. Autonomous CI/CD Release & Migration Gate"
          category="DevOps • SRE • Continuous Delivery"
          badge="Zero-Downtime"
          badgeColor="blue"
          description="Instead of fragile Bash scripts in GitHub Actions, ASL powers the entire PR triage and release pipeline. The LLM summarizes PR diffs and detects semantic breaking changes. Then, deterministic ASL blocks verify SemVer increments, inspect SQL migration files for table locks, calculate blast-radius across microservices, and cryptographically sign the release artifact."
          footerTag="# Typical stack:"
          footerContent={
            <>
              GitHub Action triggers <code className="text-zinc-200">asl run release-gate.skill --input &#39;&#123;&quot;pr&quot;: 142&#125;&#39;</code> &rarr; auto-merges or blocks PR with signed diagnostic.
            </>
          }
        />

        <BlueprintCard
          icon={<ShieldCheck className="h-5 w-5" />}
          title="2. Autonomous AI Security Firewall & Prompt Gateway"
          category="AppSec • LLM Defense • Guardrails"
          badge="SOC2 / ISO 27001"
          badgeColor="rose"
          description="Placed in front of internal AI services. Uses asl:rules to intercept prompt injection attempts, detect jailbreak signatures ('DAN mode', 'ignore previous instructions'), enforce tenant boundary isolation, and scrub sensitive PII (credit cards, API keys) before data reaches external LLM APIs."
          footerTag="# Performance advantage:"
          footerContent="Compiled in-memory to hermetic ASL execution bytecode (compatible with Starlark L1), processing requests in < 200 microseconds with zero garbage collection pause."
        />

        <BlueprintCard
          icon={<Landmark className="h-5 w-5" />}
          title="3. Financial Clearing & AML Compliance Engine"
          category="Fintech • Cross-Border Payments • Fraud Prevention"
          badge="OFAC / FinCEN"
          badgeColor="emerald"
          description="Multi-currency financial transactions are audited in real time. Pre-clearing guards verify KYC tiers, check IBAN prefix corridors against OFAC sanction lists, and flag transfers over $10,000 for FinCEN CTR compliance. Domestic low-risk transfers clear instantly, while anomalous velocity routes to enhanced human review."
          footerTag="# Verifiability:"
          footerContent={
            <>
              Every transaction decision is signed with <code className="text-zinc-200">ctx.crypto.sha256()</code>, providing a tamper-proof audit trail for regulatory inspections.
            </>
          }
        />

        <BlueprintCard
          icon={<Server className="h-5 w-5" />}
          title="4. Self-Healing SRE Incident Responder"
          category="Cloud Infrastructure • Kubernetes • Observability"
          badge="Autonomous SRE"
          badgeColor="purple"
          description="When Prometheus or Datadog fires an alert (e.g. CrashLoopBackOff or DiskPressure), an ASL skill is invoked. The LLM inspects stack traces and recent git commits to hypothesize the root cause. Meanwhile, deterministic blocks execute sandboxed diagnostics, verify that rebooting a replica won't breach the P99 SLA, and safely execute remediation without human intervention."
          footerTag="# OCap Protection:"
          footerContent="The skill is granted confined access to specific namespace logs only, preventing unauthorized lateral movement across clusters."
        />

        <BlueprintCard
          icon={<Workflow className="h-5 w-5" />}
          title="5. Autonomous Dispute & Refund Arbitration Bot"
          category="E-Commerce • Customer Ops • Risk Management"
          badge="Deterministic Limits"
          badgeColor="amber"
          description="Customer refund disputes are evaluated by an autonomous agent. The LLM handles sympathetic, multilingual customer communication and extracts item condition. But the refund decision itself is strictly deterministic: ASL rules enforce that refunds over $50 require photo proof, claims over 30 days are automatically rejected with legal terms, and duplicate claims are blocked."
          footerTag="# Zero Hallucination:"
          footerContent="The LLM cannot accidentally promise an unauthorized $1,000 refund because the deterministic code enforces a hard mathematical ceiling."
        />

        <BlueprintCard
          icon={<Cpu className="h-5 w-5" />}
          title="6. Automated Codebase Modernizer & Refactoring Bot"
          category="Developer Productivity • Monorepo Maintenance"
          badge="AST Verified"
          badgeColor="cyan"
          description="Migrates legacy codebases (e.g. React class components to modern hooks, or Python 2 to 3). The LLM performs the creative code refactoring. ASL deterministic blocks parse the before/after AST, verify that exported function signatures remain byte-for-byte identical, check that test suites compile, and automatically commit only verified green diffs."
          footerTag="# Monorepo Scale:"
          footerContent="Bounded loops and gas counters prevent refactoring scripts from hanging indefinitely on massive 100,000-file codebases."
        />
      </div>
    </DocsSection>
  );
};
