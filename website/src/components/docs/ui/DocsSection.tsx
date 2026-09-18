import React from "react";

interface DocsSectionProps {
  title?: string | React.ReactNode;
  subtitle?: string;
  badge?: string;
  badgeColor?: "emerald" | "blue" | "purple" | "amber" | "rose" | "zinc";
  children: React.ReactNode;
  className?: string;
}

export const DocsSection: React.FC<DocsSectionProps> = ({
  title,
  subtitle,
  badge,
  badgeColor = "blue",
  children,
  className = "",
}) => {
  const badgeClasses: Record<string, string> = {
    emerald: "bg-emerald-500/10 border-emerald-500/20 text-emerald-400",
    blue: "bg-blue-500/10 border-blue-500/20 text-blue-400",
    purple: "bg-purple-500/10 border-purple-500/20 text-purple-400",
    amber: "bg-amber-500/10 border-amber-500/20 text-amber-400",
    rose: "bg-rose-500/10 border-rose-500/20 text-rose-400",
    zinc: "bg-zinc-800 border-zinc-700 text-zinc-300",
  };

  return (
    <section className={`space-y-4 border-t border-zinc-800 pt-8 ${className}`}>
      {title && (
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-xl font-bold text-white tracking-tight">{title}</h2>
            {subtitle && <p className="text-sm text-zinc-400 mt-1">{subtitle}</p>}
          </div>
          {badge && (
            <span
              className={`text-[10px] font-mono rounded border px-2 py-0.5 ${
                badgeClasses[badgeColor] || badgeClasses.blue
              }`}
            >
              {badge}
            </span>
          )}
        </div>
      )}
      {children}
    </section>
  );
};
