import React from "react";

interface DocsHeaderProps {
  category: string;
  title: string | React.ReactNode;
  description: string;
}

export const DocsHeader: React.FC<DocsHeaderProps> = ({ category, title, description }) => {
  return (
    <div>
      <div className="text-xs font-mono font-medium text-zinc-400 uppercase tracking-wider">
        {category}
      </div>
      <h1 className="text-3xl sm:text-4xl font-bold tracking-tight text-white mt-1">
        {title}
      </h1>
      <p className="text-sm text-zinc-400 mt-2 leading-relaxed max-w-3xl">
        {description}
      </p>
    </div>
  );
};
