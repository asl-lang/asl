import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { FragilityTriadSection } from "@/components/docs/sections/overview/FragilityTriadSection";
import { DualConsumerAstSection } from "@/components/docs/sections/overview/DualConsumerAstSection";
import { CoreTenetsSection } from "@/components/docs/sections/overview/CoreTenetsSection";

export default function DocsOverviewPage() {
  return (
    <div className="space-y-8">
      <DocsHeader
        category="Documentation • Track 1"
        title="Agent Skill Language Overview"
        description="An AI-First, hermetic programming language and deterministic runtime designed specifically for autonomous agent orchestration."
      />

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <FragilityTriadSection />
        <DualConsumerAstSection />
        <CoreTenetsSection />

        <DocsNavFooter
          next={{ title: "Complete Syntax Reference", href: "/docs/syntax" }}
        />
      </div>
    </div>
  );
}
