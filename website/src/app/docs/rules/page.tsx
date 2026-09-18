import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { RulesGrammarSection } from "@/components/docs/sections/rules/RulesGrammarSection";
import { FinancialAmlSection } from "@/components/docs/sections/rules/FinancialAmlSection";
import { SecurityFirewallSection } from "@/components/docs/sections/rules/SecurityFirewallSection";
import { TranspilationPipelineSection } from "@/components/docs/sections/rules/TranspilationPipelineSection";

export default function RulesDocsPage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Section 5"
        title={
          <span>
            Declarative Rules in ASL (<code className="text-blue-400 font-mono">```asl</code>)
          </span>
        }
        description="Grammar specification and enterprise architectures for declarative policy engines, financial AML risk scoring, and zero-trust security firewalls in pure ASL syntax (ASL VM)."
      />

      <RulesGrammarSection />
      <FinancialAmlSection />
      <SecurityFirewallSection />
      <TranspilationPipelineSection />

      <DocsNavFooter
        prev={{ title: "Capability Context (ctx)", href: "/docs/context" }}
        next={{ title: "Complex Multi-Stage Pipelines", href: "/docs/complex-workflows" }}
      />
    </div>
  );
}
