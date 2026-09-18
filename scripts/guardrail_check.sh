#!/usr/bin/env bash
# ==============================================================================
# GUARDRAIL AUTOMATIZADO DE CONFORMIDADE ARQUITETURAL E CIENTÍFICA DO ASL
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
RUNTIME_DIR="${ROOT_DIR}/runtime"

echo "🛡️  Iniciando verificação de Guardrails do Projeto ASL..."

# 1. Verificação de limites cognitivos de tamanho de arquivo
echo "▶️  [1/7] Verificando limites de linhas por arquivo (< 450 linhas)..."
OVERSIZED=$(find "${RUNTIME_DIR}/crates" -name "*.rs" -exec wc -l {} + | awk '$1 > 450 && $2 != "total" { print $1, $2 }')
if [ -n "${OVERSIZED}" ]; then
    echo "❌ FALHA: Arquivos excedendo limite cognitivo de 450 linhas:"
    echo "${OVERSIZED}"
    exit 1
fi
echo "   ✅ Todos os arquivos respeitam os limites cognitivos."

# 2. Verificação de compilação de todas as micro-crates
echo "▶️  [2/7] Verificando compilação do Workspace Cargo..."
(cd "${RUNTIME_DIR}" && cargo check --quiet)
echo "   ✅ Workspace compila perfeitamente sem erros."

# 3. Verificação de linter rigoroso com Clippy
echo "▶️  [3/7] Verificando conformidade de linter com Cargo Clippy..."
(cd "${RUNTIME_DIR}" && cargo clippy --quiet --all-targets --all-features -- -D warnings)
echo "   ✅ Zero advertências de linter ou código inseguro."

# 4. Execução dos testes automatizados de guardrail
echo "▶️  [4/7] Executando testes unitários e de arquitetura..."
(cd "${RUNTIME_DIR}" && cargo test --quiet)
echo "   ✅ Todos os testes unitários e guardrails foram aprovados."

# 5. Verificação de integridade dos arquivos do ecossistema ASL em examples/
echo "▶️  [5/7] Auditando integridade e hashes dos arquivos do ecossistema ASL..."
for ext in skill tool asl; do
    for file in "${ROOT_DIR}/examples"/*.${ext}; do
        if [ -f "${file}" ]; then
            (cd "${RUNTIME_DIR}" && cargo run --quiet --bin asl -- check "${file}")
        fi
    done
done
echo "   ✅ Todos os arquivos canônicos do ecossistema ASL possuem digests válidos."

# 6. English-Only Compliance Audit
echo "▶️  [6/7] Verifying strict English-only language policy..."
"${ROOT_DIR}/scripts/check_english_only.sh"
echo "   ✅ Strict English-only language compliance verified."

# 7. Zero-Drift Spec-Driven Documentation (SDD) Audit
echo "▶️  [7/7] Verifying Zero-Drift between SSOT Specs, CLI and Website..."
python3 "${ROOT_DIR}/scripts/sync_docs_spec.py"
(cd "${RUNTIME_DIR}" && cargo test --quiet -p asl-cli --bin asl docs_ssot::tests::test_zero_drift_clap_vs_ssot)
echo "   ✅ Zero-Drift verified: CLI and Website documentation match SSOT 1:1."

echo "🎉 PARABÉNS: Todos os Guardrails do ASL 3.0 foram rigorosamente atendidos!"
