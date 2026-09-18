---
name: asl-english-only
description: >-
  Enforces a strict English-only policy across the entire ASL codebase, CLI
  tooling, error messages, scripts, documentation, and agent skills. Prohibits
  the use of Portuguese or other non-English languages in code, comments, or CLI outputs.
---

# Skill: English-Only Policy Enforcement for ASL

This skill MUST be adhered to by all AI agents, contributors, and CI workflows modifying or authoring code within the ASL (Agent Skill Language) repository.

## 1. Core Mandate
All code, scripts, command-line interfaces, error messages, docstrings, comments, specifications, and documentation MUST be written strictly in **English**.

The use of Portuguese, Spanish, or any other non-English language is strictly forbidden in:
- Runtime Rust crates (`runtime/crates/*`)
- CLI commands, help texts, doc comments, outputs, and error contexts (`asl-cli`)
- Shell scripts, installer scripts, and CI workflows (`install.sh`, `scripts/*`)
- Documentation, architecture decision records (ADRs), specifications, and study docs (`docs/*`)
- Examples and canonical ASL files (`examples/*.skill`, `examples/*.tool`, `examples/*.asl`)

## 2. Specific Requirements by Area

### A. CLI Tooling & Error Messages (`asl-cli`)
- Subcommands, flags, descriptions, and argument help strings must be in idiomatic English.
- All errors returned via `anyhow::bail!`, `with_context`, or `eprintln!` must be clear, actionable English messages.
- Success messages, status indicators, and summary outputs must be in English.

### B. Shell & Installer Scripts (`install.sh`, `scripts/*`)
- All user-facing echoes, logs, warnings, and error messages must be in English.
- Instructions for missing dependencies (e.g. `cargo`, `git`, `rustc`) must provide English commands and links.
- Environment path configuration instructions must be in English.

### C. Source Code & Comments
- No non-English comments (`//`, `///`, `//!`).
- No non-English identifiers or test descriptions.
- Diacritics commonly used in Portuguese (`á, é, í, ó, ú, ç, ã, õ, ê, ô, à`) are prohibited in code and comments (except in explicit multi-lingual tokenizer test fixtures).

## 3. Automated Verification

Before submitting any changes, execute the automated English-only audit script:
```bash
./scripts/check_english_only.sh
```

Ensure that:
1. `./scripts/check_english_only.sh` returns exit code `0`.
2. `./scripts/guardrail_check.sh` completes with all checks passing.
3. `cargo test` in `runtime/` passes 100%.
