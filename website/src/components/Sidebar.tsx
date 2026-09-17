"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Sparkles, Layers, BookOpen, Shield, Terminal, Cpu, FileText } from "lucide-react";

interface NavSection {
  title: string;
  items: {
    title: string;
    href: string;
    icon: React.ComponentType<{ className?: string }>;
    badge?: string;
  }[];
}

const DOCS_NAV: NavSection[] = [
  {
    title: "Introduction",
    items: [
      { title: "Overview & Philosophy", href: "/docs", icon: BookOpen },
      { title: "93.2% Token Reduction", href: "/docs/tokenomics", icon: Sparkles, badge: "Theorem" },
    ],
  },
  {
    title: "Architecture & Specification",
    items: [
      { title: "The Triad (.skill, .tool, .asl)", href: "/docs/triad", icon: Layers },
      { title: "Declarative Rules (asl:rules)", href: "/docs/rules", icon: BookOpen },
      { title: "The 7 Axioms & OCap", href: "/docs/axioms", icon: Shield },
    ],
  },
  {
    title: "Reference & Tools",
    items: [
      { title: "CLI & MCP Server", href: "/docs/cli", icon: Terminal },
      { title: "Interactive Playground", href: "/playground", icon: Cpu },
      { title: "Formal Scientific Paper", href: "/paper", icon: FileText, badge: "Peer-Review" },
    ],
  },
];

export const Sidebar: React.FC = () => {
  const pathname = usePathname();

  return (
    <aside className="w-64 shrink-0 border-r border-zinc-800/80 bg-black/40 py-6 pr-6 hidden lg:block sticky top-16 h-[calc(100vh-4rem)] overflow-y-auto">
      <div className="space-y-6">
        {DOCS_NAV.map((section) => (
          <div key={section.title} className="space-y-2">
            <h5 className="text-[11px] font-semibold uppercase tracking-wider text-zinc-400 pl-3">
              {section.title}
            </h5>
            <ul className="space-y-1">
              {section.items.map((item) => {
                const Icon = item.icon;
                const isActive = pathname === item.href;
                return (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      className={`flex items-center justify-between rounded-lg px-3 py-2 text-xs font-medium transition-all ${
                        isActive
                          ? "bg-zinc-800/80 text-white font-semibold shadow-sm border border-zinc-700/60"
                          : "text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200"
                      }`}
                    >
                      <div className="flex items-center gap-2.5">
                        <Icon className={`h-4 w-4 ${isActive ? "text-blue-400" : "text-zinc-500"}`} />
                        <span>{item.title}</span>
                      </div>
                      {item.badge && (
                        <span className="rounded bg-zinc-800 border border-zinc-700 px-1.5 py-0.2 text-[9px] font-mono text-zinc-300">
                          {item.badge}
                        </span>
                      )}
                    </Link>
                  </li>
                );
              })}
            </ul>
          </div>
        ))}
      </div>
    </aside>
  );
};
