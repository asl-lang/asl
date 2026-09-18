import React from "react";
import { CodeSwitcher } from "@/components/CodeSwitcher";

export const InteractiveTriadSection: React.FC = () => {
  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        Interactive Triad Comparison
      </h2>
      <p className="text-xs text-zinc-400">
        Click between the tabs below to inspect how the same intent is structured across the three canonical formats:
      </p>

      <CodeSwitcher />
    </div>
  );
};
