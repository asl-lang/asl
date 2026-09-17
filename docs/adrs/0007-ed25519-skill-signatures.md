# ADR-0007: Assinatura Criptográfica Ed25519 & Cadeia de Custódia de Skills

- **Status**: Aceito
- **Data**: 2026-09-17
- **Autores**: Jean Catarina (Cadente)
- **Decisores**: Conselho de Arquitetura ASL / Cadente
- **Crates Afetadas**: `asl-spec`, `asl-security`, `asl-cli`

---

## 1. Contexto e Problema

No ecossistema de agentes autônomos, arquivos `.skill` são distribuídos por repositórios Git, registries descentralizados ou trocados dinamicamente em tempo de execução. O ASL 3.0 já possui o campo `digest: "asl:sha256:..."` que atesta a integridade do arquivo contra corrupção acidental de bits.

No entanto, apenas o hash SHA-256 não comprova **autoria nem proveniência**:
1. Um atacante ou agente comprometido pode alterar o código determinístico ou injetar instruções na Seção Semântica e recalcular um novo digest SHA-256 válido.
2. Agentes de alta criticidade precisam de garantia matemática de que a skill foi homologada e assinada pela chave de **Jean Catarina** ou pela organização **Cadente**.

---

## 2. Proposta Detalhada da Decisão

Implementar assinatura assimétrica de curva elíptica **Ed25519** (Edwards-curve Digital Signature Algorithm) para os manifestos de skills, criando uma cadeia de custódia criptográfica inviolável.

### 2.1 Modelo de Dados no Frontmatter YAML (`asl-spec`)

O manifesto `SkillManifest` passa a reconhecer os campos:
```yaml
---
asl_version: "3.0"
digest: "asl:sha256:19f41c0e7ca32215176eac4acae944be4f7da01fae1dbe4e60e33402efde52cb"
signature: "asl:ed25519:7a8b...64bytes"
signer_pubkey: "asl:ed25519:pub:1c2d...32bytes"
name: "git-conventional-commit"
---
```

### 2.2 O que é Assinado?
A assinatura incide sobre o **digest canônico SHA-256** do `.skill` (que é invariante à própria assinatura, pois o cálculo do digest exclui as linhas `digest:`, `signature:` e `signer_pubkey:` do frontmatter).
Isso desacopla a assinatura da formatação e previne ataques de extensão de comprimento.

### 2.3 Módulo Criptográfico (`asl-security::crypto`)
Utiliza `ed25519-dalek` em Rust puro:
- `generate_keypair() -> (String, String)`: Gera par de chaves privada (seed 32 bytes) e pública (32 bytes).
- `sign_digest(private_key_hex: &str, digest_str: &str) -> Result<String>`: Gera assinatura de 64 bytes em hexadecimal.
- `verify_signature(public_key_hex: &str, digest_str: &str, signature_hex: &str) -> Result<bool>`: Verifica autenticidade.

### 2.4 Novos Comandos no CLI (`asl-cli`)
- `asl keygen`: Gera novo par de chaves Ed25519.
- `asl sign <skill> --key <privkey>`: Assina o digest da skill e atualiza o frontmatter.
- `asl verify <skill> [--pubkey <pubkey>]`: Audita criptograficamente a assinatura.
- `asl check <skill>`: Se a skill possuir assinatura, verifica automaticamente a validade matemática.

---

## 3. Alternativas Consideradas

- **Alternativa A: Usar RSA 4096 bits**:
  - *Descarte*: Chaves e assinaturas gigantescas que poluem o frontmatter e aumentam a latência de verificação.
- **Alternativa B: Usar GPG / PGP externo**:
  - *Descarte*: Dependência de binários externos do sistema operacional, violando o Axioma 2 (Zero dependências externas).
- **Alternativa C: Ed25519 nativo em Rust (Escolhida)**:
  - *Justificativa*: Chaves ultracompactas (32 bytes), assinaturas de 64 bytes, verificação ultraveloz em microssegundos e padrão global de criptografia moderna.

---

## 4. Consequências e Trade-offs

- **Positivas**:
  - Garantia de não-repúdio e autoria comprovada de Jean Catarina/Cadente.
  - Prevenção absoluta contra adulteração de instruções semânticas ou código determinístico.
- **Negativas**:
  - Uma dependência criptográfica adicional (`ed25519-dalek` puro em Rust).

---

## 5. Conformidade com os 7 Axiomas do ASL

- [x] **Axioma 1 (Atomicidade)**: A assinatura é encapsulada dentro do próprio arquivo atômico `.skill`.
- [x] **Axioma 2 (Zero dependências externas)**: Implementado puramente em Rust.
- [x] **Axioma 3 (Confinamento ocap)**: Operação pura de transformação criptográfica em memória.
- [x] **Axioma 4 (Isolamento hexagonal)**: Módulo criptográfico contido em `asl-security`.
- [x] **Axioma 5 (Término determinístico)**: A verificação Ed25519 possui tempo de execução constante ($O(1)$) sem loops unbounded.
- [x] **Axioma 6 (Prefixo estático)**: Não altera a Seção Semântica.
- [x] **Axioma 7 (Limite de < 400 linhas)**: `asl-security/src/crypto.rs` terá < 150 linhas.
