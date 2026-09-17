"use client";

import React, { useState } from "react";
import { Navbar } from "@/components/Navbar";
import { Footer } from "@/components/Footer";
import { CommandMenu } from "@/components/CommandMenu";

export const AppShell: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  return (
    <div className="min-h-screen flex flex-col bg-black text-zinc-100 selection:bg-zinc-800 selection:text-white antialiased">
      <Navbar onOpenSearch={() => setIsSearchOpen(true)} />
      <CommandMenu isOpen={isSearchOpen} onClose={() => setIsSearchOpen(false)} />
      <main className="flex-1">{children}</main>
      <Footer />
    </div>
  );
};
