# ADR-0003: Compilador AOT de Gramáticas de Amostragem LLM (GBNF / Regex-CFG)

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Conselho de Arquitetura ASL / Cadente
- **Crates Afetadas**: `asl-core-traits`, `asl-parser`, `asl-cli`

---

## 1. Contexto e Problema

Nos fluxos convencionais de agentes de IA baseados em LLMs, chamadas de ferramentas (*tool use*) e extrações estruturadas falham frequentemente por erros sintáticos na geração de JSON (chaves ausentes, vírgulas residuais, campos com tipos incompatíveis, strings não terminadas). Abordagens tradicionais tentam mitigar isso através de:
1. **Pós-validação com retentativa (Retry Loops)**: Consome latência adicional e tokens desnecessários em nova passagem de inferência.
2. **Prompts defensivos com Few-Shot**: Aumentam o tamanho do contexto e não oferecem garantias matemáticas de integridade.

No paradigma de engenharia concebido para o **ASL 3.0**, a interface de uma skill (`interface.input_schema` e `interface.output_schema`) define um contrato formal estrito em JSON Schema. Em vez de torcer para que o modelo amostre tokens válidos, os motores modernos de inferência (*llama.cpp*, *Ollama*, *vLLM*, *SGLang*, *Outlines*) implementam **Constrained Decoding via Logit Masking**.

O gap identificado era que o runtime do ASL possuía apenas stubs na crate `asl-parser`, sem compilar ativamente esquemas JSON arbitrários em gramáticas formais prontas para consumo imediato pelo motor de inferência.

---

## 2. Proposta Detalhada da Decisão

Implementar no adaptador de sintaxe (`asl-parser`) a compilação antecipada (*Ahead-Of-Time - AOT*) dos esquemas JSON de entrada e saída em dois alvos formais canônicos:

1. **GBNF (GGML BNF)**: Utilizado por *llama.cpp*, *Ollama* e runtimes baseados em GGUF. Gera uma gramática livre de contexto estrita onde regras definem terminais, não-terminais e alternativas.
2. **Regex DFA (Autômato Finito Determinístico)**: Utilizado por *vLLM*, *SGLang*, *Outlines* e APIs com suporte a regex-guided generation.

### 2.1 Interface Hexagonal Mantida (`asl-core-traits`)

A trait `GrammarCompilerPort` existente em `asl-core-traits` define o contrato:

```rust
pub trait GrammarCompilerPort: Send + Sync {
    fn compile_to_gbnf(&self, json_schema: &serde_json::Value) -> Result<String>;
    fn compile_to_regex_cfg(&self, json_schema: &serde_json::Value) -> Result<String>;
}
```

### 2.2 Arquitetura Modular de Compilação

Para preservar o Axioma 7 (< 400 linhas por arquivo), o subsistema é modularizado em `asl-parser`:

```
runtime/crates/asl-parser/src/
├── lib.rs                 # Parser CommonMark/YAML e re-exports
└── grammar/
    ├── mod.rs             # Adaptador de fachada e testes de integração
    ├── gbnf.rs            # Compilador de JSON Schema para regras GBNF
    └── regex_cfg.rs       # Compilador de JSON Schema para expressões regulares DFA
```

### 2.3 Regras de Compilação Semântica

- **Tipos Primitivos**: `string`, `number`, `integer`, `boolean`, `null` são mapeados para terminais JSON válidos com tratamento canônico de whitespace (`ws`).
- **Enums de String**: Mapeados para alternativas literais exatas (`"val1"` | `"val2"`), eliminando qualquer token fora do domínio permitido.
- **Objetos Estruturados**: Campos em `properties` declarados em `required` são dispostos em ordem canônica com pontuação estrita de vírgulas e chaves. Propriedades opcionais são tratadas com modificadores de opcionalidade (`?`).
- **Arrays**: Itens tipados com regras repetíveis delimitadas por colchetes e vírgulas.
- **Esquemas Abertos/Genéricos**: Schemas sem propriedades explícitas geram a gramática canônica de qualquer JSON válido.

---

## 3. Alternativas Consideradas

- **Alternativa A: Validação Post-Hoc em Runtime com Erro Semântico**:
  - *Descarte*: Não previne a alocação de tokens inválidos no LLM e desperdiça poder computacional do agente. Viola a premissa de 0.0% de erro sintático na inferência.
- **Alternativa B: Delegar a compilação para ferramentas externas em Python (ex: outlines/jsonschema)**:
  - *Descarte*: Quebraria o Axioma 2 (Zero dependências externas e runtime hermético e autocontido em Rust).
- **Alternativa C: Compilador AOT Nativo em Rust integrado ao `asl-parser` (Escolhida)**:
  - *Justificativa*: Zero dependências externas, execução ultraveloz em microssegundos, desacoplado via porta hexagonal e utilizável tanto via CLI (`asl compile-grammar`) quanto via C-ABI (`libasl`).

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - **Zero Erro Sintático**: LLMs locais e de produção não conseguem gerar JSON inválido quando guiados pela gramática gerada.
  - **Performance**: A compilação é instantânea (< 1ms para schemas típicos).
  - **Portabilidade**: Atende tanto o ecossistema GGUF (*llama.cpp*) quanto servidores de alta vazão (*vLLM*).
- **Negativas / Mitigações**:
  - Gramáticas de objetos com muitas propriedades opcionais em ordem arbitrária exigem ordenação canônica para evitar explosão combinatória ($O(N!)$). A adoção de ordem canônica de chaves é padrão aceito na indústria (Outlines/llama.cpp).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: As gramáticas são derivadas diretamente do manifesto contido no próprio arquivo `.skill`.
- [x] **Axioma 2 (Zero dependências externas)**: Implementação 100% Rust nativa sem dependências C ou I/O externo.
- [x] **Axioma 3 (Confinamento ocap)**: A compilação de gramáticas é uma transformação pura de dados (memória para memória), sem acesso ao ambiente.
- [x] **Axioma 4 (Isolamento hexagonal)**: Implementa estritamente `GrammarCompilerPort` sem acoplamento com motores de execução.
- [x] **Axioma 5 (Término determinístico)**: Algoritmo de geração estrutural com recursão finita delimitada pela profundidade do AST do schema.
- [x] **Axioma 6 (Prefixo estático imutável)**: Mantém conformidade com o formato estático de prompt da Seção Semântica.
- [x] **Axioma 7 (Limite de < 400 linhas)**: Decomposição modular em `grammar/gbnf.rs` e `grammar/regex_cfg.rs`.
