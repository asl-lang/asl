# ADR-0022: Arquitetura Integral de Ergonomia de Linguagem, Confinamento OCap Unificado e Diagnósticos Transparentes

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `asl-spec`, `asl-core-traits`, `asl-security`, `asl-vm-starlark`, `asl-parser`, `asl-cli`, `docs/`
- **Axiomas Relacionados**: Axioma 1 (Atomicidade do .skill), Axioma 2 (Zero Dependências Externas), Axioma 3 (Confinamento OCap), Axioma 4 (Isolamento Hexagonal), Axioma 5 (Término com Fuel Metering), Axioma 7 (Limite Cognitivo < 450 linhas)

---

## 1. Contexto e Declaração do Problema

Após o desenvolvimento de skills reais por agentes de IA e desenvolvedores, foi realizada uma auditoria completa de experiência de desenvolvedor (DX) que revelou 11 pontos críticos de fricção arquitetural:

1. **Inconsistência entre Documentação e Runtime**: Cheat sheets e tutoriais prometiam métodos como `ctx.env.get()`, `ctx.http.post()`, `ctx.fs.write()` e `ctx.crypto.base64_encode()`, mas nem todas as APIs estavam expostas no runtime compilado.
2. **Vazamento de Detalhes Internos de Implementação**: Erros de execução expunham internals do Starlark (`error: Object of type struct has no attribute env --> asl_skill.star:8:13`), quebrando a abstração do ASL e confundindo o desenvolvedor.
3. **Pattern Matching Restrito**: O construto `match` só operava em caminhos de entrada específicos de regras, sem suporte a expressões ou variáveis arbitrárias em código ASL procedural.
4. **Strings Não-Iteráveis e Manipulação Rígida**: O dialeto padrão do Starlark rejeita iteração de strings (`for ch in s`), dificultando validações de input.
5. **Ausência de Mecanismo Seguro de Parsing / Erros**: A inexistência de blocos `try/except` combinada com funções de conversão que entram em pânico (`int("abc")`) impossibilitava a validação segura de entradas sem abortar o runtime.
6. **Descarte Silencioso de Capacidades no Frontmatter**: Variações comuns de sintaxe no YAML (ex.: `roots:` vs `confined_read_roots:`, `domains:` vs `net.allow_domains:`) eram descartadas pelo parser sem aviso ou erro.
7. **`asl check` Não Validava Executabilidade**: O comando verificava a sintaxe YAML e os digests de integridade, mas não compilava o código nem realizava dry-run para detectar funções inexistentes ou erros de sintaxe no bloco executável.
8. **Ausência de REPL para Prototipação Rápida**: Não havia como testar chamadas OCap (`ctx.fs.read`, `ctx.crypto.sha256`) ou expressões de forma incremental sem escrever um arquivo `.skill` completo.
9. **Projeção Sombra Sem Transparência**: `asl check` gerava arquivos `.md` sem anúncio claro ou flag de desativação (`--no-shadow`).
10. **Conflito entre Posicionamento e Recursos Reais**: Promessa de "substituir bash scripts" sem oferecer os primitivos essenciais (I/O HTTP confinado, leitura/escrita de arquivos confinada, chaves de ambiente).
11. **Templates Starter Minimalistas Demais**: `asl template skill` omitia `capabilities`, `limits` e `input_schema`.

---

## 2. Proposta Detalhada e Decisão Arquitetural

Decidimos resolver todos os 11 pontos de forma unificada na raiz da arquitetura ASL através de quatro pilares:

### Pilar 1: Contexto OCap Integral e Primitivos de I/O (`asl-core-traits`, `asl-security`, `asl-vm-starlark`)
- **`ctx.env`**: `get(key, default=None)` restrito rigorosamente às chaves declaradas em `capabilities.env`.
- **`ctx.http`**: Métodos REST completos: `get(url, headers=None)`, `post(url, headers=None, json=None, data=None)`, `put`, `patch`, `delete`, retornando objeto `struct(status, text, headers, json())`. Confinado aos domínios autorizados em `capabilities.net`. Consumo de fuel físico (1 opcode por 16 bytes transferidos).
- **`ctx.fs`**:
  - `read(path)`: leitura confinada aos diretórios declarados em `capabilities.fs.roots` / `confined_read_roots`.
  - `write(path, content)`: escrita confinada aos diretórios declarados em `capabilities.fs.allow_write` / `write_roots`.
  - `exists(path)`: checagem de existência nos caminhos autorizados.
  - `list(path)`: listagem de diretórios autorizados.
- **`ctx.crypto`**:
  - `sha256(data)`: hash criptográfico SHA-256 em hex.
  - `base64_encode(data)` e `base64_decode(encoded)`.
- **`ctx.fuel`**: `consumed()` e `remaining()`.

### Pilar 2: Camada de Diagnósticos Transparentes e Mapeamento de Linhas
- A VM intercepta qualquer erro de execução do motor subjacente.
- Remove qualquer menção a `.star` ou Starlark.
- Mapeia o número da linha de volta para o arquivo `.skill` original do usuário.
- Se o erro for atributo inexistente em `struct` (ex.: `ctx.env` ou `ctx.http`), traduz imediatamente para mensagem contextual com sugestão de correção no frontmatter.

### Pilar 3: Ergonomia da Linguagem ASL e Biblioteca Padrão Segura
- **Pattern Matching Unificado**: O pré-processador do ASL traduz blocos `match <expr>:` com `when <val>:` e `otherwise:` para cadeias `if/elif/else` portáveis e determinísticas.
- **Manipulação Segura de Strings**: Inclusão na biblioteca global de:
  - `chars(s)`: retorna lista de caracteres individuais de uma string.
  - `is_digit(s)`, `is_alpha(s)`, `is_alnum(s)`.
  - `strip(s)`, `split(s, sep=" ")`, `starts_with(s, prefix)`, `ends_with(s, suffix)`, `contains(s, sub)`.
- **Conversão Segura sem Pânico**:
  - `to_int(val, default=None)`: converte para inteiro ou retorna o valor padrão em caso de falha.
  - `is_int(val)`: valida se um valor é conversível para número inteiro.
  - `to_float(val, default=None)`, `try_json(val, default=None)`.
  - `try_call(fn, *args)`: executa função com tratamento seguro, retornando `struct(ok=bool, value=..., error=...)`.

### Pilar 4: Esquema Resiliente e Ferramental de DX (`asl-cli`)
- **Deserialização Resiliente de Capabilities**: Suporta sintaxe simplificada em lista (`fs: ["..."]`, `domains: ["..."]`, `env: ["..."]`) e sintaxe estruturada (`fs: { roots: [...], write: [...] }`, `net: { allow_domains: [...] }`). Campos desconhecidos geram aviso explícito.
- **`asl check` com Verificação de Compilação & `--dry-run`**: Compila o código no parser/motor para assegurar executabilidade real e suporta `--dry-run` com mock input.
- **`asl repl`**: Prototipação interativa com contexto OCap mock/ativo configurável.
- **Transparência de Projeção Sombra**: Anúncio explícito de arquivos criados/modificados e suporte ao flag `--no-shadow`.
- **Templates Ricos e Auto-Explicativos**: `asl template` emite modelos comentados com capabilities, limites e schema.

---

## 3. Alternativas Consideradas

- **Alternativa 1: Suporte limitado apenas ao Starlark padrão**:
  - Rejeitada pois strings não iteráveis e falta de try/except forçam o desenvolvedor a lidar com complexidades acidentais da engine em vez de focar na lógica de negócio da skill.
- **Alternativa 2: Expor APIs não herméticas do SO host**:
  - Rejeitada categoricamente por violar os Axiomas 2, 3 e 4 (Confinamento OCap e Isolamento Hexagonal).

---

## 4. Consequências e Benefícios

### Positivas:
- **Zero Fricção**: Alinhamento absoluto entre a documentação oficial e o comportamento em tempo de execução.
- **Erros Úteis**: Mensagens que orientam a correção imediatamente sem vazar camadas internas.
- **Substituição Real de Scripts Shell**: Skills podem ler arquivos de configuração, consultar APIs remotas via HTTP e autenticar com Base64 de forma autocontida sem violar o confinamento OCap.
- **Robustez de Validação**: Validação de strings e números com primitivos seguros sem crash da VM.

