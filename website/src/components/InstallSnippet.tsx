"use client";

import React, { useState } from "react";
import { Check, Copy } from "lucide-react";

export const InstallSnippet: React.FC = () => {
  const [copied, setCopied] = useState(false);
  const command = "curl -fsSL https://raw.githubusercontent.com/asl-lang/asl/main/install.sh | bash";

  const handleCopy = () => {
    navigator.clipboard.writeText(command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="mt-10 max-w-2xl mx-auto w-full">
      <div className="rounded-xl border border-zinc-800 bg-zinc-950 p-1.5 sm:p-2 shadow-sm hover:border-zinc-700 transition-colors">
        <div className="flex items-center justify-between gap-3 px-3 py-1 font-mono text-xs">
          <div className="flex items-center gap-2 overflow-x-auto py-1 text-zinc-300 scrollbar-none">
            <span className="text-zinc-500 select-none font-semibold">$</span>
            <span className="select-all text-zinc-200 whitespace-nowrap">{command}</span>
          </div>

          <button
            onClick={handleCopy}
            className="flex-shrink-0 flex items-center gap-1.5 rounded-md border border-zinc-800 bg-zinc-900 px-3 py-1.5 text-xs text-zinc-300 hover:border-zinc-700 hover:bg-zinc-800 hover:text-white transition-colors"
            title="Copy install command"
            aria-label="Copy install command"
          >
            {copied ? (
              <>
                <Check className="h-3.5 w-3.5 text-white" />
                <span className="font-medium text-white">Copied</span>
              </>
            ) : (
              <>
                <Copy className="h-3.5 w-3.5 text-zinc-400" />
                <span>Copy</span>
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
};
