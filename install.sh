#!/usr/bin/env bash
# ==============================================================================
# INSTALADOR OFICIAL DO AGENT SKILL LANGUAGE (ASL 3.0)
# Desenvolvido por Jean Catarina (Cadente)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUNTIME_DIR="${SCRIPT_DIR}/runtime"

echo "========================================================"
echo "⚡ Instalando Agent Skill Language (ASL 3.0)..."
echo "========================================================"

# 1. Verificar se Cargo / Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Erro: Rust e Cargo não foram encontrados no sistema."
    echo "👉 Instale Rust via https://rustup.rs e execute este script novamente."
    exit 1
fi

echo "📦 Compilando e instalando CLI 'asl' globalmente..."
cargo install --path "${RUNTIME_DIR}/crates/asl-cli" --force

echo "📚 Compilando bibliotecas de FFI C-ABI (libasl)..."
cargo build --release --manifest-path "${RUNTIME_DIR}/crates/asl-ffi/Cargo.toml"

echo ""
echo "========================================================"
echo "🎉 ASL 3.0 instalado com sucesso!"
echo "========================================================"

# Verificar PATH
CARGO_BIN="$HOME/.cargo/bin"
if [[ ":$PATH:" != *":$CARGO_BIN:"* ]]; then
    echo "⚠️  Nota: Certifique-se de que $CARGO_BIN está no seu PATH:"
    echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    echo ""
fi

echo "Para verificar a instalação:"
echo "    asl --version"
echo ""
echo "Para executar uma skill:"
echo "    asl run examples/git-conventional-commit.skill -i '{\"intent\": \"novo recurso\", \"diff_stat\": \"1 file\"}'"
echo ""
echo "Para iniciar o servidor MCP (stdio ou HTTP/SSE):"
echo "    asl serve --transport http --port 8080"
echo ""
echo "Documentação e exemplos disponíveis no diretório 'examples/' e 'docs/'."
