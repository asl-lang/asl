import React from "react";
import { Sidebar } from "@/components/Sidebar";

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
      <div className="flex gap-8">
        <Sidebar />
        <article className="flex-1 py-10 min-w-0 max-w-4xl">
          {children}
        </article>
      </div>
    </div>
  );
}
