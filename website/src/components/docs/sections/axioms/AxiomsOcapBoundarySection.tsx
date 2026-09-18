import React from "react";
import { AslCodeBlock } from "@/components/docs/ui/AslCodeBlock";

export const AxiomsOcapBoundarySection: React.FC = () => {
  const ocapSnippet = `capabilities:
  fs:
    confined_read_roots: ["./safe-directory"]
  crypto:
    allowed_algorithms: ["sha256", "ed25519"]
limits:
  max_fuel_opcodes: 100000`;

  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        The OCap Security Boundary
      </h2>
      <p>
        In traditional Python or Bash scripts, importing <code className="text-zinc-200">os</code> or <code className="text-zinc-200">subprocess</code> grants ambient authority to read any file on the drive or open network sockets. In ASL, code runs inside a sealed sandbox:
      </p>

      <AslCodeBlock lang="asl" code={ocapSnippet} />

      <p className="text-xs text-zinc-400">
        Attempting to access paths outside <code className="text-zinc-200 font-mono">confined_read_roots</code> or execute system binaries triggers an immediate OCap confinement trap.
      </p>
    </div>
  );
};
