import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { ProblemStatementSection } from "@/components/docs/sections/use-cases/ProblemStatementSection";
import { BlueprintsSection } from "@/components/docs/sections/use-cases/BlueprintsSection";

export default function UseCasesPage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Architecture & Ecosystem • Field Guide"
        title="Real-World Projects & Daily Use Cases"
        description="ASL is not just for isolated agent functions. It is a full-stack architectural paradigm for building end-to-end autonomous software systems, verifiable pipelines, enterprise security gates, and self-healing infrastructure."
      />

      <ProblemStatementSection />
      <BlueprintsSection />

      <DocsNavFooter
        prev={{ title: "Overview & Philosophy", href: "/docs" }}
        next={{ title: "CLI & Tooling Reference", href: "/docs/cli" }}
      />
    </div>
  );
}
