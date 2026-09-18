import React from "react";
import Link from "next/link";
import { ArrowRight } from "lucide-react";

interface NavTarget {
  title: string;
  href: string;
}

interface DocsNavFooterProps {
  prev?: NavTarget;
  next?: NavTarget;
}

export const DocsNavFooter: React.FC<DocsNavFooterProps> = ({ prev, next }) => {
  return (
    <div className="pt-6 flex items-center justify-between border-t border-zinc-800 pb-8">
      {prev ? (
        <Link
          href={prev.href}
          className="flex items-center gap-1.5 text-xs font-semibold text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          ← {prev.title}
        </Link>
      ) : (
        <div />
      )}
      {next ? (
        <Link
          href={next.href}
          className="flex items-center gap-1.5 text-xs font-semibold text-blue-400 hover:text-blue-300 transition-colors"
        >
          <span>{next.title}</span>
          <ArrowRight className="h-3.5 w-3.5" />
        </Link>
      ) : (
        <div />
      )}
    </div>
  );
};
