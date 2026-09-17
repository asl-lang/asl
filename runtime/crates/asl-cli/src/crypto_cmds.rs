use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::CommonMarkYamlParser;
use std::fs;
use std::path::{Path, PathBuf};

/// Manipula o comando `asl keygen`
pub fn handle_keygen(out: Option<PathBuf>) -> Result<()> {
    let (priv_hex, pub_hex) = asl_security::crypto::generate_keypair();
    let priv_str = format!("asl:ed25519:priv:{}", priv_hex);
    let pub_str = format!("asl:ed25519:pub:{}", pub_hex);

    if let Some(prefix) = out {
        let priv_path = prefix.with_extension("priv");
        let pub_path = prefix.with_extension("pub");
        fs::write(&priv_path, format!("{}\n", priv_str))
            .with_context(|| format!("Falha ao salvar chave privada em {:?}", priv_path))?;
        fs::write(&pub_path, format!("{}\n", pub_str))
            .with_context(|| format!("Falha ao salvar chave pública em {:?}", pub_path))?;
        println!("🔑 Par de chaves Ed25519 salvo com sucesso!");
        println!("Chave Privada: {:?}", priv_path);
        println!("Chave Pública: {:?}", pub_path);
        println!("Chave Pública (hex): {}", pub_str);
    } else {
        println!("🔑 Par de chaves Ed25519 gerado com sucesso:");
        println!("Chave Privada: {}", priv_str);
        println!("Chave Pública: {}", pub_str);
    }
    Ok(())
}

/// Manipula o comando `asl sign`
pub fn handle_sign(skill_file: &Path, key: &str, parser: &CommonMarkYamlParser) -> Result<()> {
    let key_str = if Path::new(key).is_file() {
        fs::read_to_string(key).with_context(|| format!("Falha ao ler chave privada de {:?}", key))?
    } else {
        key.to_string()
    };

    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Erro ao analisar o arquivo ASL para assinatura")?;

    let sig = asl_security::crypto::sign_digest(&key_str, &doc.digest)
        .with_context(|| "Falha ao assinar o digest com a chave privada fornecida")?;

    let pubkey = asl_security::crypto::get_public_key(&key_str)
        .with_context(|| "Falha ao derivar a chave pública da chave privada")?;

    let updated_content = inject_or_update_frontmatter(&content, &doc.digest, &sig, &pubkey)?;
    fs::write(skill_file, &updated_content)
        .with_context(|| format!("Falha ao salvar arquivo assinado: {:?}", skill_file))?;

    // Atualiza projeção sombra com nova assinatura e digest (Zero-Touch)
    if let Ok(signed_doc) = parser.parse(&updated_content) {
        let _ = asl_parser::project_shadow_markdown(skill_file, &signed_doc);
    }

    println!("✅ Arquivo ASL assinado com sucesso!");
    println!("Arquivo:       {:?}", skill_file);
    println!("Digest:        {}", doc.digest);
    println!("Assinatura:    asl:ed25519:{}", sig);
    println!("Chave Pública: asl:ed25519:pub:{}", pubkey);

    Ok(())
}

/// Manipula o comando `asl verify`
pub fn handle_verify(
    skill_file: &Path,
    pubkey: Option<String>,
    parser: &CommonMarkYamlParser,
) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Falha ao ler arquivo: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Erro ao analisar arquivo ASL para verificação")?;

    let sig = doc
        .manifest
        .signature
        .as_ref()
        .with_context(|| "Arquivo ASL não possui campo 'signature' no manifesto")?;

    let key_to_use = match pubkey {
        Some(k) => {
            if Path::new(&k).is_file() {
                fs::read_to_string(&k)?
            } else {
                k
            }
        }
        None => doc
            .manifest
            .signer_pubkey
            .as_ref()
            .with_context(|| "Chave pública não fornecida via --pubkey e ausente no manifesto")?
            .clone(),
    };

    let valid = asl_security::crypto::verify_signature(&key_to_use, &doc.digest, sig)
        .with_context(|| "Erro durante a validação da assinatura Ed25519")?;

    if valid {
        println!("✅ Assinatura Ed25519 VÁLIDA!");
        println!("Arquivo:       {:?}", skill_file);
        println!("Digest:        {}", doc.digest);
        println!("Chave Pública: {}", key_to_use.trim());
    } else {
        eprintln!("❌ Assinatura Ed25519 INVÁLIDA para o digest {}", doc.digest);
        anyhow::bail!("Falha na validação da assinatura: o arquivo foi alterado ou a chave pública não confere.");
    }

    Ok(())
}

pub fn inject_or_update_frontmatter(
    content: &str,
    digest: &str,
    signature: &str,
    signer_pubkey: &str,
) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut start_idx = None;
    let mut end_idx = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if start_idx.is_none() {
                start_idx = Some(i);
            } else {
                end_idx = Some(i);
                break;
            }
        }
    }

    let (s, e) = match (start_idx, end_idx) {
        (Some(s), Some(e)) if s < e => (s, e),
        _ => anyhow::bail!("Arquivo ASL não possui delimitadores '---' válidos no frontmatter"),
    };

    let mut new_frontmatter = Vec::new();
    for line in &lines[s + 1..e] {
        let trimmed = line.trim();
        if !trimmed.starts_with("digest:")
            && !trimmed.starts_with("signature:")
            && !trimmed.starts_with("signer_pubkey:")
        {
            new_frontmatter.push(*line);
        }
    }

    let clean_sig = signature.trim().strip_prefix("asl:ed25519:").unwrap_or(signature);
    let clean_pub = signer_pubkey
        .trim()
        .strip_prefix("asl:ed25519:pub:")
        .unwrap_or(signer_pubkey);

    let formatted_digest = format!("digest: \"{}\"", digest);
    let formatted_sig = format!("signature: \"asl:ed25519:{}\"", clean_sig);
    let formatted_pub = format!("signer_pubkey: \"asl:ed25519:pub:{}\"", clean_pub);

    new_frontmatter.push(&formatted_digest);
    new_frontmatter.push(&formatted_sig);
    new_frontmatter.push(&formatted_pub);

    let mut result = Vec::new();
    for line in &lines[..=s] {
        result.push(line.to_string());
    }
    for line in new_frontmatter {
        result.push(line.to_string());
    }
    for line in &lines[e..] {
        result.push(line.to_string());
    }

    let mut output = result.join("\n");
    if content.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}
