use aes_gcm::{
    aead::{
        generic_array::{typenum::U12, GenericArray},
        Aead, AeadCore, KeyInit,
    },
    Aes256Gcm, Key,
};
use argon2::Argon2;
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};

use crate::error::DaemonError;

const SALT_LEN: usize = 16;
const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedBlob {
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

fn derive_key(passphrase: &str, salt: &[u8]) -> Result<[u8; KEY_LEN], DaemonError> {
    let mut key = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|e| DaemonError::Internal(format!("key derivation: {e}")))?;
    Ok(key)
}

/// Encrypt `plaintext` with AES-256-GCM using a passphrase-derived Argon2id key.
pub fn encrypt(passphrase: &str, plaintext: &[u8]) -> Result<EncryptedBlob, DaemonError> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    let key_bytes = derive_key(passphrase, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // generate_nonce returns GenericArray<u8, U12> — the correct nonce type
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| DaemonError::Internal(format!("encryption: {e}")))?;

    Ok(EncryptedBlob {
        salt: B64.encode(salt),
        nonce: B64.encode(nonce.as_slice()),
        ciphertext: B64.encode(ciphertext),
    })
}

/// Decrypt an `EncryptedBlob` back to plaintext.
pub fn decrypt(passphrase: &str, blob: &EncryptedBlob) -> Result<Vec<u8>, DaemonError> {
    let salt = B64
        .decode(&blob.salt)
        .map_err(|_| DaemonError::Internal("invalid salt encoding".into()))?;
    let nonce_bytes = B64
        .decode(&blob.nonce)
        .map_err(|_| DaemonError::Internal("invalid nonce encoding".into()))?;
    let ciphertext = B64
        .decode(&blob.ciphertext)
        .map_err(|_| DaemonError::Internal("invalid ciphertext encoding".into()))?;

    if nonce_bytes.len() != NONCE_LEN {
        return Err(DaemonError::Internal("invalid nonce length".into()));
    }

    let key_bytes = derive_key(passphrase, &salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Reconstruct the 12-byte nonce as GenericArray<u8, U12>
    let nonce: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce_bytes);

    cipher
        .decrypt(&nonce, ciphertext.as_slice())
        .map_err(|_| DaemonError::Internal("decryption failed: wrong passphrase or corrupted data".into()))
}
