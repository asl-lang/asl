"use client";

import React, { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";
import { Search, X, BookOpen, Terminal, Sparkles, Layers, Shield, Cpu, ArrowRight } from "lucide-react";

interface SearchItem {
  id: string;
  title: string;
  category: string;
  href: string;
  icon: React.ComponentType<{ className?: string }>;
  description: string;
}

const SEARCH_INDEX: SearchItem[] = [
  {
    id: "tokenomics",
    title: "93.2% Token Reduction Theorem",
    category: "Tokenomics",
    href: "/docs/tokenomics",
    icon: Sparkles,
    description: "Mathematical breakdown of ReAct multi-turn collapse into atomic AOT calls.",
  },
  {
    id: "triad",
    title: "The Canonical Triad (.skill, .tool, .asl)",
    category: "Architecture",
    href: "/docs/triad",
    icon: Layers,
    description: "Multi-extension AI ecosystem and shadow markdown projection rules.",
  },
  {
    id: "rules",
    title: "Declarative Semantic Rules (asl:rules)",
    category: "Language",
    href: "/docs/rules",
    icon: BookOpen,
    description: "Transpilation of high-level validation rules to Strict Starlark L1.",
  },
  {
    id: "axioms",
    title: "The 7 Axioms of ASL & OCap Confinement",
    category: "Security",
    href: "/docs/axioms",
    icon: Shield,
    description: "Mark Miller capability discipline, Lampson confinement, and fuel limits.",
  },
  {
    id: "cli",
    title: "CLI & MCP Server Reference",
    category: "Tooling",
    href: "/docs/cli",
    icon: Terminal,
    description: "Complete command guide: run, check, expand, serve (stdio & SSE), analyze-prefix.",
  },
  {
    id: "paper",
    title: "Scientific Paper: Formal Proofs & Theorems",
    category: "Research",
    href: "/paper",
    icon: BookOpen,
    description: "Unabridged paper by Jean Catarina with KaTeX formulas and BibTeX.",
  },
  {
    id: "playground",
    title: "Interactive WebAssembly Playground",
    category: "Interactive",
    href: "/playground",
    icon: Cpu,
    description: "Simulate and test ASL rules, KV-cache prefix hits, and syntax online.",
  },
  {
    id: "axiom-6",
    title: "Axiom 6: Static Invariant Prefix & 100% KV-Cache",
    category: "Axioms",
    href: "/docs/axioms#axiom-6",
    icon: Sparkles,
    description: "Eliminating token recalculation across LLM provider GPU clusters.",
  },
];

interface CommandMenuProps {
  isOpen: boolean;
  onClose: () => void;
}

export const CommandMenu: React.FC<CommandMenuProps> = ({ isOpen, onClose }) => {
  const router = useRouter();
  const [query, setQuery] = useState("");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        if (isOpen) {
          onClose();
        } else {
          // Open handled externally or here
        }
      }
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose]);

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 50);
      setSelectedIndex(0);
      setQuery("");
    }
  }, [isOpen]);

  const filtered = SEARCH_INDEX.filter((item) => {
    const q = query.toLowerCase();
    return (
      item.title.toLowerCase().includes(q) ||
      item.category.toLowerCase().includes(q) ||
      item.description.toLowerCase().includes(q)
    );
  });

  const handleSelect = (item: SearchItem) => {
    onClose();
    router.push(item.href);
  };

  const handleInputKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setSelectedIndex((prev) => (prev + 1) % (filtered.length || 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setSelectedIndex((prev) => (prev - 1 + filtered.length) % (filtered.length || 1));
    } else if (e.key === "Enter" && filtered[selectedIndex]) {
      e.preventDefault();
      handleSelect(filtered[selectedIndex]);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-20 px-4">
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/70 backdrop-blur-sm transition-opacity"
        onClick={onClose}
      />

      {/* Modal dialog */}
      <div className="relative w-full max-w-xl rounded-xl border border-zinc-700/80 bg-zinc-950 shadow-2xl overflow-hidden animate-in fade-in zoom-in-95 duration-150">
        {/* Search header */}
        <div className="flex items-center gap-3 border-b border-zinc-800 px-4 py-3">
          <Search className="h-4 w-4 text-zinc-400" />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setSelectedIndex(0);
            }}
            onKeyDown={handleInputKeyDown}
            placeholder="Type a command, axiom, or search term..."
            className="flex-1 bg-transparent text-sm text-white placeholder-zinc-500 outline-none"
          />
          <button
            onClick={onClose}
            className="rounded p-1 text-zinc-400 hover:bg-zinc-800 hover:text-white"
          >
            <X className="h-4 w-4" />
          </button>
        </div>

        {/* Results list */}
        <div className="max-h-80 overflow-y-auto p-2">
          {filtered.length === 0 ? (
            <div className="py-8 text-center text-xs text-zinc-500">
              No matching documentation or commands found for &ldquo;{query}&rdquo;
            </div>
          ) : (
            filtered.map((item, idx) => {
              const Icon = item.icon;
              const isSelected = idx === selectedIndex;
              return (
                <div
                  key={item.id}
                  onClick={() => handleSelect(item)}
                  onMouseEnter={() => setSelectedIndex(idx)}
                  className={`flex items-center justify-between rounded-lg px-3 py-2.5 cursor-pointer transition-colors ${
                    isSelected ? "bg-zinc-800/80 text-white" : "text-zinc-300 hover:bg-zinc-900"
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <div
                      className={`flex h-7 w-7 items-center justify-center rounded border ${
                        isSelected
                          ? "border-zinc-500 bg-zinc-700 text-white"
                          : "border-zinc-800 bg-zinc-900 text-zinc-400"
                      }`}
                    >
                      <Icon className="h-3.5 w-3.5" />
                    </div>
                    <div>
                      <div className="text-xs font-medium text-white flex items-center gap-2">
                        {item.title}
                        <span className="text-[10px] font-mono text-zinc-500 border border-zinc-800 rounded px-1">
                          {item.category}
                        </span>
                      </div>
                      <div className="text-[11px] text-zinc-400 truncate max-w-sm">
                        {item.description}
                      </div>
                    </div>
                  </div>
                  <ArrowRight className={`h-3.5 w-3.5 text-zinc-500 ${isSelected ? "text-zinc-200" : ""}`} />
                </div>
              );
            })
          )}
        </div>

        {/* Footer info */}
        <div className="flex items-center justify-between border-t border-zinc-800 bg-zinc-900/40 px-4 py-2 text-[10px] text-zinc-500 font-mono">
          <div className="flex items-center gap-2">
            <span>↑↓ Navigate</span>
            <span>↵ Select</span>
            <span>ESC Close</span>
          </div>
          <span>ASL Engine 3.0</span>
        </div>
      </div>
    </div>
  );
};
