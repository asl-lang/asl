import React from "react";
import Link from "next/link";
import {
  ArrowRight,
  ShieldCheck,
  Server,
  Landmark,
  Database,
  Cpu,
  Workflow,
  Sparkles,
  GitPullRequest,
  CheckCircle2,
  Lock,
  Flame,
} from "lucide-react";

export default function UseCasesPage() {
  return (
    <div className="space-y-12">
      {/* Header */}
      <div>
        <div className="text-xs font-mono font-medium text-emerald-400 uppercase tracking-wider">
          Architecture &amp; Ecosystem • Field Guide
        </div>
        <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
          Real-World Projects &amp; Daily Use Cases
        </h1>
        <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
          ASL is not just for isolated agent functions. It is a full-stack architectural paradigm for building end-to-end autonomous software systems, verifiable pipelines, enterprise security gates, and self-healing infrastructure.
        </p>
      </div>

      {/* Why ASL Everywhere? */}
      <section className="space-y-4 border-t border-zinc-800 pt-8">
        <h2 className="text-xl font-bold text-white tracking-tight">The Core Architectural Problem ASL Solves</h2>
        <p className="text-sm text-zinc-300 leading-relaxed">
          When engineers build autonomous systems today using plain Python, Bash, or raw Markdown prompts, they encounter two fatal failure modes:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-xs">
          <div className="rounded-xl border border-rose-900/40 bg-rose-950/15 p-5 space-y-2">
            <h3 className="font-bold text-rose-400 font-mono flex items-center gap-2">
              <span>Fatal Mode A: The Ambient Authority Trap (Python/Bash)</span>
            </h3>
            <p className="text-zinc-300 leading-relaxed">
              Giving an LLM access to a Python interpreter or shell script means giving it <strong>unbounded ambient authority</strong>. A confused or compromised model can run <code className="text-rose-300 font-mono">os.system(&quot;rm -rf /&quot;)</code>, leak cloud credentials, run infinite loops, or trigger non-deterministic math.
            </p>
          </div>

          <div className="rounded-xl border border-amber-900/40 bg-amber-950/15 p-5 space-y-2">
            <h3 className="font-bold text-amber-400 font-mono flex items-center gap-2">
              <span>Fatal Mode B: The Hallucination Hazard (Raw Prompts)</span>
            </h3>
            <p className="text-zinc-300 leading-relaxed">
              Relying solely on natural language prompts has zero mathematical guarantees. The model can hallucinate discounts, skip mandatory compliance checks, output malformed JSON, and blow millions of tokens re-parsing prompt context.
            </p>
          </div>
        </div>

        <div className="rounded-lg border border-zinc-800 bg-zinc-950 p-4 text-xs text-zinc-300 flex items-center gap-3">
          <Sparkles className="h-5 w-5 text-emerald-400 shrink-0" />
          <div>
            <strong className="text-white font-mono">The ASL Synthesis: </strong>
            The LLM provides the creative semantic reasoning (Region 2), while the sandboxed ASL runtime enforces mathematical invariants, strict schemas, gas budgets, and cryptographic audit trails (Regions 1 &amp; 3).
          </div>
        </div>
      </section>

      {/* 6 Real-World Blueprints */}
      <section className="space-y-8 border-t border-zinc-800 pt-8">
        <div>
          <h2 className="text-xl font-bold text-white tracking-tight">6 End-to-End Enterprise Project Blueprints</h2>
          <p className="text-sm text-zinc-400 mt-1">
            Real architectural blueprints where entire production subsystems are built natively on ASL.
          </p>
        </div>

        {/* Blueprint 1: DevOps CI/CD */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="rounded-lg bg-blue-500/10 p-2 text-blue-400 border border-blue-500/20">
                <GitPullRequest className="h-5 w-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-white">1. Autonomous CI/CD Release &amp; Migration Gate</h3>
                <span className="text-xs text-zinc-500 font-mono">DevOps • SRE • Continuous Delivery</span>
              </div>
            </div>
            <span className="text-[10px] font-mono rounded bg-blue-500/10 border border-blue-500/20 px-2 py-0.5 text-blue-400">
              Zero-Downtime
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Instead of fragile Bash scripts in GitHub Actions, ASL powers the entire PR triage and release pipeline. The LLM summarizes PR diffs and detects semantic breaking changes. Then, deterministic ASL blocks verify SemVer increments, inspect SQL migration files for table locks, calculate blast-radius across microservices, and cryptographically sign the release artifact.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
            <span className="text-zinc-500"># Typical stack:</span> GitHub Action triggers <code className="text-zinc-200">asl run release-gate.skill --input &#39;&#123;&quot;pr&quot;: 142&#125;&#39;</code> &rarr; auto-merges or blocks PR with signed diagnostic.
          </div>
        </div>

        {/* Blueprint 2: Zero-Trust Security Firewall */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="rounded-lg bg-rose-500/10 p-2 text-rose-400 border border-rose-500/20">
                <ShieldCheck className="h-5 w-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-white">2. Autonomous AI Security Firewall &amp; Prompt Gateway</h3>
                <span className="text-xs text-zinc-500 font-mono">AppSec • LLM Defense • Guardrails</span>
              </div>
            </div>
            <span className="text-[10px] font-mono rounded bg-rose-500/10 border border-rose-500/20 px-2 py-0.5 text-rose-400">
              SOC2 / ISO 27001
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Placed in front of internal AI services. Uses <code className="text-zinc-200 font-mono">asl:rules</code> to intercept prompt injection attempts, detect jailbreak signatures (<code className="text-zinc-200">&quot;DAN mode&quot;</code>, <code className="text-zinc-200">&quot;ignore previous instructions&quot;</code>), enforce tenant boundary isolation, and scrub sensitive PII (credit cards, API keys) before data reaches external LLM APIs.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
            <span className="text-zinc-500"># Performance advantage:</span> Transpiled in-memory to strict Starlark L1, processing requests in &lt; 200 microseconds with zero garbage collection pause.
          </div>
        </div>

        {/* Blueprint 3: Fintech Settlement & AML */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="rounded-lg bg-emerald-500/10 p-2 text-emerald-400 border border-emerald-500/20">
                <Landmark className="h-5 w-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-white">3. Financial Clearing &amp; AML Compliance Engine</h3>
                <span className="text-xs text-zinc-500 font-mono">Fintech • Cross-Border Payments • Fraud Prevention</span>
              </div>
            </div>
            <span className="text-[10px] font-mono rounded bg-emerald-500/10 border border-emerald-500/20 px-2 py-0.5 text-emerald-400">
              OFAC / FinCEN
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Multi-currency financial transactions are audited in real time. Pre-clearing guards verify KYC tiers, check IBAN prefix corridors against OFAC sanction lists, and flag transfers over $10,000 for FinCEN CTR compliance. Domestic low-risk transfers clear instantly, while anomalous velocity routes to enhanced human review.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
            <span className="text-zinc-500"># Verifiability:</span> Every transaction decision is signed with <code className="text-zinc-200">ctx.crypto.sha256()</code>, providing a tamper-proof audit trail for regulatory inspections.
          </div>
        </div>

        {/* Blueprint 4: Self-Healing SRE */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="rounded-lg bg-purple-500/10 p-2 text-purple-400 border border-purple-500/20">
                <Server className="h-5 w-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-white">4. Self-Healing SRE Incident Responder</h3>
                <span className="text-xs text-zinc-500 font-mono">Cloud Infrastructure • Kubernetes • Observability</span>
              </div>
            </div>
            <span className="text-[10px] font-mono rounded bg-purple-500/10 border border-purple-500/20 px-2 py-0.5 text-purple-400">
              Autonomous SRE
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            When Prometheus or Datadog fires an alert (e.g. <code className="text-zinc-200">CrashLoopBackOff</code> or <code className="text-zinc-200">DiskPressure</code>), an ASL skill is invoked. The LLM inspects stack traces and recent git commits to hypothesize the root cause. Meanwhile, deterministic blocks execute sandboxed diagnostics, verify that rebooting a replica won&apos;t breach the P99 SLA, and safely execute remediation without human intervention.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
            <span className="text-zinc-500"># OCap Protection:</span> The skill is granted confined access to specific namespace logs only, preventing unauthorized lateral movement across clusters.
          </div>
        </div>

        {/* Blueprint 5: Customer Service Arbitration */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="rounded-lg bg-amber-500/10 p-2 text-amber-400 border border-amber-500/20">
                <Workflow className="h-5 w-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-white">5. Autonomous Dispute &amp; Refund Arbitration Bot</h3>
                <span className="text-xs text-zinc-500 font-mono">E-Commerce • Customer Ops • Risk Management</span>
              </div>
            </div>
            <span className="text-[10px] font-mono rounded bg-amber-500/10 border border-amber-500/20 px-2 py-0.5 text-amber-400">
              Deterministic Limits
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Customer refund disputes are evaluated by an autonomous agent. The LLM handles sympathetic, multilingual customer communication and extracts item condition. But the refund decision itself is <strong>strictly deterministic</strong>: ASL rules enforce that refunds over $50 require photo proof, claims over 30 days are automatically rejected with legal terms, and duplicate claims are blocked.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
            <span className="text-zinc-500"># Zero Hallucination:</span> The LLM cannot accidentally promise an unauthorized $1,000 refund because the deterministic code enforces a hard mathematical ceiling.
          </div>
        </div>

        {/* Blueprint 6: Enterprise Codebase Migration */}
        <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="rounded-lg bg-cyan-500/10 p-2 text-cyan-400 border border-cyan-500/20">
                <Cpu className="h-5 w-5" />
              </div>
              <div>
                <h3 className="text-base font-bold text-white">6. Automated Codebase Modernizer &amp; Refactoring Bot</h3>
                <span className="text-xs text-zinc-500 font-mono">Developer Productivity • Monorepo Maintenance</span>
              </div>
            </div>
            <span className="text-[10px] font-mono rounded bg-cyan-500/10 border border-cyan-500/20 px-2 py-0.5 text-cyan-400">
              AST Verified
            </span>
          </div>
          <p className="text-xs text-zinc-300 leading-relaxed">
            Migrates legacy codebases (e.g. React class components to modern hooks, or Python 2 to 3). The LLM performs the creative code refactoring. ASL deterministic blocks parse the before/after AST, verify that exported function signatures remain byte-for-byte identical, check that test suites compile, and automatically commit only verified green diffs.
          </p>
          <div className="rounded-lg border border-zinc-850 bg-black p-3 font-mono text-xs text-zinc-400">
            <span className="text-zinc-500"># Monorepo Scale:</span> Bounded loops and gas counters prevent refactoring scripts from hanging indefinitely on massive 100,000-file codebases.
          </div>
        </div>
      </section>

      {/* Navigation Footer */}
      <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
        <Link
          href="/docs"
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← Overview &amp; Philosophy
        </Link>
        <Link
          href="/docs/cli"
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>CLI &amp; Tooling Reference</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      </div>
    </div>
  );
}
