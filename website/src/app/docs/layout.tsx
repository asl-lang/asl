import React from "react";
import { Sidebar, DocsMobileNav } from "@/components/Sidebar";

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="w-full">
      {/* Mobile Sub-Navigation Bar (Top of Docs on Mobile) */}
      <DocsMobileNav />

      {/* Main Container: block on mobile (100% width), flex on desktop (>= lg) */}
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="lg:flex lg:gap-10">
          {/* Desktop Left Sidebar */}
          <Sidebar />

          {/* Main Article Content - 100% width on mobile */}
          <main className="w-full min-w-0 max-w-4xl flex-1 py-6 lg:py-10">
            {children}
          </main>
        </div>
      </div>
    </div>
  );
}
