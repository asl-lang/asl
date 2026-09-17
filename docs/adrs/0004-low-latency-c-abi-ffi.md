# ADR-0004: Interface C-ABI de Baixa Latência In-Process (`libasl` / `asl-ffi`)

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Conselho de Arquitetura ASL / Cadente
- **Crates Afetadas**: `asl-ffi` (nova micro-crate), `runtime/Cargo.toml`

---

## 1. Contexto e Problema

O ASL 3.0 disponibiliza execução via CLI (`asl run`) e via transporte de rede/stdio no padrão Model Context Protocol (`asl serve`). No entanto, em sistemas de agentes de IA de alto desempenho — como runtimes em Python (LangChain, AutoGen, CrewAI), daemons em Node.js ou microsserviços em Go/C++ —, a comunicação via IPC sobre JSON-RPC introduz:
1. **Sobrecarga de Latência**: Context-switch de processo e IPC via pipe stdio adicionam $\approx 1.2\text{ a }5.0\text{ ms}$ por invocação de ferramenta.
2. **Duplicação de Processos**: A necessidade de orquestrar subprocessos zumbis e gerenciar pipes stdio sujeitos a travamentos de buffer.

Para agentes com loops cognitivos ultra-rápidos (ex: centenas de chamadas de validação e formatação por segundo), é imperativo que o runtime do ASL possa ser carregado **diretamente no mesmo espaço de memória do processo hospedeiro (in-process)** com latência inferior a $35\ \mu\text{s}$.

---

## 2. Proposta Detalhada da Decisão

Criar a micro-crate `asl-ffi` compilada como biblioteca compartilhada (`cdylib`), biblioteca estática (`staticlib`) e crate Rust (`rlib`), exportando símbolos de ligação C puros (`extern "C"` / `no_mangle`) acompanhados pelo cabeçalho canônico `include/libasl.h`.

### 2.1 Contrato da C-ABI (`libasl.h`)

```c
#ifndef LIBASL_H
#define LIBASL_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct asl_runtime_t asl_runtime_t;
typedef struct asl_skill_t asl_skill_t;

typedef struct {
    uint8_t success;           // 1 se sucesso, 0 se erro
    const char* json_output;   // JSON de saída ou envelope padronizado de erro
    uint64_t fuel_consumed;    // Opcodes consumidos
    uint64_t execution_time_ns;// Duração exata em nanossegundos
} asl_exec_result_t;

// Ciclo de Vida do Runtime
asl_runtime_t* asl_runtime_init(void);
void asl_runtime_free(asl_runtime_t* rt);

// Ciclo de Vida do Skill
asl_skill_t* asl_skill_load(asl_runtime_t* rt, const char* skill_source);
void asl_skill_free(asl_skill_t* skill);

// Execução Hermética In-Process
asl_exec_result_t* asl_skill_execute(
    asl_runtime_t* rt,
    asl_skill_t* skill,
    const char* entrypoint,
    const char* json_args
);
void asl_exec_result_free(asl_exec_result_t* result);

#ifdef __cplusplus
}
#endif

#endif // LIBASL_H
```

### 2.2 Barreira Rigorosa de Pânico (`catch_unwind`)

Nenhum erro de execução no Starlark, erro de alocação ou pânico interno pode propagar desenrolamento de pilha (*stack unwinding*) através da fronteira da FFI para código C/Python hospedeiro (o que causaria um `SIGABRT` imediato do processo).
Toda função de entrada da FFI deve ser encapsulada em:

```rust
std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
    // Lógica da chamada FFI
})).unwrap_or_else(|_| {
    // Retorno seguro de erro sem abortar processo hospedeiro
})
```

### 2.3 Gerenciamento de Memória Sem Leaks

Todas as estruturas de resultado alocam ponteiros gerenciados (`Box<T>`) cujos destrutores são chamados de volta nas funções dedicadas (`asl_exec_result_free`, `asl_skill_free`, `asl_runtime_free`).

---

## 3. Alternativas Consideradas

- **Alternativa A: Usar apenas IPC stdio (MCP)**:
  - *Descarte*: Não atende os requisitos de ultra-baixa latência (< 35 µs) e exige orquestração de processos filhos em Python/Node.js.
- **Alternativa B: Gerar bindings PyO3 específicos apenas para Python**:
  - *Descarte*: Acopla o projeto a um ecossistema único (Python). Uma C-ABI canônica atende simultaneamente Python (`ctypes`/`cffi`), Node.js (N-API), Go (`cgo`), C, C++, Zig e Rust.
- **Alternativa C: C-ABI pura em micro-crate `asl-ffi` (Escolhida)**:
  - *Justificativa*: Universal, atende à Seção 4 do Estudo Científico, oferece latência nanométrica e preserva a pureza hexagonal.

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - Latência in-process $\approx 25\text{ a }35\ \mu\text{s}$ (mais de $100\times$ mais rápido que IPC stdio).
  - Facilidade de consumo através de qualquer linguagem moderna.
  - Métricas de observabilidade de primeira classe (`fuel_consumed`, `execution_time_ns`).
- **Negativas / Riscos**:
  - Ponteiros brutos de C exigem gerenciamento cuidadoso de memória pelo consumidor hospedeiro (mitigado pelo fornecimento de funções `free` específicas e documentação de posse).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: A skill é carregada a partir de sua fonte atômica única `.skill`.
- [x] **Axioma 2 (Zero dependências externas)**: Implementado nativamente usando apenas `std::ffi::CStr` e os contratos do workspace ASL.
- [x] **Axioma 3 (Confinamento ocap)**: A execução in-process respeita o `ConfinedSecurityContext` do ASL com autoridade ambiente nula.
- [x] **Axioma 4 (Isolamento hexagonal)**: A crate `asl-ffi` atua como camada de aplicação/entrypoint, orquestrando adaptadores via traits.
- [x] **Axioma 5 (Término determinístico)**: O fuel metering decrementa normalmente in-process.
- [x] **Axioma 6 (Prefixo estático)**: Não interfere com a integridade do prefixo semântico.
- [x] **Axioma 7 (Limite de < 400 linhas)**: O arquivo `asl-ffi/src/lib.rs` terá aproximadamente 220 linhas.
