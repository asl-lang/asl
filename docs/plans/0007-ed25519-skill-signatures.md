# Plano de Implementação: Assinatura Criptográfica Ed25519 & Cadeia de Custódia de Skills

- **ADR Vinculado**: `docs/adrs/0007-ed25519-skill-signatures.md`
- **Data**: 2026-09-17
- **Responsável**: Jean Catarina (Cadente)
- **Status**: Concluído (4/4 Fases - 100%)
- **Meta**: Prover assinatura digital Ed25519 e verificação de proveniência de arquivos `.skill`, garantindo autenticidade e não-repúdio na distribuição de skills.

---

## Fase 1: Suporte a Metadados de Assinatura no `SkillManifest` (`asl-spec`)

### 1.1 Objetivo da Fase
Adicionar os campos opcionais `signature` e `signer_pubkey` à struct `SkillManifest` em `asl-spec`.

### 1.2 Código a Implementar
No arquivo `runtime/crates/asl-spec/src/lib.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub asl_version: String,
    #[serde(default)]
    pub digest: Option<String>,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub signer_pubkey: Option<String>,
    pub name: String,
    ...
}
```

### 1.3 Verificação Local
```bash
cargo test -p asl-spec
```

---

## Fase 2: Módulo Criptográfico Ed25519 (`asl-security::crypto`)

### 2.1 Objetivo da Fase
Criar `runtime/crates/asl-security/src/crypto.rs` com funções para geração de chaves, assinatura de digest e verificação de assinatura Ed25519.

### 2.2 Código a Implementar
```rust
pub fn generate_keypair() -> (String, String);
pub fn sign_digest(private_key_hex: &str, digest_str: &str) -> Result<String>;
pub fn verify_signature(public_key_hex: &str, digest_str: &str, signature_hex: &str) -> Result<bool>;
```

### 2.3 Testes Unitários da Fase
- Geração de par de chaves, assinatura de mensagem/digest e verificação com sucesso.
- Rejeição de assinatura corrompida ou chave incompatível.

### 2.4 Verificação Local
```bash
cargo test -p asl-security
cargo clippy -p asl-security -- -D warnings
```

---

## Fase 3: Comandos CLI `keygen`, `sign`, `verify` e Auditoria em `check`

### 3.1 Objetivo da Fase
Expandir `asl-cli`:
- `asl keygen`: Imprime chaves geradas em hexadecimal.
- `asl sign <skill> --key <privkey>`: Assina o digest e atualiza o cabeçalho do arquivo `.skill`.
- `asl verify <skill> [--pubkey <pubkey>]`: Valida a assinatura contra a chave pública informada ou contida no manifesto.
- `asl check <skill>`: Se houver assinatura, valida a integridade matemática da assinatura.

### 3.2 Verificação Local
```bash
cargo check -p asl-cli
```

---

## Fase 4: Validação de Guardrails e Teste End-to-End

### 4.1 Objetivo da Fase
Executar todos os guardrails do projeto e teste com skill real.

### 4.2 Verificação Local
```bash
./scripts/guardrail_check.sh
```

### 4.3 Finalização (Commit & Push)
```bash
git add runtime/crates/asl-spec runtime/crates/asl-security runtime/crates/asl-cli docs/adrs/ docs/plans/
git commit -m "feat(security): implementar assinatura Ed25519 e cadeia de custodia de skills (Marco 4)"
git push origin main
```
