# Plano de Implementação: Plataforma de Documentação no GitHub Pages para ASL

- **ADR Vinculado**: `docs/adrs/0012-github-pages-documentation-platform.md`
- **Status**: Concluído (5/5 Fases - 100%)
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Meta**: Construir a plataforma oficial de documentação e laboratório interativo do ASL 3.0 no GitHub Pages (`cadente-hub.github.io/asl`) inspirada no design system Geist do Next.js, contendo as 5 trilhas canônicas de conteúdo, pavilhão acadêmico do artigo científico com KaTeX, playground de simulação de regras em memória e esteira automatizada de CI/CD.

---

## Fase 1: Fundação Next.js e Design System Geist (`website/`)
Scaffolding do portal estático em `website/` com Next.js, TypeScript, Tailwind CSS, paleta Geist monocromática (dark-mode first `#000000`), fontes tipográficas de alta legibilidade e configuração de exportação estática (`output: 'export'`) com suporte ao basePath `/asl`.
- Validação: `npm install && npm run build`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 2: Navegação, Busca Instantânea ⌘K e Seletor de Código da Tríade
Implementação da Top Navbar com branding Cadente/ASL, modal de busca rápida ⌘K indexando tópicos, axiomas e comandos da CLI, e componente interativo `CodeSwitcher` para comparar `.skill`, `.tool` e `.asl`.
- Validação: `npm run build`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 3: Trilhas de Conteúdo: Visão Geral, Teorema dos 93.2% de Tokens e CLI
Construção das páginas e rotas de documentação:
- Visão Geral e motivação ("Why ASL?").
- Teorema da Redução de 93.2% de Tokens com detalhamento matemático do ciclo ReAct vs AOT.
- A Tríade Canônica e regras de Projeção Sombra.
- Sintaxe declarativa `asl:rules`.
- Os 7 Axiomas de Arquitetura e Modelo OCap.
- Referência completa da CLI e servidor MCP.
- Validação: `npm run build`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 4: Pavilhão Acadêmico do Artigo Científico com KaTeX (`/paper`)
Implementação da rota acadêmica `/paper` renderizando o artigo formal de Jean Catarina (`ASL_SCIENTIFIC_PAPER.md`) com notação matemática KaTeX, definições formais, lemas, provas, tabelas comparativas e exportador de citação BibTeX.
- Validação: `npm run build`
- Finalização: Commit & Push com `git push origin main`.

---

## Fase 5: Playground Interativo no Navegador e Pipeline CI/CD no GitHub Actions
Criação do simulador interativo em `/playground` permitindo testar `asl:rules`, ver a transpilação para Starlark L1 e analisar o prefixo invariante de KV-cache no navegador. Configuração do fluxo de GitHub Actions `.github/workflows/deploy-docs.yml` para publicação contínua no GitHub Pages.
- Validação: `npm run build && ./scripts/guardrail_check.sh`
- Finalização: Commit & Push com `git push origin main`.
