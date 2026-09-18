import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { VariablesSection } from "@/components/docs/sections/control-flow/VariablesSection";
import { ConditionalsSection } from "@/components/docs/sections/control-flow/ConditionalsSection";
import { BoundedLoopsSection } from "@/components/docs/sections/control-flow/BoundedLoopsSection";
import { FunctionsSection } from "@/components/docs/sections/control-flow/FunctionsSection";

export default function ControlFlowReferencePage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Section 2"
        title="Variables, Loops & Functions"
        description="Complete operational reference for variable bindings, lexical scoping, bounded iteration, and function definitions in Agent Skill Language (ASL 3.0) under the Axiom 5 Bounded Termination model."
      />

      <VariablesSection />
      <ConditionalsSection />
      <BoundedLoopsSection />
      <FunctionsSection />

      <DocsNavFooter
        prev={{ title: "Types & Data Model", href: "/docs/types" }}
        next={{ title: "Standard Library & Builtins", href: "/docs/stdlib" }}
      />
    </div>
  );
}
