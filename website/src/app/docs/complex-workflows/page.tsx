import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { PipelineModelSection } from "@/components/docs/sections/complex-workflows/PipelineModelSection";
import { DatabaseMigrationCase } from "@/components/docs/sections/complex-workflows/DatabaseMigrationCase";
import { DesignPatternsSection } from "@/components/docs/sections/complex-workflows/DesignPatternsSection";

export default function ComplexWorkflowsPage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Guide"
        title="Complex Multi-Stage Pipelines"
        description="Architectural guide for authoring mission-critical, enterprise-grade ASL skills combining interleaved semantic intent stages, modular deterministic functions, and cryptographic audit trails."
      />

      <PipelineModelSection />
      <DatabaseMigrationCase />
      <DesignPatternsSection />

      <DocsNavFooter
        prev={{ title: "Declarative Rules (asl:rules)", href: "/docs/rules" }}
        next={{ title: "The Canonical Triad", href: "/docs/triad" }}
      />
    </div>
  );
}
