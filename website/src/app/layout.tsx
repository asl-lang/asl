import type { Metadata, Viewport } from "next";
import "./globals.css";
import { AppShell } from "@/components/AppShell";

export const viewport: Viewport = {
  width: "device-width",
  initialScale: 1,
};

export const metadata: Metadata = {
  title: "Agent Skill Language (ASL 3.0) — Deterministic Runtime for AI Agents",
  description:
    "An AI-First, hermetic, capability-secure execution framework and deterministic orchestration runtime for autonomous AI agents.",
  keywords: [
    "ASL",
    "Agent Skill Language",
    "Autonomous Agents",
    "LLM Runtime",
    "Deterministic Execution",
    "Rust",
    "Tokenomics",
    "Starlark",
    "Model Context Protocol",
  ],
  authors: [{ name: "Jean Catarina" }],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="dark">
      <head>
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/npm/katex@0.16.11/dist/katex.min.css"
          crossOrigin="anonymous"
        />
      </head>
      <body>
        <AppShell>{children}</AppShell>
      </body>
    </html>
  );
}
