use anyhow::{Context, Result};
use asl_core_traits::ParserPort;
use asl_parser::CommonMarkYamlParser;
use std::fs;
use std::path::{Path, PathBuf};

/// Handles the `asl keygen` command
pub fn handle_keygen(out: Option<PathBuf>) -> Result<()> {
    let (priv_hex, pub_hex) = asl_security::crypto::generate_keypair();
    let priv_str = format!("asl:ed25519:priv:{}", priv_hex);
    let pub_str = format!("asl:ed25519:pub:{}", pub_hex);

    if let Some(prefix) = out {
        let priv_path = prefix.with_extension("priv");
        let pub_path = prefix.with_extension("pub");
        fs::write(&priv_path, format!("{}\n", priv_str))
            .with_context(|| format!("Failed to save private key at {:?}", priv_path))?;
        fs::write(&pub_path, format!("{}\n", pub_str))
            .with_context(|| format!("Failed to save public key at {:?}", pub_path))?;
        println!("🔑 Ed25519 keypair saved successfully!");
        println!("Private Key:       {:?}", priv_path);
        println!("Public Key:        {:?}", pub_path);
        println!("Public Key (hex):  {}", pub_str);
    } else {
        println!("🔑 Ed25519 keypair generated successfully:");
        println!("Private Key:       {}", priv_str);
        println!("Public Key:        {}", pub_str);
    }
    Ok(())
}

/// Handles the `asl sign` command
pub fn handle_sign(skill_file: &Path, key: &str, parser: &CommonMarkYamlParser) -> Result<()> {
    let key_str = if Path::new(key).is_file() {
        fs::read_to_string(key)
            .with_context(|| format!("Failed to read private key from {:?}", key))?
    } else {
        key.to_string()
    };

    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Failed to parse ASL file for signing")?;

    let sig = asl_security::crypto::sign_digest(&key_str, &doc.digest)
        .with_context(|| "Failed to sign digest with the provided private key")?;

    let pubkey = asl_security::crypto::get_public_key(&key_str)
        .with_context(|| "Failed to derive public key from private key")?;

    let updated_content = inject_or_update_frontmatter(&content, &doc.digest, &sig, &pubkey)?;
    fs::write(skill_file, &updated_content)
        .with_context(|| format!("Failed to save signed file: {:?}", skill_file))?;

    // Update shadow projection with new signature and digest (Zero-Touch)
    if let Ok(signed_doc) = parser.parse(&updated_content) {
        let _ = asl_parser::project_shadow_markdown(skill_file, &signed_doc);
    }

    println!("✅ ASL file signed successfully!");
    println!("File:              {:?}", skill_file);
    println!("Digest:            {}", doc.digest);
    println!("Signature:         asl:ed25519:{}", sig);
    println!("Public Key:        asl:ed25519:pub:{}", pubkey);

    Ok(())
}

/// Handles the `asl verify` command
pub fn handle_verify(
    skill_file: &Path,
    pubkey: Option<String>,
    parser: &CommonMarkYamlParser,
) -> Result<()> {
    let content = fs::read_to_string(skill_file)
        .with_context(|| format!("Failed to read file: {:?}", skill_file))?;

    let doc = parser
        .parse(&content)
        .with_context(|| "Failed to parse ASL file for verification")?;

    let sig = doc
        .manifest
        .signature
        .as_ref()
        .with_context(|| "ASL file does not have 'signature' field in manifest")?;

    let (key_to_use, is_embedded) = match pubkey {
        Some(k) => {
            let key = if Path::new(&k).is_file() {
                fs::read_to_string(&k)?
            } else {
                k
            };
            (key, false)
        }
        None => {
            let key = doc
                .manifest
                .signer_pubkey
                .as_ref()
                .with_context(|| "Public key not provided via --pubkey and missing in manifest")?
                .clone();
            (key, true)
        }
    };

    let valid = asl_security::crypto::verify_signature(&key_to_use, &doc.digest, sig)
        .with_context(|| "Error during Ed25519 signature validation")?;

    if valid {
        println!("✅ Ed25519 signature VALID!");
        println!("File:              {:?}", skill_file);
        println!("Digest:            {}", doc.digest);
        println!("Public Key:        {}", key_to_use.trim());
        if is_embedded {
            println!("⚠️  Note: Public key extracted from file manifest (self-signed). Validates integrity against declared key, but does not attest author identity. For strict verification, use --pubkey <TRUSTED_KEY>.");
        }
    } else {
        eprintln!(
            "❌ Ed25519 signature INVALID for digest {}",
            doc.digest
        );
        anyhow::bail!("Signature validation failed: the file was modified or the public key does not match.");
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
        _ => anyhow::bail!("ASL file does not have valid '---' delimiters in frontmatter"),
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

    let clean_sig = signature
        .trim()
        .strip_prefix("asl:ed25519:")
        .unwrap_or(signature);
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
