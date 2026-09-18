# Plano de Implementação: Distribuição de Binários Pré-Compilados e Instalador Instantâneo

- **ADR Vinculado**: `docs/adrs/0014-prebuilt-binary-distribution-and-fast-installer.md`
- **Data**: 2026-09-18
- **Responsável**: Jean Catarina & Antigravity (IA)
- **Status**: **Concluído** (4/4 Fases - 100%)
- **Meta**: Reduzir a latência de instalação do ASL de ~150s (compilação de 230+ crates via `cargo install`) para < 3s via download de binário pré-compilado, eliminando o pré-requisito de ter Rust instalado para usuários do CLI, com fallback gracioso para compilação local.

---

## Fase 1: Criação do Script Canônico de Empacotamento de Releases (`scripts/package_release.sh`)

### 1.1 Objetivo da Fase
Implementar a ferramenta oficial de empacotamento que compila o `asl-cli` em modo `--release`, executa `strip` para redução agressiva de tamanho (< 10MB), compacta o pacote em `.tar.gz` e gera os hashes SHA-256 de integridade em `dist/`.

### 1.2 Código a Implementar
No arquivo `scripts/package_release.sh`:
```bash
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
DIST_DIR="${ROOT_DIR}/dist"

VERSION="$(grep -m1 'version = ' "${ROOT_DIR}/runtime/Cargo.toml" | cut -d '"' -f2)"
TARGET="${1:-$(rustc -vV | grep 'host:' | cut -d ' ' -f2)}"

echo "📦 Packaging ASL v${VERSION} for target: ${TARGET}..."
mkdir -p "${DIST_DIR}"

(cd "${ROOT_DIR}/runtime" && cargo build --release --target "${TARGET}" -p asl-cli)

BIN_SRC="${ROOT_DIR}/runtime/target/${TARGET}/release/asl"
STAGE_DIR="${DIST_DIR}/asl-v${VERSION}-${TARGET}"
rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}"

cp "${BIN_SRC}" "${STAGE_DIR}/asl"
strip "${STAGE_DIR}/asl" 2>/dev/null || true

cp "${ROOT_DIR}/README.md" "${STAGE_DIR}/" 2>/dev/null || true
cp "${ROOT_DIR}/LICENSE"* "${STAGE_DIR}/" 2>/dev/null || true

ARCHIVE_NAME="asl-v${VERSION}-${TARGET}.tar.gz"
(cd "${DIST_DIR}" && tar -czf "${ARCHIVE_NAME}" -C "${DIST_DIR}" "asl-v${VERSION}-${TARGET}")
rm -rf "${STAGE_DIR}"

(cd "${DIST_DIR}" && shasum -a 256 "${ARCHIVE_NAME}" >> checksums.sha256)
echo "✅ Packaged: ${DIST_DIR}/${ARCHIVE_NAME}"
```

### 1.3 Verificação Local da Fase 1
```bash
chmod +x scripts/package_release.sh
./scripts/package_release.sh
ls -lh dist/
```

### 1.4 Finalização da Fase 1
```bash
git add scripts/package_release.sh
git commit -m "feat(release): adicionar script canonico de empacotamento de binarios pre-compilados"
```

---

## Fase 2: Modernização do Instalador Oficial (`install.sh`) com Fast-Path

### 2.1 Objetivo da Fase
Atualizar o `install.sh` para:
1. Detectar arquitetura de SO e CPU (`Darwin`/`Linux`, `arm64`/`x86_64`).
2. Tentar download imediato do tarball pré-compilado do GitHub Releases (< 3s).
3. Extrair o binário diretamente para a pasta de destino (`$HOME/.asl/bin` ou `$HOME/.cargo/bin`).
4. Fallback resiliente para `cargo install` caso não haja binário pré-compilado para a arquitetura ou ocorra falha de rede.
5. Manter 100% de conformidade com a política de inglês estrito (`check_english_only.sh`).

### 2.2 Código a Implementar
No arquivo `install.sh`:
- Funções: `detect_platform`, `resolve_install_dir`, `install_prebuilt`, `fallback_source_build`.
- Resolução de PATH amigável ao usuário.

### 2.3 Verificação Local da Fase 2
```bash
./scripts/check_english_only.sh
bash -c "$(cat install.sh)"
asl --version
```

### 2.4 Finalização da Fase 2
```bash
git add install.sh
git commit -m "feat(installer): implementar download de binario pre-compilado em sub-3s com fallback"
```

---

## Fase 3: Publicação dos Artefatos de Release v0.3.0 no GitHub

### 3.1 Objetivo da Fase
Compilar e disponibilizar os primeiros binários pré-compilados na release oficial `v0.3.0` do repositório `asl-lang/asl` via GitHub CLI (`gh release create`).

### 3.2 Execução
```bash
./scripts/package_release.sh x86_64-apple-darwin
./scripts/package_release.sh aarch64-apple-darwin
gh release create v0.3.0 dist/*.tar.gz dist/checksums.sha256 \
  --title "ASL v0.3.0: High-Performance Agent Skill Runtime" \
  --notes "Official v0.3.0 release featuring sub-3-second installation and pre-built binaries."
```

---

## Fase 4: Auditoria Completa dos Guardrails e Push na Main

### 4.1 Objetivo da Fase
Rodar a bateria completa de guardrails e garantir conformidade científica, clippy zero warnings, testes 100% ok e push para `main`.

### 4.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 4.3 Finalização
```bash
git push origin main
```
