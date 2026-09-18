import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { CliCommandsSection } from "@/components/docs/sections/cli/CliCommandsSection";
import { McpConfigSection } from "@/components/docs/sections/cli/McpConfigSection";

export default function CliDocsPage() {
  return (
    <div className="space-y-8">
      <DocsHeader
        category="Tooling • Track 5"
        title="CLI & MCP Server Reference"
        description="Complete reference for the pure-Rust asl binary, subcommands, and Model Context Protocol server integration."
      />

      <div className="border-t border-zinc-850 pt-6 space-y-6 text-sm text-zinc-300 leading-relaxed">
        <CliCommandsSection />
        <McpConfigSection />

        <DocsNavFooter
          prev={{ title: "The 7 Axioms & Security", href: "/docs/axioms" }}
          next={{ title: "The Formal Scientific Paper 🔬", href: "/paper" }}
        />
      </div>
    </div>
  );
}
