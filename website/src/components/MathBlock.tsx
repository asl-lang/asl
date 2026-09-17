"use client";

import React, { useMemo } from "react";
import katex from "katex";

interface MathBlockProps {
  math: string;
  block?: boolean;
  className?: string;
}

export const MathBlock: React.FC<MathBlockProps> = ({
  math,
  block = false,
  className = "",
}) => {
  const html = useMemo(() => {
    try {
      return katex.renderToString(math, {
        displayMode: block,
        throwOnError: false,
      });
    } catch (e) {
      console.error("KaTeX error:", e);
      return math;
    }
  }, [math, block]);

  if (block) {
    return (
      <div
        className={`my-4 overflow-x-auto py-2 text-center text-zinc-100 ${className}`}
        dangerouslySetInnerHTML={{ __html: html }}
      />
    );
  }

  return (
    <span
      className={`inline-block text-zinc-100 ${className}`}
      dangerouslySetInnerHTML={{ __html: html }}
    />
  );
};
