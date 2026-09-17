# ADR-0005: Adaptador de Motor de Execução WebAssembly / WASI (`asl-vm-wasm`)

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Conselho de Arquitetura ASL / Cadente
- **Crates Afetadas**: `asl-vm-wasm` (nova micro-crate), `runtime/Cargo.toml`

---

## 1. Contexto e Problema

O ASL 3.0 adota o Starlark como motor determinístico hermético padrão (`asl-vm-starlark`). O Starlark é ideal para validações, transformações funcionais de JSON e orquestração de APIs. Contudo, determinadas habilidades de agentes demandam:
1. **Linguagem Livre**: Habilidade de escrever skills em Rust, C, C++, Zig, Go ou AssemblyScript e compilá-las para bytecode portável.
2. **Computação Intensiva**: Operações criptográficas, manipulação de tensores, compressão ou parseamento de formatos binários de alto desempenho.
3. **Isolamento de Memória Linear Hermético**: A garantia formal de que o código roda dentro de uma memória linear WebAssembly (`WebAssembly Memory Sandbox`) com boundaries invioláveis.

Graças à Arquitetura Hexagonal adotada no ADR-0001, o sistema possui a porta `EnginePort` desacoplada em `asl-core-traits`, tornando a adição de um motor WebAssembly natural e sem quebras no restante do runtime.

---

## 2. Proposta Detalhada da Decisão

Criar a micro-crate `asl-vm-wasm` implementando o trait `EnginePort`.

### 2.1 Escolha da Engine: `wasmi` (Pure Rust Deterministic WASM Interpreter)

Avaliamos duas tecnologias para o motor WebAssembly:
- **Wasmtime (Bytecode Alliance)**: JIT compilation de alta performance, mas requer dependências nativas complexas (C/LLVM/Cranelift), compilação demorada e binários pesados.
- **wasmi (Pure Rust Interpreter)**:
  - Zero dependências de C ou compiladores externos (100% Rust puro).
  - Suporte nativo a **Fuel Metering** por instrução de bytecode ($O(1)$ timeout determinístico).
  - Suporte nativo a WAT (WebAssembly Text Format) e binários `.wasm` (em hex ou base64).
  - Isolamento hermético de memória e injeção de host functions seguras mapeadas para `CapabilityContext`.

A escolha oficial é **`wasmi`**, garantindo total aderência ao Axioma 2 (Zero dependências externas e hermeticidade nativa).

### 2.2 Estrutura do Adaptador `WasmEngine`

```rust
pub struct WasmEngine {
    // Motor hermético wasmi configurado com consumo de fuel
}

impl EnginePort for WasmEngine {
    fn name(&self) -> &'static str {
        "wasm-component"
    }

    fn execute(
        &self,
        code: &str,
        entrypoint: &str,
        input_args: &Value,
        context: &dyn CapabilityContext,
        limits: &Limits,
    ) -> Result<ExecutionResult>;
}
```

### 2.3 Formatos Suportados de Código
O bloco de código determinístico com tag `asl:wasm` ou `wasm` pode conter:
1. **WAT (WebAssembly Text Format)**: `(module (func (export "run") ...))` — legível por humanos e modelos de IA.
2. **WASM Binário Codificado**: Base64 ou Hexadecimal contendo o módulo compilado.

### 2.4 Mapeamento de Argumentos e Retorno
Para manter a interface JSON universal:
- Funções simples aceitam inteiros e retornam inteiros.
- Funções estruturadas lêem o JSON de entrada da memória linear através de ponteiro e tamanho (`ptr`, `len`) e retornam a saída na memória linear ou via ponteiro exportado, com fallbacks seguros.

---

## 3. Alternativas Consideradas

- **Alternativa A: Usar Wasmtime com JIT Cranelift**:
  - *Descarte*: Traz dependências C/LLVM pesadas que violam o Axioma 2 e quebram a portabilidade hermética em arquiteturas diversas sem ferramentas C instaladas.
- **Alternativa B: QuickJS compilado**:
  - *Descarte*: Não é WebAssembly universal, restringe-se a JavaScript e possui histórico de memory safety vulnerável em C.
- **Alternativa C: `wasmi` puro em Rust com fuel metering (Escolhida)**:
  - *Justificativa*: 100% de conformidade com os 7 Axiomas do ASL.

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - ASL passa a executar bytecode WebAssembly de qualquer linguagem compilável para WASM.
  - Fuel metering de opcodes garante que loops infinitos em C/Rust compilados terminem deterministamente.
  - Substituibilidade hexagonal comprovada na prática.
- **Negativas / Mitigações**:
  - Interpretação via `wasmi` é mais lenta que compilação JIT de máquina (porém mais rápida que Starlark para algoritmos numéricos e sem latência de compilação JIT).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: O código WASM (em WAT ou base64) é contido no arquivo atômico `.skill`.
- [x] **Axioma 2 (Zero dependências externas)**: `wasmi` é um motor 100% Rust puro.
- [x] **Axioma 3 (Confinamento ocap)**: Acesso ao mundo externo é bloqueado por padrão; apenas capabilities atenuadas de `CapabilityContext` são injetadas.
- [x] **Axioma 4 (Isolamento hexagonal)**: Implementa estritamente `EnginePort`.
- [x] **Axioma 5 (Término determinístico)**: `wasmi::Config::consume_fuel(true)` limita opcodes a `limits.max_fuel_opcodes`.
- [x] **Axioma 6 (Prefixo estático)**: Mantido intacto.
- [x] **Axioma 7 (Limite de < 400 linhas)**: O arquivo `asl-vm-wasm/src/lib.rs` terá < 300 linhas.
