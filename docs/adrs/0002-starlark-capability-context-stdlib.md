# ADR-0002: Biblioteca Padrão de Capabilities no Starlark (ctx.fs, ctx.crypto, ctx.fuel)

- **Status**: Concluído
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Arquitetura Central ASL 3.0
- **Plano Vinculado**: [`docs/plans/0002-starlark-capability-context-stdlib.md`](../plans/0002-starlark-capability-context-stdlib.md)
- **Crates Afetadas**: `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-cli`

---

## 1. Contexto e Problema

No protótipo inicial do ASL 3.0, as funções determinísticas em Starlark recebiam um objeto de contexto vazio (`asl_ctx = struct()`). 

Para que skills executem operações reais (como auditar o diff do git, calcular hashes de integridade de artefatos ou verificar arquivos de configuração), elas precisam interagir com o ambiente externo. Porém, conceder acesso direto a APIs de sistema operacional (como `std::fs` ou sockets) violaria fatalmente o **Axioma 3 (Object-Capability estrito)** e reintroduziria ataques como o *Vice-Confuso* e *Injeção Indireta de Prompt*.

Precisamos de uma biblioteca padrão embutida no Starlark que exponha primitivas atenuadas através de `ctx`, sem depender de interpretadores externos (Axioma 2) e garantindo estrito confinamento (Lampson/Miller).

---

## 2. Proposta Detalhada da Decisão

Decidimos introduzir a **Biblioteca Padrão de Capabilities no Starlark** injetada dinamicamente via funções auxiliares acopladas à porta `CapabilityContext`:

```
               ┌─────────────────────────────────────────────────────┐
               │              Código .skill (Starlark)               │
               │                                                     │
               │  def run(ctx, input):                               │
               │      file_data = ctx.fs.read("src/config.json")     │
               │      digest = ctx.crypto.sha256(file_data)          │
               │      fuel = ctx.fuel.consumed()                     │
               │      return {"hash": digest}                        │
               └───────────┬─────────────┬─────────────┬─────────────┘
                           │             │             │
                    [Invocado]    [Invocado]    [Invocado]
                           │             │             │
                           ▼             ▼             ▼
                     ┌───────────┐ ┌───────────┐ ┌───────────┐
                     │  ctx.fs   │ │ctx.crypto │ │ ctx.fuel  │
                     └─────┬─────┘ └─────┬─────┘ └─────┬─────┘
                           │             │             │
                           └─────────────┼─────────────┘
                                         │ [Delega para]
                                         ▼
                           ┌───────────────────────────┐
                           │   asl-core-traits (Port)  │
                           │     CapabilityContext     │
                           └─────────────┬─────────────┘
                                         │ [Implementa]
                                         ▼
                           ┌───────────────────────────┐
                           │   asl-security (Adapter)  │
                           │  ConfinedSecurityContext  │
                           └───────────────────────────┘
```

### 2.1 Interface no Starlark (`ctx`)
1. **`ctx.fs.read(path: str) -> str | None`**:
   - Delega a leitura para `CapabilityContext::read_file(path)`.
   - Se o caminho estiver fora das raízes autorizadas (`confined_read_roots`), lança um erro determinístico Starlark.
   - O conteúdo retornado é automaticamente envolto em delimitadores de integridade se não confiável.
2. **`ctx.crypto.sha256(data: str) -> str`**:
   - Calcula o hash SHA-256 da string em hexadecimal, de forma pura em Rust sem dependências externas.
3. **`ctx.fuel.consumed() -> int`**:
   - Retorna o volume de opcodes de bytecode consumidos até o momento.
4. **`ctx.fuel.remaining() -> int`**:
   - Retorna o saldo remanescente antes do limite abortar a execução.

### 2.2 Extensão do Trait `CapabilityContext` em `asl-core-traits`
```rust
pub trait CapabilityContext: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Option<String>>;
    fn sha256(&self, data: &[u8]) -> String;
    fn check_fuel(&self) -> Result<u64>;
    fn fuel_consumed(&self) -> u64;
}
```

---

## 3. Alternativas Consideradas

- **Alternativa A: Expor módulos nativos Starlark em C/Python**:
  - *Descarte*: Viola o Axioma 2 (hermeticidade absoluta em Rust puro) e cria vetores de ataque em memória desprotegida.
- **Alternativa B: Fornecer arquivos pré-carregados no input JSON**:
  - *Descarte*: Ineficiente para repositórios médios/grandes. Se uma skill só precisa ler condicionalmente um arquivo dependendo de um parâmetro, pré-ler todos os arquivos desperdiça I/O e tokens de contexto.

---

## 4. Consequências e Trade-offs

### Positivas
- **Controle Cirúrgico**: A skill só lê caminhos explicitamente liberados em `capabilities.fs.confined_read_roots`.
- **Zero Sobrecarga de Processos**: Funções executam in-process no motor Rust com latência de microssegundos.
- **Determinismo**: Primitivas criptográficas e de leitura têm comportamento invariante e seguro.

### Negativas / Custos
- Necessidade de injetar stubs/funções nativas no wrapper de inicialização do Starlark em `asl-vm-starlark`.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: Preservada; todas as chamadas `ctx.*` ocorrem internamente no `.skill`.
- [x] **Axioma 2 (Zero Dependências)**: Criptografia e I/O usam Rust puro (`sha2`, `std::fs`).
- [x] **Axioma 3 (Confinamento ocap)**: Acesso exclusivo via handles atenuados de diretório (`canonicalize`).
- [x] **Axioma 4 (Isolamento Hexagonal)**: A engine Starlark não sabe se o contexto é real ou mock (`CapabilityContext`).
- [x] **Axioma 5 (Término por Fuel)**: Métodos `ctx.fuel` expõem o consumo para verificações formais.
- [x] **Axioma 6 (Prefixo Estático)**: As chamadas de biblioteca não afetam a seção de prefixo do `.skill`.
- [x] **Axioma 7 (Limite Cognitivo)**: Módulos mantidos com menos de 250 linhas cada.
