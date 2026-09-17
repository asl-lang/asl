# ADR-0012: GitHub Pages Documentation Platform for Agent Skill Language (ASL)

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Revisores**: Architecture Advisory Board & Core Maintainers
- **Crates Afetadas**: `docs/`, `website/`, `.github/workflows/`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade), Axioma 2 (Zero Dependências no Runtime), Axioma 4 (Desacoplamento Hexagonal), Axioma 6 (KV-Cache Invariante), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Motivação (Context and Problem Statement)

The **Agent Skill Language (ASL 3.0 - Omni-Spec)** has evolved into a comprehensive, mathematically grounded ecosystem featuring:
- A formal scientific paper with termination and confinement theorems (`ASL_SCIENTIFIC_PAPER.md`).
- A modular hexagonal Rust runtime with 10 micro-crates (`asl-spec`, `asl-core-traits`, `asl-parser`, `asl-vm-starlark`, `asl-vm-wasm`, `asl-security`, `asl-protocol-mcp`, `asl-protocol-http`, `asl-ffi`, `asl-cli`).
- The canonical file triad (`.skill`, `.tool`, `.asl`) with isolated shadow projection.
- An in-memory semantic declarative rules transpiler (`asl:rules`).
- Native Model Context Protocol (MCP) server support (stdio and HTTP/SSE).
- Cryptographic Ed25519 signatures and KV-Cache static prefix optimization.

### Identified Deficiencies
1. **Discoverability & Developer Onboarding**: Repository Markdown documents, while rigorous, lack an accessible, modern, search-indexed portal for human developers and autonomous agents.
2. **Interactive Experience**: Prospective users cannot experiment with `asl:rules` transpilation, GBNF grammar generation, or KV-cache analysis without cloning and compiling Rust locally.
3. **Scientific Paper Presentation**: The formal proofs, mathematical lemmas, and KaTeX notation in `ASL_SCIENTIFIC_PAPER.md` deserve a first-class reading experience with typography and citations comparable to top-tier computer science conference proceedings (ACM, IEEE, NeurIPS).

---

## 2. Proposta Detalhada da Arquitetura (Detailed Architecture & Design)

We propose establishing the official ASL documentation and interactive portal hosted on **GitHub Pages** (`cadente-hub.github.io/asl`), inspired by the design system, typography, and dark-mode aesthetic of **Next.js** (`nextjs.org`).

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                    ASL DOCUMENTATION PORTAL ARCHITECTURE                     │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  [ Top Navbar ]   Brand (Cadente/ASL) | Docs | Triad | Paper | Playground    │
│                                                                              │
│  ┌───────────────────────┬────────────────────────────────┬───────────────┐  │
│  │     LEFT SIDEBAR      │         CENTER CONTENT         │ RIGHT SIDEBAR │  │
│  │                       │                                │               │  │
│  │  • Getting Started    │  # Agent Skill Language (3.0)  │ On This Page  │  │
│  │  • The Triad Concept  │  [Badge: v0.3.0] [Badge: Rust] │ • Overview    │  │
│  │    - .skill (Modular) │                                │ • Axioms      │  │
│  │    - .tool (MCP tool) │  Interactive Code Switcher:    │ • Benchmarks  │  │
│  │    - .asl (Native)    │  [ .skill | .tool | .asl ]     │ • Citations   │  │
│  │  • Rules Transpiler   │                                │               │  │
│  │  • Hermetic Starlark  │  Callouts / Admonitions:       │ Edit on GitHub│  │
│  │  • Security & OCap    │  > [!NOTE] Invariant Prefix    │ Star on GitHub│  │
│  │  • Formal Paper 🔬    │                                │               │  │
│  │  • CLI Reference      │  LaTeX / Math Formulation:     │               │  │
│  │  • Interactive Wasm   │  \mathcal{V}(S) \le \mathcal{B}│               │  │
│  └───────────────────────┴────────────────────────────────┴───────────────┘  │
│                                                                              │
│  [ Search Dialog ]  Instant Command Palette (⌘K) with client-side indexing   │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Aesthetic & Design Pillars (Inspired by Next.js)
1. **Geist Monochrome Aesthetic**:
   - Dark-mode first palette: deep black background (`#000000`), subtle borders (`#222222`), muted text (`#888888`), bright high-contrast typography (`#FFFFFF`).
   - Monospace typography for identifiers, code blocks, and tokens (`Geist Mono`, `ui-monospace`).
2. **Global Command Menu (`⌘K`)**:
   - Ultra-fast client-side search indexing all topics, API flags, axioms, and theorems without external network round-trips.
3. **Interactive Code Switcher**:
   - Dual-tab code containers displaying the same logical operation written in `.skill`, `.tool`, and `.asl`, with copy buttons and syntax highlighting.
4. **Dedicated Scientific Paper Pavilion**:
   - Standalone academic view mode with dual-column or single-column serif/sans typography, KaTeX formula rendering, downloadable PDF link, and BibTeX citation exporter.
5. **Interactive Wasm Playground (Client-Side)**:
   - Compiling the core `asl-parser` and `asl-vm-starlark` to `wasm32-unknown-unknown` allows live editing of `asl:rules` directly in the browser, showing instant Starlark transpilation and execution results with zero server backend.

### 2.2 Structural Information Architecture
The portal is structured into 5 cohesive tracks:
- **Track 1: Introduction & Overview ("Why ASL?")**:
  - The agentic revolution, problems with loose scripts, and the ASL Triad solution (`.skill`, `.tool`, `.asl`).
  - **Formal Tokenomics & The 93.2% Token Reduction Theorem**:
    - Detailed mathematical breakdown showing how the ReAct exploratory cycle ($\mathcal{T}_{\text{legacy}} \approx 2,100\text{ tokens}$ across doc reading, directory inspection, shell tracebacks, and JSON retry loops) collapses into an atomic, grammar-masked tool invocation ($\mathcal{T}_{\text{ASL}} \approx 142\text{ tokens}$).
    - The three mathematical drivers: (1) Elimination of exploratory tool turns via black-box schemas, (2) Elimination of stochastic JSON retries via AOT GBNF grammar constraints ($0\%$ syntax errors), and (3) $100\%$ KV-Cache hit rate via Axiom 6 (static immutable prefix).
- **Track 2: Programming in ASL**: Literate format, `asl:rules` declarative syntax, Starlark L1 runtime, and Wasm extensions.
- **Track 3: Security & Axioms**: Mark Miller OCap, Lampson confinement, Lamport monotonic fuel, and Ed25519 digital custody.
- **Track 4: The Formal Scientific Paper (`/paper`)**: Unabridged academic paper by Jean Catarina with KaTeX equations and lemmas.
- **Track 5: CLI & Tooling**: Comprehensive flags, commands (`run`, `check`, `expand`, `serve`, `analyze-prefix`), and MCP configurations.

### 2.3 CI/CD Deployment Pipeline
The portal will deploy automatically to GitHub Pages via GitHub Actions (`.github/workflows/deploy-docs.yml`):
- Runs on push to `main` branch when changes occur in `website/` or `docs/`.
- Executes static build export (`next build` / static export).
- Publishes artifacts to GitHub Pages environment (`cadente-hub.github.io/asl`).

---

## 3. Alternativas Consideradas (Alternatives Considered)

| Criteria | Option A: Nextra (Next.js SSG) | Option B: Astro + Starlight | Option C: Docusaurus |
| :--- | :---: | :---: | :---: |
| **Next.js Design Parity** | **Native 100% (Vercel Geist)** | High (via custom CSS) | Moderate (Infima theme) |
| **KaTeX Math Rendering** | ✅ Full KaTeX plugin | ✅ Full KaTeX plugin | ✅ Full KaTeX plugin |
| **Search Experience** | ✅ Instant FlexSearch / ⌘K | ✅ Pagefind / ⌘K | ✅ Algolia / Local Search |
| **Wasm Playground Integration**| ✅ Native React hooks | ✅ Vanilla / React islands | ⚠️ Complex bundling |
| **GitHub Pages Static Export** | ✅ `next build` (static export)| ✅ `astro build` (pure static) | ✅ `docusaurus build` |
| **Build & Maintenance Simplicity**| **Optimal for Vercel/Next aesthetic** | High performance | Heavier dependencies |

### Decision
We choose **Option A: Nextra (Next.js Documentation Framework)** because it directly provides the exact Next.js dark-mode aesthetic, supports React Server Components, KaTeX math rendering, instant ⌘K search, and static HTML export for GitHub Pages without running node servers in production.

---

## 4. Consequências e Trade-offs (Consequences & Trade-offs)

### Positive Consequences
- **Developer Experience**: Interactive web documentation with ⌘K search significantly reduces onboarding friction.
- **Academic Stature**: A beautifully typeset paper section with KaTeX formulas cements ASL's scientific rigor and theoretical foundations.
- **Zero Runtime Interference**: The website code lives isolated in `website/`, having zero footprint on the core Rust engine or binary distribution.
- **Instant Experimentation**: Developers can try ASL rules transpilation directly in the browser via client-side WebAssembly without installing anything.

### Negative Consequences & Mitigations
- **Build Tooling Separation**: Adds Node.js/npm dependencies to CI solely for the documentation build.
  - *Mitigation*: The website dependencies are completely excluded from the Cargo workspace and core Rust repository checkouts.
- **Content Synchronization**: Documentation must remain synchronized with evolving Rust crates and traits.
  - *Mitigation*: Automated doc-tests and CI validation to ensure example snippets in documentation remain bit-for-bit accurate.

---

## 5. Conformidade com os 7 Axiomas do ASL (Axiomatic Compliance)

- [x] **Axioma 1 (Atomicidade do .skill preservada)**: The documentation platform does not introduce companion scripts into `.skill`, `.tool`, or `.asl` files; all units remain strictly atomic.
- [x] **Axioma 2 (Zero dependências externas mantido)**: The core Rust runtime (`asl-cli`, `libasl`) remains 100% pure Rust with zero external interpreter dependencies. The documentation portal is completely separated in `website/`.
- [x] **Axioma 3 (Confinamento ocap sem autoridade ambiente)**: The site showcases and documents OCap handles without granting ambient authority.
- [x] **Axioma 4 (Isolamento hexagonal sem import cruzado de adaptadores)**: Architecture documentation explicitly diagrams ports, traits, and adapters.
- [x] **Axioma 5 (Término determinístico com Fuel Metering)**: The documentation features an interactive fuel simulation demonstrating monotone decreasing execution.
- [x] **Axioma 6 (Prefixo estático imutável / KV-Cache 100%)**: Live prefix analyzer embedded in the documentation calculates KV-cache hit rate.
- [x] **Axioma 7 (Limite cognitivo de contexto < 450 linhas)**: This ADR file contains less than 220 lines, fully conforming to context window limits.

---

## 6. Próximos Passos (Next Steps)

1. Formulate the implementation plan (`docs/plans/0012-github-pages-documentation-platform.md`) via skill `asl-plan`.
2. Scaffold `website/` with Nextra / Next.js static export using the Geist design system.
3. Transcribe and format the formal paper (`/paper`) with KaTeX equations.
4. Establish the GitHub Actions workflow for deployment to `https://cadente-hub.github.io/asl/`.
