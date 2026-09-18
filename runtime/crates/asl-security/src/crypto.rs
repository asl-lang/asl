use asl_spec::{AslError, Result};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};

/// Gera um novo par de chaves Ed25519 retornando (chave_privada_hex, chave_publica_hex)
pub fn generate_keypair() -> (String, String) {
    let mut rng = rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let priv_hex = hex::encode(signing_key.to_bytes());
    let pub_hex = hex::encode(verifying_key.to_bytes());

    (priv_hex, pub_hex)
}

/// Assina um digest em string com uma chave privada Ed25519 de 32 bytes em hexadecimal
pub fn sign_digest(private_key_hex: &str, digest_str: &str) -> Result<String> {
    let clean_hex = private_key_hex
        .trim()
        .strip_prefix("asl:ed25519:priv:")
        .or_else(|| private_key_hex.trim().strip_prefix("0x"))
        .unwrap_or(private_key_hex.trim());

    let priv_bytes = hex::decode(clean_hex).map_err(|e| {
        AslError::CapabilityViolation(format!("Invalid hexadecimal private key: {}", e))
    })?;

    if priv_bytes.len() != 32 {
        return Err(AslError::CapabilityViolation(format!(
            "Ed25519 private key must be exactly 32 bytes (found: {})",
            priv_bytes.len()
        )));
    }

    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&priv_bytes);
    let signing_key = SigningKey::from_bytes(&key_arr);

    let signature = signing_key.sign(digest_str.as_bytes());
    Ok(hex::encode(signature.to_bytes()))
}

/// Derives Ed25519 public key (hexadecimal) from a hexadecimal private key
pub fn get_public_key(private_key_hex: &str) -> Result<String> {
    let clean_hex = private_key_hex
        .trim()
        .strip_prefix("asl:ed25519:priv:")
        .or_else(|| private_key_hex.trim().strip_prefix("0x"))
        .unwrap_or(private_key_hex.trim());

    let priv_bytes = hex::decode(clean_hex).map_err(|e| {
        AslError::CapabilityViolation(format!("Invalid hexadecimal private key: {}", e))
    })?;

    if priv_bytes.len() != 32 {
        return Err(AslError::CapabilityViolation(format!(
            "Ed25519 private key must be exactly 32 bytes (found: {})",
            priv_bytes.len()
        )));
    }

    let mut key_arr = [0u8; 32];
    key_arr.copy_from_slice(&priv_bytes);
    let signing_key = SigningKey::from_bytes(&key_arr);
    let pub_hex = hex::encode(signing_key.verifying_key().to_bytes());
    Ok(pub_hex)
}

/// Verifies whether the 64-byte Ed25519 signature is valid for digest and public key
pub fn verify_signature(
    public_key_hex: &str,
    digest_str: &str,
    signature_hex: &str,
) -> Result<bool> {
    let clean_pub = public_key_hex
        .trim()
        .strip_prefix("asl:ed25519:pub:")
        .or_else(|| public_key_hex.trim().strip_prefix("0x"))
        .unwrap_or(public_key_hex.trim());

    let pub_bytes = hex::decode(clean_pub).map_err(|e| {
        AslError::CapabilityViolation(format!("Invalid hexadecimal public key: {}", e))
    })?;

    if pub_bytes.len() != 32 {
        return Err(AslError::CapabilityViolation(format!(
            "Ed25519 public key must be exactly 32 bytes (found: {})",
            pub_bytes.len()
        )));
    }

    let mut pub_arr = [0u8; 32];
    pub_arr.copy_from_slice(&pub_bytes);
    let verifying_key = VerifyingKey::from_bytes(&pub_arr).map_err(|e| {
        AslError::CapabilityViolation(format!("Malformed Ed25519 public key: {}", e))
    })?;

    let clean_sig = signature_hex
        .trim()
        .strip_prefix("asl:ed25519:")
        .or_else(|| signature_hex.trim().strip_prefix("0x"))
        .unwrap_or(signature_hex.trim());

    let sig_bytes = hex::decode(clean_sig).map_err(|e| {
        AslError::CapabilityViolation(format!("Invalid hexadecimal signature: {}", e))
    })?;

    if sig_bytes.len() != 64 {
        return Ok(false);
    }

    let mut sig_arr = [0u8; 64];
    sig_arr.copy_from_slice(&sig_bytes);
    let sig = Signature::from_bytes(&sig_arr);

    Ok(verifying_key.verify(digest_str.as_bytes(), &sig).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen_sign_verify_cycle() {
        let (priv_key, pub_key) = generate_keypair();
        assert_eq!(priv_key.len(), 64);
        assert_eq!(pub_key.len(), 64);

        let derived_pub = get_public_key(&priv_key).expect("Public key derivation should succeed");
        assert_eq!(derived_pub, pub_key);

        let digest = "asl:sha256:19f41c0e7ca32215176eac4acae944be4f7da01fae1dbe4e60e33402efde52cb";
        let sig = sign_digest(&priv_key, digest).expect("Signing should succeed");
        assert_eq!(sig.len(), 128); // 64 bytes = 128 hex chars

        let is_valid = verify_signature(&pub_key, digest, &sig).expect("Verification should succeed");
        assert!(is_valid);

        // Tampered digest should fail verification
        let is_tampered = verify_signature(&pub_key, "asl:sha256:corrupted", &sig).unwrap();
        assert!(!is_tampered);
    }
}
