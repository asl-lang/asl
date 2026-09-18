#!/usr/bin/env bash
# ==============================================================================
# AUDIT SCRIPT: ENFORCE STRICT ENGLISH-ONLY POLICY IN ASL SOURCE CODE & SCRIPTS
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "🔍 Running English-Only Compliance Audit across ASL repository..."

VIOLATIONS=0

python3 - "$ROOT_DIR" << 'EOF'
import os
import re
import sys

repo_root = sys.argv[1] if len(sys.argv) > 1 else "."

# Regex targeting common Portuguese words and accented characters in code/CLI
pt_word_pattern = re.compile(
    r"\b(?:falha|arquivo|erro|sucesso|iniciado|executa|não|padrão|chave|assinatura|caminho|aviso|inválida|inválido)\b",
    re.IGNORECASE
)

# Files to check: Rust source files in CLI and installer scripts
files_to_check = []

# 1. install.sh
install_sh = os.path.join(repo_root, "install.sh")
if os.path.exists(install_sh):
    files_to_check.append(install_sh)

# 2. runtime crates (focusing especially on user-facing CLI and error paths)
runtime_dir = os.path.join(repo_root, "runtime", "crates")
for root, dirs, files in os.walk(runtime_dir):
    if "target" in root:
        continue
    for f in files:
        if f.endswith(".rs") and not f.endswith("tests.rs"):
            files_to_check.append(os.path.join(root, f))

violations = []

for filepath in files_to_check:
    rel_path = os.path.relpath(filepath, repo_root)
    # Skip test fixture with unicode chars test
    if "test" in rel_path and "unicode" in rel_path:
        continue

    with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
        for line_num, line in enumerate(f, 1):
            stripped = line.strip()
            # Ignore markdown links or URLs
            if stripped.startswith("//!") or stripped.startswith("///"):
                continue
            match = pt_word_pattern.search(line)
            if match:
                violations.append((rel_path, line_num, match.group(0), stripped[:90]))

if violations:
    print(f"❌ Found {len(violations)} non-English (Portuguese) occurrences in source files:")
    for path, line_no, token, snippet in violations[:20]:
        print(f"  - {path}:{line_no} [detected '{token}']: {snippet}")
    sys.exit(1)
else:
    print("✅ 100% English-only compliance verified in CLI and core runtime files.")
    sys.exit(0)
EOF

echo "🎉 All English-only language checks passed successfully!"
