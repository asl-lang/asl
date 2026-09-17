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
} from "lucide-react";

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
    title: "Getting Started",
    items: [
      { title: "Overview & Philosophy", href: "/docs", icon: BookOpen },
      { title: "CLI & Tooling Reference", href: "/docs/cli", icon: Terminal },
    ],
  },
  {
    title: "Language Specification",
    items: [
      { title: "Syntax Reference", href: "/docs/syntax", icon: Code2, badge: "Spec" },
      { title: "The Triad (.skill, .tool, .asl)", href: "/docs/triad", icon: Layers },
      { title: "Declarative Rules (asl:rules)", href: "/docs/rules", icon: BookOpen },
      { title: "The 7 Axioms & OCap", href: "/docs/axioms", icon: Shield },
    ],
  },
  {
    title: "Foundation & Ecosystem",
    items: [
      { title: "Tokenomics & KV-Cache", href: "/docs/tokenomics", icon: Terminal },
      { title: "Scientific Paper", href: "/paper", icon: FileText, badge: "Full-Text" },
      { title: "WebAssembly Playground", href: "/playground", icon: Cpu },
    ],
  },
];

export const Sidebar: React.FC = () => {
  const pathname = usePathname();
  const [mobileOpen, setMobileOpen] = useState(false);

  // Close mobile drawer on route change
  useEffect(() => {
    setMobileOpen(false);
  }, [pathname]);

  // Find current active item title for mobile breadcrumb
  let activeTitle = "Documentation";
  for (const section of DOCS_NAV) {
    for (const item of section.items) {
      if (pathname === item.href) {
        activeTitle = item.title;
        break;
      }
    }
  }

  const renderNavContent = () => (
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
                        ? "bg-zinc-800 text-white font-semibold border border-zinc-700/80 shadow-sm"
                        : "text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200"
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

  return (
    <>
      {/* Mobile Top Sub-Header with Breadcrumb and Menu Trigger */}
      <div className="lg:hidden w-full border-b border-zinc-800 bg-zinc-950/90 backdrop-blur-md px-4 py-2.5 flex items-center justify-between sticky top-16 z-30 mb-6 -mx-4 sm:-mx-6">
        <div className="flex items-center gap-1.5 text-xs text-zinc-400 font-mono">
          <span>Docs</span>
          <ChevronRight className="h-3.5 w-3.5 text-zinc-600" />
          <span className="text-white font-medium truncate max-w-[200px]">{activeTitle}</span>
        </div>
        <button
          onClick={() => setMobileOpen(!mobileOpen)}
          className="flex items-center gap-1.5 rounded-lg border border-zinc-800 bg-zinc-900 px-2.5 py-1 text-xs font-medium text-zinc-300 hover:border-zinc-700 hover:text-white transition-colors"
        >
          {mobileOpen ? <X className="h-3.5 w-3.5" /> : <Menu className="h-3.5 w-3.5" />}
          <span>{mobileOpen ? "Close" : "Menu"}</span>
        </button>
      </div>

      {/* Mobile Slide-Over Drawer */}
      {mobileOpen && (
        <div className="lg:hidden fixed inset-0 z-50 flex">
          {/* Backdrop */}
          <div
            className="fixed inset-0 bg-black/80 backdrop-blur-sm"
            onClick={() => setMobileOpen(false)}
          />

          {/* Drawer panel */}
          <div className="relative ml-0 flex h-full w-4/5 max-w-xs flex-col overflow-y-auto bg-black border-r border-zinc-800 p-6 shadow-2xl z-10">
            <div className="flex items-center justify-between border-b border-zinc-800 pb-4 mb-6">
              <div className="flex items-center gap-2">
                <span className="font-mono text-sm font-bold text-white tracking-tighter">λ ASL</span>
                <span className="text-xs text-zinc-400 font-mono">Documentation</span>
              </div>
              <button
                onClick={() => setMobileOpen(false)}
                className="rounded-lg p-1.5 text-zinc-400 hover:bg-zinc-900 hover:text-white"
              >
                <X className="h-4 w-4" />
              </button>
            </div>
            {renderNavContent()}
          </div>
        </div>
      )}

      {/* Desktop Sticky Sidebar */}
      <aside className="w-64 shrink-0 border-r border-zinc-800/80 bg-black/40 py-6 pr-6 hidden lg:block sticky top-16 h-[calc(100vh-4rem)] overflow-y-auto">
        {renderNavContent()}
      </aside>
    </>
  );
};
