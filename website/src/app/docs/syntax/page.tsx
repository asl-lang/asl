import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { FileAnatomySection } from "@/components/docs/sections/syntax/FileAnatomySection";
import { DualConsumerModelSection } from "@/components/docs/sections/syntax/DualConsumerModelSection";
import { ManifestSpecTable } from "@/components/docs/sections/syntax/ManifestSpecTable";
import { RosettaStoneSection } from "@/components/docs/sections/syntax/RosettaStoneSection";
import { LanguageRefDirectory } from "@/components/docs/sections/syntax/LanguageRefDirectory";

export default function SyntaxReferencePage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Specification"
        title="Syntax & File Anatomy"
        description="Formal structure, dual-consumer semantic model, and complete manifest schema for Agent Skill Language (ASL 3.0). Every ASL unit unites machine metadata, neural instructions, and deterministic logic in a single atomic file."
      />

      <FileAnatomySection />
      <DualConsumerModelSection />
      <RosettaStoneSection />
      <ManifestSpecTable />
      <LanguageRefDirectory />

      <DocsNavFooter
        prev={{ title: "CLI & Tooling Reference", href: "/docs/cli" }}
        next={{ title: "Types & Data Model", href: "/docs/types" }}
      />
    </div>
  );
}
