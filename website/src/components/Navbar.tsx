"use client";

import React from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Search, Github, Terminal, Sparkles, BookOpen, Layers, Cpu } from "lucide-react";

interface NavbarProps {
  onOpenSearch?: () => void;
}

export const Navbar: React.FC<NavbarProps> = ({ onOpenSearch }) => {
  const pathname = usePathname();

  const navLinks = [
    { href: "/docs", label: "Docs", icon: BookOpen },
    { href: "/docs/tokenomics", label: "93.2% Tokenomics", icon: Sparkles },
    { href: "/docs/triad", label: "The Triad", icon: Layers },
    { href: "/paper", label: "Scientific Paper 🔬", icon: Terminal },
    { href: "/playground", label: "Playground", icon: Cpu },
  ];

  return (
    <header className="sticky top-0 z-40 w-full border-b border-zinc-800/80 bg-black/80 backdrop-blur-md">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        {/* Brand */}
        <div className="flex items-center gap-6">
          <Link href="/" className="flex items-center gap-2.5 group">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-zinc-900 border border-zinc-700/60 group-hover:border-zinc-500 transition-colors">
              <span className="font-mono text-sm font-bold text-white tracking-tighter">λ</span>
            </div>
            <div className="flex flex-col">
              <span className="font-sans text-sm font-semibold tracking-tight text-white flex items-center gap-1.5">
                ASL
                <span className="rounded bg-zinc-800/80 px-1.5 py-0.5 text-[10px] font-mono font-medium text-zinc-300 border border-zinc-700/50">
                  v3.0
                </span>
              </span>
              <span className="text-[10px] font-medium text-zinc-500 tracking-wider uppercase">
                Language
              </span>
            </div>
          </Link>

          {/* Desktop Nav Links */}
          <nav className="hidden md:flex items-center gap-1 pl-4">
            {navLinks.map((link) => {
              const isActive = pathname === link.href || (link.href !== "/" && pathname.startsWith(link.href) && link.href !== "/docs");
              return (
                <Link
                  key={link.href}
                  href={link.href}
                  className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
                    isActive
                      ? "bg-zinc-800/60 text-white font-semibold"
                      : "text-zinc-400 hover:bg-zinc-900 hover:text-zinc-200"
                  }`}
                >
                  {link.label}
                </Link>
              );
            })}
          </nav>
        </div>

        {/* Right side: Search & Github */}
        <div className="flex items-center gap-3">
          <button
            onClick={onOpenSearch}
            className="flex items-center gap-2 rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-1.5 text-xs text-zinc-400 hover:border-zinc-700 hover:bg-zinc-800/60 hover:text-zinc-200 transition-colors"
          >
            <Search className="h-3.5 w-3.5 text-zinc-400" />
            <span className="hidden sm:inline">Search docs, theorems, CLI...</span>
            <span className="sm:hidden">Search</span>
            <kbd className="ml-2 hidden sm:inline-flex items-center rounded border border-zinc-700/80 bg-zinc-800 px-1.5 py-0.5 text-[10px] font-mono text-zinc-400">
              ⌘K
            </kbd>
          </button>

          <a
            href="https://github.com/asl-lang/asl"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1.5 rounded-lg border border-zinc-800 bg-zinc-900/60 px-3 py-1.5 text-xs font-medium text-zinc-300 hover:border-zinc-700 hover:bg-zinc-800/60 hover:text-white transition-colors"
          >
            <Github className="h-3.5 w-3.5" />
            <span className="hidden sm:inline">GitHub</span>
          </a>
        </div>
      </div>
    </header>
  );
};
