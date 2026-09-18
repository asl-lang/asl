import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { ReActComparisonSection } from "@/components/docs/sections/tokenomics/ReActComparisonSection";
import { TokenReductionProofSection } from "@/components/docs/sections/tokenomics/TokenReductionProofSection";
import { TokenomicDriversSection } from "@/components/docs/sections/tokenomics/TokenomicDriversSection";

export default function TokenomicsPage() {
  return (
    <div className="space-y-8">
      <DocsHeader
        category="Scientific Foundation • Tokenomics"
        title="The 93.2% Token Reduction Theorem"
        description="How Ahead-of-Time schema binding, elimination of exploratory ReAct turns, and static KV-cache invariance achieve mathematically proven token efficiency."
      />

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <ReActComparisonSection />
        <TokenReductionProofSection />
        <TokenomicDriversSection />

        <DocsNavFooter
          prev={{ title: "Back to Overview", href: "/docs" }}
          next={{ title: "Explore The Triad (.skill, .tool, .asl)", href: "/docs/triad" }}
        />
      </div>
    </div>
  );
}
