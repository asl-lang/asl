#!/usr/bin/env bash
# ==============================================================================
# OFFICIAL AGENT SKILL LANGUAGE (ASL 3.0) FAST ZERO-DEPENDENCY INSTALLER
# ==============================================================================
set -euo pipefail

# 1. Environment & PATH configuration
if [ -f "$HOME/.cargo/env" ]; then
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
fi

export PATH="$HOME/.asl/bin:$HOME/.local/bin:$HOME/.cargo/bin:/usr/local/bin:/opt/homebrew/bin:$PATH"

# Determine script directory safely even when piped from curl/stdin
SCRIPT_DIR=""
if [ -n "${BASH_SOURCE[0]:-}" ]; then
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || true)"
fi

echo "========================================================"
echo "⚡ Installing Agent Skill Language (ASL 3.0)..."
echo "========================================================"

# 2. Detect platform architecture
detect_target() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Darwin)
            case "$arch" in
                x86_64) echo "x86_64-apple-darwin" ;;
                arm64|aarch64) echo "aarch64-apple-darwin" ;;
                *) echo "" ;;
            esac
            ;;
        Linux)
            case "$arch" in
                x86_64|amd64) echo "x86_64-unknown-linux-gnu" ;;
                aarch64|arm64) echo "aarch64-unknown-linux-gnu" ;;
                *) echo "" ;;
            esac
            ;;
        *)
            echo ""
            ;;
    esac
}

# 3. Resolve destination installation directory
resolve_install_dir() {
    if [ -n "${ASL_INSTALL_DIR:-}" ]; then
        echo "${ASL_INSTALL_DIR}"
        return 0
    fi
    if [ -d "$HOME/.cargo/bin" ]; then
        echo "$HOME/.cargo/bin"
        return 0
    fi
    if [ -d "$HOME/.local/bin" ] || [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
        echo "$HOME/.local/bin"
        return 0
    fi
    echo "$HOME/.asl/bin"
}

TARGET="$(detect_target)"
DEST_DIR="$(resolve_install_dir)"
mkdir -p "${DEST_DIR}"

INSTALLED=0
RELEASE_VERSION="${ASL_VERSION:-0.3.0}"

# 4. Fast-path: Download pre-built binary (< 3s, zero dependencies required)
if [ -n "${TARGET}" ] && command -v curl &> /dev/null && command -v tar &> /dev/null; then
    TARBALL_NAME="asl-v${RELEASE_VERSION}-${TARGET}.tar.gz"
    DOWNLOAD_URL="https://github.com/asl-lang/asl/releases/download/v${RELEASE_VERSION}/${TARBALL_NAME}"

    echo "🚀 Downloading pre-compiled binary for ${TARGET}..."
    TMP_DL_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t 'asl-dl')"
    trap 'rm -rf "${TMP_DL_DIR}"' EXIT INT TERM

    if curl -fsSL "${DOWNLOAD_URL}" -o "${TMP_DL_DIR}/${TARBALL_NAME}" 2>/dev/null; then
        echo "📦 Extracting binary to ${DEST_DIR}..."
        tar -xzf "${TMP_DL_DIR}/${TARBALL_NAME}" -C "${TMP_DL_DIR}"
        if [ -f "${TMP_DL_DIR}/asl-v${RELEASE_VERSION}-${TARGET}/asl" ]; then
            mv "${TMP_DL_DIR}/asl-v${RELEASE_VERSION}-${TARGET}/asl" "${DEST_DIR}/asl"
        elif [ -f "${TMP_DL_DIR}/asl" ]; then
            mv "${TMP_DL_DIR}/asl" "${DEST_DIR}/asl"
        fi

        chmod +x "${DEST_DIR}/asl"
        if "${DEST_DIR}/asl" --version &> /dev/null; then
            INSTALLED=1
            echo "⚡ Pre-compiled binary verified in seconds!"
        fi
    fi
fi

# 5. Fallback path: Compile from source using Cargo if pre-built is unavailable
if [ "${INSTALLED}" -eq 0 ]; then
    echo "⚠️  Pre-compiled binary not found for target '${TARGET:-unknown}' (or network offline)."
    echo "🔄 Attempting build from source via Cargo..."

    if ! command -v cargo &> /dev/null; then
        echo ""
        echo "❌ Error: Neither pre-compiled binary nor Cargo was found on this system."
        echo ""
        echo "👉 To install Rust and Cargo, run:"
        echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        echo ""
        echo "   Or check official releases for supported platforms:"
        echo "    https://github.com/asl-lang/asl/releases"
        exit 1
    fi

    if [ -n "${SCRIPT_DIR}" ] && [ -d "${SCRIPT_DIR}/runtime/crates/asl-cli" ]; then
        echo "📦 Building and installing CLI 'asl' from local repository..."
        cargo install --path "${SCRIPT_DIR}/runtime/crates/asl-cli" --force
    else
        echo "📦 Fetching latest ASL source repository..."
        TMP_SRC_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t 'asl-src')"
        trap 'rm -rf "${TMP_SRC_DIR}"' EXIT INT TERM

        if command -v git &> /dev/null; then
            git clone --depth 1 https://github.com/asl-lang/asl.git "${TMP_SRC_DIR}"
            echo "📦 Compiling and installing CLI 'asl' globally..."
            cargo install --path "${TMP_SRC_DIR}/runtime/crates/asl-cli" --force
        else
            echo "❌ Error: 'git' is required to clone and build ASL from source."
            exit 1
        fi
    fi
    INSTALLED=1
fi

echo ""
echo "========================================================"
echo "🎉 ASL 3.0 installed successfully!"
echo "========================================================"

# 6. Verify PATH accessibility
if ! command -v asl &> /dev/null; then
    echo "⚠️  Note: '${DEST_DIR}' is not yet in your PATH."
    echo "Add the following line to your shell profile (~/.zshrc or ~/.bashrc):"
    echo "    export PATH=\"${DEST_DIR}:\$PATH\""
    echo ""
    echo "Then reload your shell:"
    echo "    source ~/.zshrc  # or source ~/.bashrc"
    echo ""
fi

echo "To verify the installation:"
echo "    asl --version"
echo ""
echo "To run an ASL skill:"
echo "    asl run examples/git-conventional-commit.skill"
echo ""
echo "To start the MCP server:"
echo "    asl serve --transport http --port 8080"
echo ""
echo "Documentation and specifications: https://asl-lang.github.io/"
