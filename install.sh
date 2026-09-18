#!/usr/bin/env bash
# ==============================================================================
# OFFICIAL AGENT SKILL LANGUAGE (ASL 3.0) INSTALLER
# ==============================================================================
set -euo pipefail

# 1. Automatically detect Cargo / Rust in standard locations (e.g. after fresh rustup install)
if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
fi

if [ -d "$HOME/.cargo/bin" ]; then
    export PATH="$HOME/.cargo/bin:$PATH"
fi
export PATH="$HOME/.cargo/bin:/usr/local/cargo/bin:/opt/homebrew/bin:/usr/local/bin:$PATH"

# Determine script directory safely even when piped from curl/stdin
SCRIPT_DIR=""
if [ -n "${BASH_SOURCE[0]:-}" ]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || true)"
fi

echo "========================================================"
echo "⚡ Installing Agent Skill Language (ASL 3.0)..."
echo "========================================================"

# 2. Verify Rust & Cargo presence
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust and Cargo were not found on this system."
    echo ""
    echo "👉 Rust is required to compile ASL. To install Rust and Cargo now, run:"
    echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "   Or visit https://rustup.rs for platform-specific packages."
    echo ""
    echo "Tip: If you already ran rustup, reload your environment first:"
    echo "    source \"\$HOME/.cargo/env\""
    echo ""
    echo "Then re-run this installer:"
    echo "    curl -fsSL https://raw.githubusercontent.com/asl-lang/asl/main/install.sh | bash"
    exit 1
fi

# 3. Build & Install ASL CLI globally
if [ -n "${SCRIPT_DIR}" ] && [ -d "${SCRIPT_DIR}/runtime/crates/asl-cli" ]; then
    echo "📦 Building and installing CLI 'asl' from local repository..."
    cargo install --path "${SCRIPT_DIR}/runtime/crates/asl-cli" --force
else
    echo "📦 Fetching latest ASL source repository..."
    TMP_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t 'asl-install')"
    trap 'rm -rf "${TMP_DIR}"' EXIT INT TERM
    
    if command -v git &> /dev/null; then
        git clone --depth 1 https://github.com/asl-lang/asl.git "${TMP_DIR}"
        echo "📦 Compiling and installing CLI 'asl' globally..."
        cargo install --path "${TMP_DIR}/runtime/crates/asl-cli" --force
    else
        echo "❌ Error: 'git' is required to clone and build ASL from source."
        echo "Please install git on your system and re-run this installer."
        exit 1
    fi
fi

echo ""
echo "========================================================"
echo "🎉 ASL 3.0 installed successfully!"
echo "========================================================"

# Verify PATH
CARGO_BIN="$HOME/.cargo/bin"
if [[ ":$PATH:" != *":$CARGO_BIN:"* ]]; then
    echo "⚠️  Note: Make sure $CARGO_BIN is in your PATH:"
    echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\""
    echo ""
fi

echo "To verify the installation:"
echo "    asl --version"
echo ""
echo "To run an ASL skill:"
echo "    asl run examples/english-only-guard.skill -i '{\"content\": \"audit text\", \"context_type\": \"documentation\"}'"
echo ""
echo "To start the MCP server (stdio or HTTP/SSE):"
echo "    asl serve --transport http --port 8080"
echo ""
echo "Documentation and specifications: https://asl-lang.github.io/"
