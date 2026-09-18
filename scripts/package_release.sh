#!/usr/bin/env bash
# ==============================================================================
# OFFICIAL RELEASE PACKAGING SCRIPT FOR AGENT SKILL LANGUAGE (ASL 3.0)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
DIST_DIR="${ROOT_DIR}/dist"

VERSION="$(grep -m1 'version = ' "${ROOT_DIR}/runtime/Cargo.toml" | cut -d '"' -f2)"
HOST_TARGET="$(rustc -vV | grep 'host:' | cut -d ' ' -f2)"
TARGET="${1:-${HOST_TARGET}}"

echo "========================================================"
echo "📦 Packaging ASL v${VERSION} for target: ${TARGET}"
echo "========================================================"

mkdir -p "${DIST_DIR}"

echo "🔨 Compiling 'asl-cli' in release mode..."
(cd "${ROOT_DIR}/runtime" && cargo build --release --target "${TARGET}" -p asl-cli)

BIN_SRC="${ROOT_DIR}/runtime/target/${TARGET}/release/asl"
if [ ! -f "${BIN_SRC}" ]; then
    echo "❌ Error: Compiled binary not found at ${BIN_SRC}"
    exit 1
fi

PACKAGE_NAME="asl-v${VERSION}-${TARGET}"
STAGE_DIR="${DIST_DIR}/${PACKAGE_NAME}"
rm -rf "${STAGE_DIR}"
mkdir -p "${STAGE_DIR}"

echo "✂️  Copying and stripping binary symbols..."
cp "${BIN_SRC}" "${STAGE_DIR}/asl"
chmod +x "${STAGE_DIR}/asl"

if command -v strip &> /dev/null; then
    strip "${STAGE_DIR}/asl" 2>/dev/null || true
fi

# Include basic licenses and readme
cp "${ROOT_DIR}/README.md" "${STAGE_DIR}/README.md" 2>/dev/null || true
for lic in "${ROOT_DIR}/LICENSE"*; do
    if [ -f "${lic}" ]; then
        cp "${lic}" "${STAGE_DIR}/" 2>/dev/null || true
    fi
done

ARCHIVE_NAME="${PACKAGE_NAME}.tar.gz"
echo "📦 Compressing into ${ARCHIVE_NAME}..."
(cd "${DIST_DIR}" && tar -czf "${ARCHIVE_NAME}" "${PACKAGE_NAME}")
rm -rf "${STAGE_DIR}"

CHECKSUM_FILE="${DIST_DIR}/asl-v${VERSION}-checksums.sha256"
if command -v shasum &> /dev/null; then
    (cd "${DIST_DIR}" && shasum -a 256 "${ARCHIVE_NAME}" >> "${CHECKSUM_FILE}")
elif command -v sha256sum &> /dev/null; then
    (cd "${DIST_DIR}" && sha256sum "${ARCHIVE_NAME}" >> "${CHECKSUM_FILE}")
fi

echo "========================================================"
echo "🎉 Successfully packaged: ${DIST_DIR}/${ARCHIVE_NAME}"
echo "   Artifact size: $(ls -lh "${DIST_DIR}/${ARCHIVE_NAME}" | awk '{print $5}')"
echo "========================================================"
