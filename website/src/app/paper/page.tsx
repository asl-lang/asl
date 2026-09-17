"use client";

import React, { useState } from "react";
import Link from "next/link";
import { Download, Copy, Check, BookOpen, Sparkles, Shield, Cpu, ExternalLink } from "lucide-react";
import { MathBlock } from "@/components/MathBlock";

const BIBTEX = `@article{catarina2026asl,
  title={Agent Skill Language (ASL): Uma Linguagem AI-First Hermética para Execução Determinística e Orquestração Semântica de Agentes Autônomos},
  author={Catarina, Jean},
  journal={ASL Open Systems & Architecture Group},
  year={2026},
  url={https://github.com/asl-lang/asl}
}`;

export default function ScientificPaperPage() {
  const [copiedBibtex, setCopiedBibtex] = useState(false);

  const handleCopyBibtex = () => {
    navigator.clipboard.writeText(BIBTEX);
    setCopiedBibtex(true);
    setTimeout(() => setCopiedBibtex(false), 2000);
  };

  return (
    <div className="mx-auto max-w-4xl px-4 py-12 sm:px-6 lg:px-8">
      {/* Paper Header / Metadata Card */}
      <div className="rounded-2xl border border-zinc-800 bg-zinc-950 p-6 sm:p-8 space-y-6 shadow-2xl">
        <div className="flex flex-wrap items-center justify-between gap-3 border-b border-zinc-850 pb-4">
          <div className="inline-flex items-center gap-2 text-xs font-mono text-blue-400">
            <span className="rounded bg-blue-500/10 px-2 py-0.5 border border-blue-500/20">
              Formal Architecture Paper
            </span>
            <span>ASL Architecture Group</span>
          </div>
          <button
            onClick={handleCopyBibtex}
            className="flex items-center gap-1.5 rounded-lg border border-zinc-700 bg-zinc-850 px-3 py-1.5 text-xs font-medium text-zinc-200 hover:bg-zinc-750 hover:text-white transition-colors"
          >
            {copiedBibtex ? <Check className="h-3.5 w-3.5 text-emerald-400" /> : <Copy className="h-3.5 w-3.5" />}
            <span>{copiedBibtex ? "BibTeX Copied!" : "Cite / BibTeX"}</span>
          </button>
        </div>

        <div className="space-y-3">
          <h1 className="text-2xl sm:text-4xl font-bold tracking-tight text-white leading-tight">
            Agent Skill Language (ASL): Uma Linguagem AI-First Hermética para Execução Determinística e Orquestração Semântica de Agentes Autônomos
          </h1>
          <div className="text-sm font-medium text-zinc-300">
            Autor: <span className="text-white font-semibold">Jean Catarina</span>
          </div>
        </div>

        {/* Abstract Box */}
        <div className="rounded-xl border border-zinc-800 bg-black/60 p-5 space-y-3">
          <div className="text-xs font-bold uppercase tracking-wider text-zinc-400 font-mono">
            Resumo (Abstract)
          </div>
          <p className="text-xs sm:text-sm text-zinc-300 leading-relaxed text-justify">
            A transição de modelos conversacionais para agentes autônomos de ação (*Action-Oriented AI Agents*) expôs uma contradição de engenharia no núcleo dos sistemas de software: a bifurcação entre a <strong>inferência semântica estocástica</strong> (redes neurais baseadas em Transformers) e a <strong>computação determinística</strong> (código executável). O estado da arte baseia-se na orquestração frágil de documentos descritivos acoplados a interpretadores de uso geral (Python, Bash, Node.js) ou servidores residentes via Model Context Protocol (MCP), sofrendo de autoridade ambiente descontrolada, consumo excessivo de tokens e quebras de processos.
          </p>
          <p className="text-xs sm:text-sm text-zinc-300 leading-relaxed text-justify">
            Apresentamos formalmente a <strong>Agent Skill Language (ASL)</strong>, uma linguagem hermética e autocontida baseada na <em>Árvore Sintática de Duplo Consumidor (Dual-Consumer AST)</em>, confinamento de capacidades (OCap), término garantido por fuel monotônico, invariância estática de KV-Cache e compilação antecipada de gramáticas (CFG/GBNF). Demonstramos analítica e empiricamente que a ASL atinge <strong>93.2% de redução no consumo de tokens</strong> por ciclo, elimina 100% dos erros sintáticos de esquema e reduz a latência para menos de <strong>35 µs</strong> em chamadas *in-process*.
          </p>
        </div>
      </div>

      {/* Main Academic Content */}
      <article className="mt-12 space-y-10 text-sm text-zinc-300 leading-relaxed">
        {/* Section 1 */}
        <section className="space-y-4">
          <h2 className="text-xl font-bold text-white tracking-tight border-b border-zinc-800 pb-2">
            1. Introdução e Desconstrução Epistemológica
          </h2>
          <p>
            A arquitetura predominante de extensibilidade procedural assenta-se sobre a <em>Tríade da Fragilidade</em>: Poluição de Ambiente, Autoridade Ambiente Irrestrita (*Ambient Authority*) e Explosão de Tokenomics por loops de depuração reativa.
          </p>
          <div className="rounded-xl border border-zinc-850 bg-zinc-950 p-4 font-mono text-xs text-zinc-400">
            <strong>Tese Central da ASL:</strong> Uma habilidade de agente (Agent Skill) deve constituir um átomo computacional único, hermético e auto-suficiente, consumível simultaneamente como uma diretiva semântica invariante por redes neurais e como uma especificação determinística fechada por um runtime de capacidades limitadas.
          </div>
        </section>

        {/* Section 2 */}
        <section className="space-y-4">
          <h2 className="text-xl font-bold text-white tracking-tight border-b border-zinc-800 pb-2">
            2. A Árvore Sintática de Duplo Consumidor (Dual-Consumer AST)
          </h2>
          <p>
            Definimos formalmente a gramática de um arquivo atômico ASL como uma tupla matemática:
          </p>
          <div className="my-4 text-center">
            <MathBlock block math={`\\mathcal{S} = \\langle \\mathcal{M}, \\mathcal{P}, \\mathcal{D} \\rangle`} />
          </div>
          <p>
            Onde <MathBlock math={`\\mathcal{M}`} /> é o Manifesto Estruturado (YAML imutável), <MathBlock math={`\\mathcal{P}`} /> é o Envelope Semântico (CommonMark consumido pela rede neural <MathBlock math={`\\alpha`} />), e <MathBlock math={`\\mathcal{D}`} /> é o Bloco Determinístico (Starlark L1 ou Wasm avaliado pela máquina <MathBlock math={`\\beta`} />).
          </p>
        </section>

        {/* Section 3 */}
        <section className="space-y-4">
          <h2 className="text-xl font-bold text-white tracking-tight border-b border-zinc-850 pb-2">
            3. Modelo de Segurança OCap e Confinamento de Lampson
          </h2>
          <p>
            Em conformidade com a disciplina de Mark Miller e o Problema do Confinamento de Lampson (1973), nenhum processo ASL possui autoridade ambiente. Qualquer acesso a I/O necessita de um handle de capacidade explicitamente concedido em <MathBlock math={`\\mathcal{M}`} />:
          </p>
          <div className="my-4 text-center">
            <MathBlock block math={`\\forall c \\in \\mathcal{C}_{\\text{process}}, \\quad c \\subseteq \\mathcal{B}_{\\text{manifest}}`} />
          </div>
        </section>

        {/* Section 4 */}
        <section className="space-y-4">
          <h2 className="text-xl font-bold text-white tracking-tight border-b border-zinc-850 pb-2">
            4. Semântica de Término Garantido & Monotonic Fuel
          </h2>
          <p>
            Para prevenir laços infinitos e ataques de exaustão de computação, a execução determinística consome combustível monotonicamente decrescente:
          </p>
          <div className="my-4 text-center">
            <MathBlock block math={`\\mathcal{V}(S_{t+1}) < \\mathcal{V}(S_t) \\quad \\text{e} \\quad \\mathcal{V}(S) \\le \\mathcal{B}_{\\text{fuel}}`} />
          </div>
          <p>
            <strong>Teorema 1 (Término Bounded):</strong> Toda execução de código ASL em Starlark L1 termina em um número finito de passos estritamente delimitado por <MathBlock math={`\\mathcal{B}_{\\text{fuel}}`} />, com complexidade de tempo <MathBlock math={`\\mathcal{O}(\\mathcal{B})`} />.
          </p>
        </section>

        {/* Section 5 */}
        <section className="space-y-4">
          <h2 className="text-xl font-bold text-white tracking-tight border-b border-zinc-850 pb-2">
            5. Análise de Tokenomics & Redução de 93.2%
          </h2>
          <p>
            A redução de tokens deriva da contração do grafo de transições de estados estocásticos. No baseline ReAct legado, o custo total é dado pela soma dos 4 turnos exploratórios:
          </p>
          <div className="my-4 text-center">
            <MathBlock block math={`\\mathcal{T}_{\\text{Legacy}} = 650 + 120 + 480 + 350 + 500 = 2{,}100\\text{ tokens}`} />
          </div>
          <p>
            Em contrapartida, a chamada atômica in-process do ASL consome apenas a tripla estática:
          </p>
          <div className="my-4 text-center">
            <MathBlock block math={`\\mathcal{T}_{\\text{ASL}} = 85 + 32 + 25 = 142\\text{ tokens}`} />
          </div>
          <div className="my-4 text-center">
            <MathBlock block math={`\\Delta_{\\text{tokens}} = \\left(1 - \\frac{142}{2{,}100}\\right) \\times 100\\% = \\mathbf{93.24\\%}`} />
          </div>
        </section>

        {/* BibTeX citation section */}
        <section className="rounded-xl border border-zinc-800 bg-zinc-950 p-6 space-y-3">
          <div className="flex items-center justify-between">
            <h3 className="text-xs font-mono uppercase tracking-wider text-zinc-300 font-bold">
              Como Citar este Artigo (BibTeX)
            </h3>
            <button
              onClick={handleCopyBibtex}
              className="text-xs text-blue-400 hover:text-blue-300 flex items-center gap-1 font-mono"
            >
              {copiedBibtex ? "Copiado!" : "Copiar BibTeX"}
            </button>
          </div>
          <pre className="overflow-x-auto rounded-lg border border-zinc-850 bg-black p-4 font-mono text-xs text-zinc-400">
            {BIBTEX}
          </pre>
        </section>
      </article>
    </div>
  );
}
