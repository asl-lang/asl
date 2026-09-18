---
name: nextjs-architect
description: >-
  Audita, refatora e mantém a arquitetura modular do website Next.js App Router.
  Garante páginas enxutas (< 70 linhas), componentes decompostos em seções,
  blocos de código 100% em sintaxe ASL nativa e build estático sem erros.
---

# Skill: Next.js Documentation Architect

Esta skill estabelece e audita as diretrizes arquiteturais para o website e documentação do ASL em Next.js (App Router). Seu objetivo é impedir a degradação de arquivos em monólitos de conteúdo estático e assegurar manutenção modular.

---

## 1. Princípios Arquiteturais Obrigatórios

### 1.1 `page.tsx` como Ponto de Composição Enxuto (< 70 Linhas)
Cada arquivo `app/**/page.tsx` no App Router deve atuar exclusivamente como **raiz de composição**:
- Importa o cabeçalho (`DocsHeader`), o rodapé de navegação (`DocsNavFooter`) e as seções correspondentes.
- Não deve conter tabelas estáticas embutidas, matrizes JSON, strings de código de centenas de linhas ou marcação repetitiva.
- **Limite rígido**: `wc -l page.tsx` deve ser rigorosamente inferior a 70 linhas (meta ideal: 25 a 40 linhas).

### 1.2 Separação de Componentes e Primitivas
- **Primitivas Compartilhadas (`components/docs/ui/`)**:
  - `DocsHeader.tsx`: Título, categoria monocromática/destaque e resumo da página.
  - `DocsNavFooter.tsx`: Links de próximo/anterior com setas padronizadas.
  - `DocsSection.tsx`: Container padronizado com bordas, títulos, subtítulos e badges.
  - `AslCodeBlock.tsx`: Renderizador padronizado de blocos de código (`asl`, `asl:rules`, `asl:deterministic`).
- **Seções Específicas de Domínio (`components/docs/sections/<feature>/`)**:
  - Cada tópico ou subtítulo conceitual vira um componente isolado (`*Section.tsx`).
  - Cada arquivo de seção deve ter responsabilidade única e menos de 150 linhas.

### 1.3 Sintaxe 100% ASL Nativa na Documentação
- Todos os exemplos de código exibidos na UI devem utilizar estritamente a sintaxe da linguagem semântica ASL (`asl:rules`, `asl:deterministic` ou arquivo `.skill` completo).
- Menções a Starlark L1 devem ser limitadas a detalhes de compatibilidade binária do motor de execução, nunca como linguagem primária de autoria.

---

## 2. Roteiro de Auditoria e Validação

Ao criar ou editar qualquer página na documentação, execute o checklist:

### Passo 1: Verificar Limite de Linhas das Páginas
```bash
wc -l website/src/app/docs/**/page.tsx website/src/app/docs/page.tsx
```
*Critério*: Nenhuma página pode ultrapassar 70 linhas. Se ultrapassar, quebre em seções dentro de `components/docs/sections/<página>/`.

### Passo 2: Validar Build Estático
```bash
cd website && npm run build
```
*Critério*: Todas as 19+ rotas estáticas devem compilar com sucesso sem erros de tipagem TypeScript ou de renderização.

### Passo 3: Validar Guardrails Globais do Repositório
```bash
./scripts/guardrail_check.sh
```
*Critério*: Todos os 5 níveis de guardrails do ASL devem ser aprovados (Axioma 7 de contexto cognitivo, grafo acíclico, testes de rust, etc).

---

## 3. Exemplo de Composição Canônica (`page.tsx`)

```tsx
import React from "react";
import { DocsHeader } from "@/components/docs/ui/DocsHeader";
import { DocsNavFooter } from "@/components/docs/ui/DocsNavFooter";
import { FeatureSectionOne } from "@/components/docs/sections/my-feature/FeatureSectionOne";
import { FeatureSectionTwo } from "@/components/docs/sections/my-feature/FeatureSectionTwo";

export default function MyFeatureDocsPage() {
  return (
    <div className="space-y-12">
      <DocsHeader
        category="Language Reference • Section N"
        title="My Feature Title"
        description="Clear, concise architectural description of this module."
      />

      <FeatureSectionOne />
      <FeatureSectionTwo />

      <DocsNavFooter
        prev={{ title: "Previous Section", href: "/docs/prev" }}
        next={{ title: "Next Section", href: "/docs/next" }}
      />
    </div>
  );
}
```
