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
echo "▶️  [1/5] Verificando limites de linhas por arquivo (< 450 linhas)..."
OVERSIZED=$(find "${RUNTIME_DIR}/crates" -name "*.rs" -exec wc -l {} + | awk '$1 > 450 && $2 != "total" { print $1, $2 }')
if [ -n "${OVERSIZED}" ]; then
    echo "❌ FALHA: Arquivos excedendo limite cognitivo de 450 linhas:"
    echo "${OVERSIZED}"
    exit 1
fi
echo "   ✅ Todos os arquivos respeitam os limites cognitivos."

# 2. Verificação de compilação de todas as micro-crates
echo "▶️  [2/5] Verificando compilação do Workspace Cargo..."
(cd "${RUNTIME_DIR}" && cargo check --quiet)
echo "   ✅ Workspace compila perfeitamente sem erros."

# 3. Verificação de linter rigoroso com Clippy
echo "▶️  [3/5] Verificando conformidade de linter com Cargo Clippy..."
(cd "${RUNTIME_DIR}" && cargo clippy --quiet --all-targets --all-features -- -D warnings)
echo "   ✅ Zero advertências de linter ou código inseguro."

# 4. Execução dos testes automatizados de guardrail
echo "▶️  [4/5] Executando testes unitários e de arquitetura..."
(cd "${RUNTIME_DIR}" && cargo test --quiet)
echo "   ✅ Todos os testes unitários e guardrails foram aprovados."

# 5. Verificação de integridade dos arquivos .skill em examples/
echo "▶️  [5/5] Auditando integridade e hashes dos arquivos .skill..."
for skill in "${ROOT_DIR}/examples"/*.skill; do
    if [ -f "${skill}" ]; then
        (cd "${RUNTIME_DIR}" && cargo run --quiet --bin asl -- check "${skill}")
    fi
done
echo "   ✅ Todos os arquivos .skill canônicos possuem digests válidos."

echo "🎉 PARABÉNS: Todos os Guardrails do ASL 3.0 foram rigorosamente atendidos!"
