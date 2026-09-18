import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { OcapPrinciplesSection } from "@/components/docs/sections/context/OcapPrinciplesSection";
import { ContextApiTables } from "@/components/docs/sections/context/ContextApiTables";
import { ContextCompleteExample } from "@/components/docs/sections/context/ContextCompleteExample";

export default function CapabilityContextPage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Section 4"
        title={
          <span>
            Capability Context (<code className="text-blue-400 font-mono">ctx</code>)
          </span>
        }
        description="Complete reference for the Object Capability (OCap) host interface, sandboxed filesystem access, cryptographic primitives, and fuel monitoring in Agent Skill Language (ASL 3.0)."
      />

      <OcapPrinciplesSection />
      <ContextApiTables />
      <ContextCompleteExample />

      <DocsNavFooter
        prev={{ title: "Standard Library & Builtins", href: "/docs/stdlib" }}
        next={{ title: "Declarative Rules (asl:rules)", href: "/docs/rules" }}
      />
    </div>
  );
}
