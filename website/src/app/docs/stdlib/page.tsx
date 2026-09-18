import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { BuiltinsSection } from "@/components/docs/sections/stdlib/BuiltinsSection";
import { StringMethodsSection } from "@/components/docs/sections/stdlib/StringMethodsSection";
import { CollectionsSection } from "@/components/docs/sections/stdlib/CollectionsSection";
import { StandardModulesSection } from "@/components/docs/sections/stdlib/StandardModulesSection";

export default function StdlibReferencePage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Section 3"
        title="Standard Library & Builtins"
        description="Complete API specification for all built-in functions, string methods, list methods, dict methods, and standard modules in Agent Skill Language (ASL 3.0)."
      />

      <BuiltinsSection />
      <StringMethodsSection />
      <CollectionsSection />
      <StandardModulesSection />

      <DocsNavFooter
        prev={{ title: "Variables, Loops & Functions", href: "/docs/control-flow" }}
        next={{ title: "Capability Context (ctx)", href: "/docs/context" }}
      />
    </div>
  );
}
