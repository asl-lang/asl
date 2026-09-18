# ADR-0014: Distribuição de Binários Pré-Compilados e Instalador Instantâneo de Zero Dependências

- **Status**: Aceito
- **Data**: 2026-09-18
- **Autores**: Jean Catarina & Antigravity (IA)
- **Decisores**: Conselho de Arquitetura ASL
- **Componentes Afetados**: `install.sh`, `scripts/package_release.sh`, `asl-cli`, GitHub Releases Distribution

---

## 1. Contexto e Problema

Até a versão 0.3.0, a instalação do Agent Skill Language (ASL) através do comando canônico:
```bash
curl -fsSL https://raw.githubusercontent.com/asl-lang/asl/main/install.sh | bash
```
dependia exclusivamente da compilação a partir do código-fonte utilizando `cargo install --path ...`. 

Embora esse método seja robusto para desenvolvedores do próprio runtime, ele apresenta graves restrições para adoção por usuários finais, desenvolvedores de agentes e ambientes de CI/CD:
1. **Alta Latência de Instalação (1 a 3 minutos)**: A compilação do workspace do ASL envolve mais de 230 crates (incluindo o motor Starlark, Tokio, Axum, parsers, e primitivas criptográficas Blake3/Ed25519 em modo `--release`).
2. **Requisito Obrigatório do Toolchain Rust**: Exigia que qualquer máquina consumidora tivesse o `rustc`, `cargo` e toolchain C instalados previamente, gerando fricção de entrada e erros em máquinas limpas.
3. **Consumo Excessivo de Recursos de CPU e Memória**: Compilar o runtime consome múltiplos núcleos e gigabytes de RAM na máquina alvo.
4. **Discrepância com o Ecossistema Moderno de Ferramentas**: Ferramentas contemporâneas como Rustup (`sh.rustup.rs`), Bun, Deno, uv e RTK realizam download de binários pré-compilados e instalam em menos de 3 segundos com zero dependências externas.

---

## 2. Proposta Detalhada da Decisão

Estabelecemos o modelo formal de **Pre-built Binary Distribution** acoplado a um **Instalador Inteligente de Resolução Graciosa**.

### 2.1 Matriz Canônica de Plataformas e Nomenclatura de Artefatos

A cada release (tag `vX.Y.Z`), o processo automatizado de release produz arquivos compactados em `.tar.gz` contendo o binário compilado com otimização máxima (`--release`) e símbolos de depuração removidos via `strip`:

| Identificador de Destino | Sistema Operacional | Arquitetura de CPU | Artefato Gerado |
| :--- | :--- | :--- | :--- |
| `aarch64-apple-darwin` | macOS (11.0+) | Apple Silicon (M1/M2/M3/M4) | `asl-v{VERSION}-aarch64-apple-darwin.tar.gz` |
| `x86_64-apple-darwin` | macOS (10.12+) | Intel 64-bit | `asl-v{VERSION}-x86_64-apple-darwin.tar.gz` |
| `x86_64-unknown-linux-gnu` | Linux | x86_64 (glibc >= 2.17) | `asl-v{VERSION}-x86_64-unknown-linux-gnu.tar.gz` |
| `aarch64-unknown-linux-gnu` | Linux | ARM64 | `asl-v{VERSION}-aarch64-unknown-linux-gnu.tar.gz` |

Juntamente com os arquivos binários, é gerado o arquivo canônico de integridade `asl-v{VERSION}-checksums.sha256`, contendo as assinaturas criptográficas SHA-256 de todos os pacotes.

### 2.2 Estrutura do Pacote `.tar.gz`

Cada arquivo compactado contém exclusivamente:
```
asl-v{VERSION}-{TARGET}/
├── asl                # Executável compilado e stripped (< 10 MB)
├── LICENSE-MIT        # Termos de licença MIT
├── LICENSE-APACHE     # Termos de licença Apache-2.0
└── README.md          # Resumo de inicialização rápida
```

### 2.3 Máquina de Estados do `install.sh`

O instalador canônico adota uma arquitetura em 4 fases:

```
[Início]
   │
   ▼
[Fase 1: Detecção de SO e CPU] ──(OS != Darwin/Linux ou CPU desconhecida)──► [Fallback para Cargo Build]
   │
   ▼
[Fase 2: Resolução do Diretório de Destino]
   ├── Prioridade 1: $ASL_INSTALL_DIR (se definido)
   ├── Prioridade 2: $CARGO_HOME/bin ou ~/.cargo/bin (se existir)
   ├── Prioridade 3: ~/.local/bin (se estiver no PATH)
   └── Prioridade 4: ~/.asl/bin (cria e instrui PATH)
   │
   ▼
[Fase 3: Download & Instalação Instantânea (< 3s)]
   ├── Baixa tarball de: https://github.com/asl-lang/asl/releases/latest/download/...
   ├── Extrai o binário `asl` diretamente no diretório alvo
   ├── Atribui permissão de execução: chmod +x
   └── Valida execução atômica: `asl --version`
   │
   ├───(Sucesso)───► [Instalação Concluída em ~2 segundos]
   │
   └───(Falha de Download / 404)──► [Fase 4: Fallback Gracioso para Cargo Install]
                                       ├── Se Cargo presente: clona e compila localmente
                                       └── Se Cargo ausente: exibe instrução clara de instalação
```

### 2.4 Script de Release e Empacotamento (`scripts/package_release.sh`)

Script canônico de empacotamento responsável por:
1. Validar a versão corrente do workspace.
2. Compilar para os targets selecionados via `cargo build --release --target <triple> -p asl-cli`.
3. Executar `strip` no binário compilado, reduzindo o tamanho de ~35MB para ~8MB.
4. Empacotar o tarball `.tar.gz`.
5. Calcular hashes SHA-256 no formato padrão `shasum -a 256`.
6. Criar ou atualizar a release no GitHub via `gh release create v{VERSION} dist/*`.

---

## 3. Alternativas Consideradas

- **Alternativa A: Manter apenas compilação local com `cargo install`**:
  - *Descartada*: Inviabiliza a adoção rápida por agentes de IA e desenvolvedores que não utilizam Rust como linguagem primária, além de demorar minutos a cada teste ou ambiente novo.
- **Alternativa B: Repositório APT/Homebrew Tap exclusivo**:
  - *Descartada como canal exclusivo*: Requer que o usuário tenha Homebrew (Mac/Linux) ou APT (Debian/Ubuntu), fragmentando a experiência em containers Docker ou máquinas sem gerenciador de pacotes. Será adicionado futuramente como canal complementar.
- **Alternativa C: Pre-built Binaries via GitHub Releases com Fallback para Cargo**:
  - *Escolhida*: Padrão ouro da indústria de CLIs modernas. Oferece latência de instalação $< 3$ segundos, zero dependências prévias no sistema hospedeiro e preserva total compatibilidade através de fallback automático.

---

## 4. Consequências e Trade-offs

### Consequências Positivas
- **Velocidade Extrema**: A instalação passa de ~150 segundos para menos de 3 segundos.
- **Zero Fricção de Dependências**: Usuários não precisam instalar Rust, Cargo ou compiladores C para utilizar o ASL.
- **Leveza em CI/CD**: Ambientes de integração contínua podem rodar testes com o binário ASL instantaneamente.
- **Fallback Resiliente**: Se o usuário estiver em uma arquitetura exótica, o instalador compila via Cargo sem falhar.

### Riscos e Mitigações
- **Risco**: Binários pré-compilados incompatíveis com versões muito antigas de GLIBC no Linux.
  - *Mitigação*: Compilar os binários Linux contra targets padrão ou musl (`x86_64-unknown-linux-musl`), garantindo compatibilidade universal estática.
- **Risco**: Falhas transitórias de rede no download do GitHub Releases.
  - *Mitigação*: Tratamento de erro com fallback automático para o pipeline de compilação via código-fonte caso o download retorne erro.

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade do .skill)**: Não altera o formato nem o isolamento de arquivos `.skill`.
- [x] **Axioma 2 (Zero Dependências Externas)**: O binário distribuído é autocontido e não requer npm, pip ou crates externas para ser executado.
- [x] **Axioma 3 (Confinamento OCAP)**: As permissões de execução do binário permanecem no espaço de usuário sem elevação de privilégios.
- [x] **Axioma 4 (Isolamento Hexagonal)**: O instalador e o packaging respeitam as fronteiras dos ports e micro-crates do workspace.
- [x] **Axioma 5 (Término Determinístico)**: O download e extração possuem timeout e checagem de integridade SHA-256.
- [x] **Axioma 6 (Prefixo Estático Imutável)**: O CLI instalado preserva a arquitetura de cálculo de prefixos KV-cache.
- [x] **Axioma 7 (Limite Cognitivo de Linhas)**: Todos os scripts e arquivos de implementação permanecem estritamente dentro dos limites cognitivos (< 450 linhas).
