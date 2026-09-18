import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { TypesPrinciples } from "@/components/docs/sections/types/TypesPrinciples";
import { PrimitiveTypes } from "@/components/docs/sections/types/PrimitiveTypes";
import { CompositeTypes } from "@/components/docs/sections/types/CompositeTypes";
import { JsonSchemaMatrix } from "@/components/docs/sections/types/JsonSchemaMatrix";

export default function TypesReferencePage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Section 1"
        title="Types & Data Model"
        description="Formal data type specifications, memory representation, immutability semantics, and JSON Schema boundary serialization in Agent Skill Language (ASL 3.0)."
      />

      <TypesPrinciples />
      <PrimitiveTypes />
      <CompositeTypes />
      <JsonSchemaMatrix />

      <DocsNavFooter
        prev={{ title: "Syntax & File Anatomy", href: "/docs/syntax" }}
        next={{ title: "Variables, Loops & Functions", href: "/docs/control-flow" }}
      />
    </div>
  );
}
