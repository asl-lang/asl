import React from "react";

interface AslCodeBlockProps {
  code: string;
  lang?: "asl" | "asl:rules" | "asl:deterministic" | "bash" | "yaml";
  filename?: string;
}

export const AslCodeBlock: React.FC<AslCodeBlockProps> = ({
  code,
  lang = "asl:deterministic",
  filename,
}) => {
  return (
    <div className="rounded-xl border border-zinc-800 bg-[#0d0d0d] overflow-hidden">
      {(filename || lang) && (
        <div className="flex items-center justify-between border-b border-zinc-850 px-4 py-2 bg-zinc-950/60 font-mono text-[11px] text-zinc-400">
          <span>{filename || "Code Block"}</span>
          <span className="rounded bg-zinc-800/80 border border-zinc-700/60 px-1.5 py-0.5 text-[9px] text-zinc-300">
            {lang}
          </span>
        </div>
      )}
      <div className="p-4 font-mono text-xs text-zinc-300 overflow-x-auto leading-relaxed">
        <pre>{code}</pre>
      </div>
    </div>
  );
};
