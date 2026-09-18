#!/usr/bin/env bash
# ==============================================================================
# OFFICIAL AGENT SKILL LANGUAGE (ASL 3.0) FAST ZERO-DEPENDENCY INSTALLER
# ==============================================================================
set -euo pipefail

DAEMON_ARG=""
for arg in "$@"; do
    case "$arg" in
        --enable-daemon|--yes|-y)
            DAEMON_ARG="yes"
            ;;
        --no-daemon|--no|-n)
            DAEMON_ARG="no"
            ;;
        --help|-h)
            echo "Usage: install.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --enable-daemon, -y   Automatically enable background zero-touch sync"
            echo "  --no-daemon, -n       Skip background daemon installation (clean on-demand mode)"
            echo "  --help, -h            Show this help message"
            exit 0
            ;;
    esac
done

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

# 6. Guided configuration of zero-touch background daemon
ASL_BIN=""
if [ -x "${DEST_DIR}/asl" ]; then
    ASL_BIN="${DEST_DIR}/asl"
elif command -v asl &> /dev/null; then
    ASL_BIN="$(command -v asl)"
elif [ -x "${HOME}/.cargo/bin/asl" ]; then
    ASL_BIN="${HOME}/.cargo/bin/asl"
fi

if [ -n "${ASL_BIN}" ]; then
    WANT_DAEMON=""

    if [ "$DAEMON_ARG" = "yes" ]; then
        WANT_DAEMON=1
    elif [ "$DAEMON_ARG" = "no" ]; then
        WANT_DAEMON=0
    elif [ -n "${ASL_ENABLE_DAEMON:-}" ]; then
        case "$ASL_ENABLE_DAEMON" in
            1|true|yes|YES) WANT_DAEMON=1 ;;
            0|false|no|NO)  WANT_DAEMON=0 ;;
        esac
    fi

    if [ -z "${WANT_DAEMON}" ]; then
        # Check if an interactive terminal is available (direct or through /dev/tty when piped)
        if [ -c /dev/tty ] || [ -t 0 ]; then
            echo ""
            echo "========================================================"
            echo "💡 Guided Configuration: Automatic Zero-Touch Sync"
            echo "========================================================"
            echo "ASL includes a lightweight background watcher that automatically"
            echo "projects and updates .md files whenever a .skill is created or edited"
            echo "in Claude Code, Cursor, Gemini, or your workspaces."
            echo ""
            printf "Enable automatic background sync? (Recommended) [Y/n]: "

            USER_INPUT=""
            if [ -c /dev/tty ]; then
                read -r USER_INPUT < /dev/tty || USER_INPUT=""
            else
                read -r USER_INPUT || USER_INPUT=""
            fi

            case "${USER_INPUT}" in
                [nN]|[nN][oO])
                    WANT_DAEMON=0
                    ;;
                *)
                    WANT_DAEMON=1
                    ;;
            esac
        else
            # Headless / CI non-interactive environment: default to safe, non-intrusive opt-out
            WANT_DAEMON=0
        fi
    fi

    if [ "${WANT_DAEMON}" -eq 1 ]; then
        echo ""
        echo "⚡ Configuring universal zero-touch daemon (LaunchAgent / systemd / background service)..."
        if "${ASL_BIN}" daemon install; then
            echo "✅ Zero-touch shadow projection daemon installed and active across your OS!"
        else
            echo "ℹ️  Run '${ASL_BIN} daemon install' to activate automatic zero-touch shadow projection."
        fi
    else
        echo ""
        echo "ℹ️  Automatic background daemon skipped (operating in clean, on-demand mode)."
        echo "💡 Useful on-demand commands:"
        echo "    asl sync <path>       # Project .md from .skill files on demand"
        echo "    asl watch <path>      # Run temporary watcher in foreground"
        echo "    asl daemon install    # Activate background service later anytime"
    fi
fi

# 7. Verify and configure PATH accessibility
if ! command -v asl &> /dev/null; then
    SHELL_PROFILE=""
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_PROFILE="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        SHELL_PROFILE="$HOME/.bashrc"
    elif [ -f "$HOME/.profile" ]; then
        SHELL_PROFILE="$HOME/.profile"
    fi

    if [ -n "${SHELL_PROFILE}" ]; then
        if ! grep -q "${DEST_DIR}" "${SHELL_PROFILE}" 2>/dev/null; then
            echo "" >> "${SHELL_PROFILE}"
            echo "# ASL (Agent Skill Language) PATH" >> "${SHELL_PROFILE}"
            echo "export PATH=\"${DEST_DIR}:\$PATH\"" >> "${SHELL_PROFILE}"
            echo "✅ Automatically configured PATH in ${SHELL_PROFILE}"
        fi
    fi

    echo ""
    echo "⚠️  To start using 'asl' immediately in this terminal session, run:"
    echo "    export PATH=\"${DEST_DIR}:\$PATH\""
    echo ""
fi

echo "To verify the installation:"
echo "    asl --version"
echo ""
echo "To manage the zero-touch background daemon:"
echo "    asl daemon status"
echo "    asl daemon install"
echo "    asl daemon stop"
echo ""
echo "To run an ASL skill:"
echo "    asl run examples/git-conventional-commit.skill"
echo ""
echo "To start the MCP server:"
echo "    asl serve --transport http --port 8080"
echo ""
echo "Documentation and specifications: https://asl-lang.github.io/"
