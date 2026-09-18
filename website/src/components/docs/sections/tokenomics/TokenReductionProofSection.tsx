import React from "react";
import { MathBlock } from "@/components/MathBlock";

export const TokenReductionProofSection: React.FC = () => {
  return (
    <div className="space-y-4 pt-4">
      <h2 className="text-xl font-bold text-white tracking-tight">
        3. Mathematical Proof of Token Reduction
      </h2>
      <p className="text-sm text-zinc-300 leading-relaxed">
        Let <MathBlock math={`\\Delta_{\\text{tokens}}`} /> be the fractional reduction in tokens per automation cycle:
      </p>

      <div className="rounded-xl border border-zinc-800 bg-black p-6 text-center">
        <MathBlock
          block
          math={`\\Delta_{\\text{tokens}} = \\left( 1 - \\frac{\\mathcal{T}_{\\text{ASL}}}{\\mathcal{T}_{\\text{Legacy}}} \\right) \\times 100\\%`}
        />
        <MathBlock
          block
          math={`\\Delta_{\\text{tokens}} = \\left( 1 - \\frac{142}{2{,}100} \\right) \\times 100\\% = 93.238\\% \\approx \\mathbf{93.2\\%}`}
        />
      </div>
    </div>
  );
};
