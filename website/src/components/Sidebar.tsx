"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Layers,
  BookOpen,
  Shield,
  Terminal,
  Cpu,
  FileText,
  Code2,
  Menu,
  X,
  ChevronRight,
  Sparkles,
  Binary,
  Workflow,
} from "lucide-react";

export interface NavSection {
  title: string;
  items: {
    title: string;
    href: string;
    icon: React.ComponentType<{ className?: string }>;
    badge?: string;
  }[];
}

export const DOCS_NAV: NavSection[] = [
  {
    title: "Getting Started",
    items: [
      { title: "Overview & Philosophy", href: "/docs", icon: BookOpen },
      { title: "CLI & Tooling Reference", href: "/docs/cli", icon: Terminal },
    ],
  },
  {
    title: "Language Reference",
    items: [
      { title: "Syntax & File Anatomy", href: "/docs/syntax", icon: Code2, badge: "Spec" },
      { title: "Types & Data Model", href: "/docs/types", icon: Binary, badge: "Manual" },
      { title: "Variables, Loops & Functions", href: "/docs/control-flow", icon: Workflow, badge: "Manual" },
      { title: "Standard Library & Builtins", href: "/docs/stdlib", icon: Terminal, badge: "API" },
      { title: "Capability Context (ctx)", href: "/docs/context", icon: Shield, badge: "OCap" },
      { title: "Declarative Rules (asl:rules)", href: "/docs/rules", icon: Sparkles, badge: "DSL" },
    ],
  },
  {
    title: "Architecture & Specs",
    items: [
      { title: "The Triad (.skill, .tool, .asl)", href: "/docs/triad", icon: Layers },
      { title: "The 7 Axioms & OCap", href: "/docs/axioms", icon: Shield },
      { title: "Tokenomics & KV-Cache", href: "/docs/tokenomics", icon: Terminal },
    ],
  },
  {
    title: "Foundation & Ecosystem",
    items: [
      { title: "Scientific Paper", href: "/paper", icon: FileText, badge: "Full-Text" },
      { title: "WebAssembly Playground", href: "/playground", icon: Cpu },
    ],
  },
];

function NavList({ pathname, onNavigate }: { pathname: string; onNavigate?: () => void }) {
  return (
    <div className="space-y-6">
      {DOCS_NAV.map((section) => (
        <div key={section.title} className="space-y-2">
          <h5 className="text-[11px] font-semibold uppercase tracking-wider text-zinc-500 pl-3">
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
                    onClick={onNavigate}
                    className={`flex items-center justify-between rounded-md px-3 py-2 text-xs font-medium transition-colors ${
                      isActive
                        ? "bg-zinc-800 text-white font-semibold border border-zinc-700/70 shadow-sm"
                        : "text-zinc-400 hover:bg-zinc-900 hover:text-white"
                    }`}
                  >
                    <div className="flex items-center gap-2.5">
                      <Icon className={`h-4 w-4 ${isActive ? "text-white" : "text-zinc-500"}`} />
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
  );
}

/** Mobile Sub-Header and Slide-Over Drawer for Docs */
export const DocsMobileNav: React.FC = () => {
  const pathname = usePathname();
  const [open, setOpen] = useState(false);

  useEffect(() => {
    setOpen(false);
  }, [pathname]);

  // Find active title for breadcrumb
  let activeTitle = "Overview";
  for (const section of DOCS_NAV) {
    for (const item of section.items) {
      if (pathname === item.href) {
        activeTitle = item.title;
        break;
      }
    }
  }

  return (
    <div className="lg:hidden w-full border-b border-zinc-800 bg-zinc-950/95 backdrop-blur-md sticky top-14 z-30">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 flex h-11 items-center justify-between">
        <div className="flex items-center gap-1.5 text-xs text-zinc-400 font-mono">
          <span className="text-zinc-500">Docs</span>
          <ChevronRight className="h-3.5 w-3.5 text-zinc-600" />
          <span className="text-white font-medium truncate max-w-[200px]">{activeTitle}</span>
        </div>

        <button
          onClick={() => setOpen(!open)}
          className="flex items-center gap-1.5 rounded-md border border-zinc-800 bg-zinc-900 px-2.5 py-1 text-xs font-medium text-zinc-300 hover:border-zinc-700 hover:text-white transition-colors"
        >
          {open ? <X className="h-3.5 w-3.5" /> : <Menu className="h-3.5 w-3.5" />}
          <span>{open ? "Close" : "Menu"}</span>
        </button>
      </div>

      {/* Drawer */}
      {open && (
        <div className="fixed inset-0 z-50 flex">
          <div
            className="fixed inset-0 bg-black/80 backdrop-blur-sm"
            onClick={() => setOpen(false)}
          />
          <div className="relative ml-0 flex h-full w-4/5 max-w-xs flex-col overflow-y-auto bg-black border-r border-zinc-800 p-6 shadow-2xl z-10">
            <div className="flex items-center justify-between border-b border-zinc-800 pb-4 mb-6">
              <div className="flex items-center gap-2">
                <span className="font-mono text-sm font-bold text-white tracking-tighter">λ ASL</span>
                <span className="text-xs text-zinc-400 font-mono">Documentation</span>
              </div>
              <button
                onClick={() => setOpen(false)}
                className="rounded-lg p-1.5 text-zinc-400 hover:bg-zinc-900 hover:text-white"
              >
                <X className="h-4 w-4" />
              </button>
            </div>
            <NavList pathname={pathname} onNavigate={() => setOpen(false)} />
          </div>
        </div>
      )}
    </div>
  );
};

/** Desktop Sticky Left Sidebar */
export const Sidebar: React.FC = () => {
  const pathname = usePathname();

  return (
    <aside className="w-64 shrink-0 border-r border-zinc-800/80 bg-black/30 py-6 pr-6 hidden lg:block sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto">
      <NavList pathname={pathname} />
    </aside>
  );
};
