# AGENTS.md: Diretrizes Rígidas e Guardrails Universais para IAs no Projeto ASL

> ⚠️ **LEI SUPREMA PARA TODOS OS MODELOS DE LINGUAGEM (LLMs)**:
> Este repositório é governado pelas premissas científicas publicadas em [`ASL_SCIENTIFIC_PAPER.md`](./ASL_SCIENTIFIC_PAPER.md) e pela arquitetura hexagonal estrita de [`ARCHITECTURE.md`](./ARCHITECTURE.md).
> NENHUMA IA tem autorização para violar, contornar ou modificar os axiomas abaixo descritos. Toda e qualquer alteração deve aderir estritamente a estas leis.

---

## 🏛️ Os 7 Axiomas Invioláveis do Projeto ASL

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                          OS 7 AXIOMAS IMUTÁVEIS DO ASL                                 │
├────────────────────────────────────────────────────────────────────────────────────────┤
│ 1. ATOMICIDADE DE DOCUMENTOS ASL: Proibido criar scripts companheiros (.py, .sh, etc.) │
│ 2. ZERO DEPENDÊNCIA DE ECOSSISTEMAS: Proibido depender de Python, Node.js, JVM ou C/C++│
│ 3. OBJECT-CAPABILITY ESTRITO (ocap): Proibido autoridade ambiente ou paths abertos    │
│ 4. DESACOPLAMENTO HEXAGONAL: Adaptadores NUNCA dependem de outros adaptadores          │
│ 5. TÉRMINO DETERMINÍSTICO: Todo loop deve consumir Fuel em nível de opcode             │
│ 6. PREFIXO ESTÁTICO IMUTÁVEL: Preservar 100% de reuso de KV-Cache em arquivos ASL      │
│ 7. LIMITE COGNITIVO DE CONTEXTO: Nenhum arquivo de código pode exceder 400 linhas      │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### Axioma 1: Atomicidade Absoluta de Documentos ASL (`.skill`, `.tool`, `.asl`)
- **Regra**: Toda e qualquer skill ou ferramenta deve ser implementada em um **único arquivo atômico com extensão canônica (`.skill`, `.tool`, `.asl`)**.
- **Proibição Estrita**: É terminantemente proibido criar pastas `scripts/`, `requirements.txt`, `package.json` ou arquivos companheiros. Se uma lógica determinística é necessária, ela deve estar contida no bloco ````asl:deterministic ou ````asl:rules dentro do próprio arquivo.

### Axioma 2: Zero Dependência de Interpretadores Externos
- **Regra**: O runtime do ASL é construído exclusivamente em **Rust puro** gerando binário estático e biblioteca C-ABI (`libasl`).
- **Proibição Estrita**: Nenhuma parte do projeto pode depender de Python instalado na máquina, Node.js, npm, pip, Docker, Java ou compiladores de C no host. O ASL é $100\%$ autossuficiente.

### Axioma 3: Object-Capability (ocap) Sem Autoridade Ambiente
- **Regra**: Nenhuma função determinística tem permissão de chamar primitivas de I/O abertas. Toda operação com o mundo externo deve ocorrer através de handles atenuados injetados em `ctx` (ex: `ctx.fs.confined_dir(root_handle)`).
- **Proibição Estrita**: É proibido permitir caminhos de arquivos relativos ou absolutos que resolvam symlinks para fora dos diretórios autorizados em `confined_read_roots` (Defesa formal contra o Ataque do Vice-Confuso).

### Axioma 4: Arquitetura Hexagonal e Isolamento de Micro-Crates
- **Regra**: O grafo de dependências do Workspace Cargo deve ser rigorosamente acíclico:
  - `asl-spec`: Camada de domínio pura. Zero I/O, zero dependências de rede ou OS.
  - `asl-core-traits`: Apenas definições abstratas de interfaces (`EnginePort`, `ParserPort`, `CapabilityContext`).
  - Adaptadores (`asl-parser`, `asl-security`, `asl-vm-starlark`, `asl-protocol-mcp`): Implementam traits.
- **Proibição Estrita**: Um adaptador **JAMAIS** pode importar diretamente outro adaptador. A comunicação entre módulos ocorre exclusivamente através de injeção de dependências orquestrada pelo `asl-cli`.

### Axioma 5: Término Determinado por Fuel e Invariantes Monotônicas
- **Regra**: Qualquer laço (`bounded_while`) ou iteração em Starlark deve consumir obrigatoriamente $1$ unidade de Fuel por operação elementar de bytecode.
- **Proibição Estrita**: É proibido usar temporizadores de relógio de parede (*wall-clock time*) como única garantia de término. O término deve ser garantido matematicamente pelo esgotamento de Fuel.

### Axioma 6: Otimização de Prefixo Estático (KV-Cache 100%)
- **Regra**: Em qualquer arquivo ASL (`.skill`, `.tool`, `.asl`), o cabeçalho YAML e a seção semântica de instruções devem ser **bit-a-bit idênticos e imutáveis** entre invocações.
- **Proibição Estrita**: Nunca injete timestamps dinâmicos, IDs de sessão ou dados variáveis no topo do arquivo. Parâmetros mutáveis pertencem estritamente aos argumentos de entrada no sufixo de chamada.

### Axioma 7: Limite Cognitivo de Contexto para Agentes de IA
- **Regra**: Nenhum arquivo-fonte no projeto pode exceder **400 linhas de código**.
- **Justificativa**: Garante que qualquer LLM consiga carregar e raciocinar sobre o arquivo inteiro em uma única janela de atenção sem sofrer truncamento ou perda de atenção. Se um arquivo estiver crescendo além de 350 linhas, fatie-o em submódulos ortogonais.

---

## 🔄 O Protocolo Universal Spec-Driven Development (SDD)

Toda IA atuando neste repositório DEVE operar no modo **Spec-Driven Development**. É estritamente proibido iniciar a codificação ou modificar crates sem seguir as 5 etapas abaixo (meta-skill `asl-spec-driven`):

```
1. ESPECIFICAÇÃO DETALHADA (ADR) ──► docs/adrs/NNNN-<nome>.md (skill `asl-adr`)
2. PLANO EXAUSTIVO EM FASES      ──► docs/plans/NNNN-<nome>.md (skill `asl-plan`)
3. IMPLEMENTAÇÃO ATÔMICA DA FASE ──► runtime/crates/<crate>/ (com código e testes)
4. VERIFICAÇÃO AUTOMATIZADA      ──► ./scripts/guardrail_check.sh (100% aprovado)
5. FINALIZAÇÃO DA FASE           ──► git commit -m "..." && git push origin main
```

Nenhuma nova funcionalidade, alteração de contratos em `asl-core-traits`, novo adaptador ou expansão de sintaxe pode pular este ciclo.

---

## 🧪 Padrão de Organização e Estratégia de Testes (Rust)

Seguindo as melhores práticas de grandes projetos Rust maduros, a **proximidade entre código e teste é o padrão**. A separação física só ocorre quando escala, legibilidade ou a própria natureza do teste (integração/E2E) justificarem.

### 1. Modelo Mental Decisório

```
1. Lógica pequena / isolada          ──► Teste unitário no mesmo arquivo (`#[cfg(test)] mod tests`)
2. Volume alto de testes no módulo   ──► Módulo separado adjacente (`foo/tests.rs` com `#[cfg(test)] mod tests;`)
3. Interação entre módulos / API pub ──► Diretório `tests/` na raiz da crate
4. Fluxo completo / E2E / Sistema    ──► Suíte dedicada de integração/E2E (ex: `asl-cli/tests/`)
```

### 2. Princípios Operacionais para Agentes de IA

- **Proximidade por Padrão**: Testes unitários concisos devem ficar no mesmo arquivo via `#[cfg(test)] mod tests { ... }`. NUNCA crie arquivos separados de teste por mero automatismo ou reflexo estético.
- **Extração por Sobrecarga Cognitiva**: Extraia testes unitários para um módulo separado adjacente (ex: `foo/tests.rs` ou `foo_tests.rs`, mantendo `#[cfg(test)] mod tests;` no módulo original) apenas quando seu volume prejudicar a leitura do código principal ou aproximar o arquivo do limite de 400 linhas (Axioma 7).
- **Escopo e Visibilidade**:
  - *Testes Unitários*: Podem e devem testar detalhes internos/privados quando útil.
  - *Testes em `tests/`*: Devem preferencialmente validar comportamento observável através da API pública da crate (visão de consumidor externo / black-box).
- **Invariantes sobre Implementação Efêmera**: Priorize testes que garantam contratos de comportamento, invariantes e saídas determinísticas, evitando acoplamento excessivo com detalhes efêmeros de implementação interna.
- **Sem Duplicação Artificial**: Não duplique o mesmo cenário em testes unitários e de integração sem um motivo técnico concreto.
- **Reutilização de Fixtures e Mocks**: Mantenha fixtures, mocks e helpers compartilhados (como `MockSecurityContext`, `helpers.rs`) organizados e reutilizáveis quando começarem a aparecer em múltiplos testes.
- **Preservação de Padrões Existentes**: Não mova nem refatore testes consolidados existentes apenas para uniformizar visualmente a árvore se não houver ganho real.
- **Localidade de Atualização**: Toda mudança em código existente deve, quando aplicável, atualizar ou adicionar os testes mais próximos do nível onde o comportamento foi alterado.

### 3. Exemplo Mínimo de Estrutura de Diretórios

```
runtime/crates/<crate-name>/
├── src/
│   ├── lib.rs              # Código + #[cfg(test)] mod tests { ... } (padrão para testes unitários)
│   ├── parser.rs           # #[cfg(test)] mod tests; (quando testes crescerem muito)
│   └── parser/
│       └── tests.rs        # Testes unitários extraídos para preservar a leitura do módulo principal
└── tests/                  # Apenas para contratos públicos e integração entre componentes
    └── integration_test.rs # Validação black-box multi-módulos e fluxos de API pública
```

---

## 🛠️ Checklist Obrigatório Antes de Submeter Qualquer Alteração

Toda IA que realizar modificações neste projeto DEVE executar e verificar com sucesso:
1. `cargo test`: Todos os testes unitários e de integração devem passar com 0 falhas (respeitando o padrão de organização de testes acima).
2. `asl check <arquivo>`: Todos os arquivos ASL (`.skill`, `.tool`, `.asl`) devem validar com digest SHA-256 canônico correto.
3. `cargo clippy`: Zero advertências de linter ou código inseguro (*unsafe* sem barreira).
4. Verificação de acoplamento: Nenhuma nova dependência cruzada entre adaptadores foi introduzida.
5. Propostas Arquiteturais (`docs/adrs/`): Toda mudança estrutural ou novo componente deve ter seu ADR detalhado via skill `asl-adr`.
6. Planos de Implementação (`docs/plans/`): Todo plano deve ser decomposto em fases pequenas com exemplos exaustivos de código, encerrando cada fase com commit convencional e push na main via skill `asl-plan`.
