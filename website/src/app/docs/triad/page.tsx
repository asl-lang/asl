import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { TriadTableSection } from "@/components/docs/sections/triad/TriadTableSection";
import { ShadowProjectionSection } from "@/components/docs/sections/triad/ShadowProjectionSection";
import { InteractiveTriadSection } from "@/components/docs/sections/triad/InteractiveTriadSection";

export default function TriadDocsPage() {
  return (
    <div className="space-y-8">
      <DocsHeader
        category="Architecture • Track 2"
        title="The Canonical Triad (.skill, .tool, .asl)"
        description="How ASL 3.0 unifies autonomous agent capabilities, direct MCP tool calls, and native specifications under a polymorphic AST."
      />

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <TriadTableSection />
        <ShadowProjectionSection />
        <InteractiveTriadSection />

        <DocsNavFooter
          prev={{ title: "93.2% Token Reduction", href: "/docs/tokenomics" }}
          next={{ title: "Semantic Rules in ASL", href: "/docs/rules" }}
        />
      </div>
    </div>
  );
}
