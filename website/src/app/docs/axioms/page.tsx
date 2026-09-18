import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { AxiomsListSection } from "@/components/docs/sections/axioms/AxiomsListSection";
import { AxiomsOcapBoundarySection } from "@/components/docs/sections/axioms/AxiomsOcapBoundarySection";

export default function AxiomsDocsPage() {
  return (
    <div className="space-y-8">
      <DocsHeader
        category="Architecture & Security • Track 3"
        title="The 7 Axioms of ASL 3.0"
        description="The non-negotiable mathematical and architectural invariants governing the Agent Skill Language ecosystem."
      />

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <AxiomsListSection />
        <AxiomsOcapBoundarySection />

        <DocsNavFooter
          prev={{ title: "Declarative Rules", href: "/docs/rules" }}
          next={{ title: "CLI & Tooling Reference", href: "/docs/cli" }}
        />
      </div>
    </div>
  );
}
