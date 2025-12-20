use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};

const NONCE_LEN: usize = 12;

pub struct Encryptor {
    key: Vec<u8>,
    rng: SystemRandom,
}

impl Encryptor {
    pub fn new() -> Result<Self> {
        let key_str = std::env::var("ENCRYPTION_KEY")
            .unwrap_or_else(|_| "default-32-byte-encryption-key!".to_string());
        
        let mut key = vec![0u8; 32];
        let key_bytes = key_str.as_bytes();
        let len = std::cmp::min(key_bytes.len(), 32);
        key[..len].copy_from_slice(&key_bytes[..len]);

        Ok(Self {
            key,
            rng: SystemRandom::new(),
        })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(|_| anyhow!("Failed to create encryption key"))?;
        
        let sealing_key = LessSafeKey::new(unbound_key);

        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow!("Failed to generate nonce"))?;

        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut in_out = plaintext.as_bytes().to_vec();
        sealing_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|_| anyhow!("Encryption failed"))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);

        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String> {
        let data = general_purpose::STANDARD.decode(ciphertext)
            .map_err(|_| anyhow!("Failed to decode base64"))?;

        if data.len() < NONCE_LEN {
            return Err(anyhow!("Invalid ciphertext"));
        }

        let (nonce_bytes, encrypted) = data.split_at(NONCE_LEN);
        
        let mut nonce_array = [0u8; NONCE_LEN];
        nonce_array.copy_from_slice(nonce_bytes);
        let nonce = Nonce::assume_unique_for_key(nonce_array);

        let unbound_key = UnboundKey::new(&AES_256_GCM, &self.key)
            .map_err(|_| anyhow!("Failed to create decryption key"))?;
        
        let opening_key = LessSafeKey::new(unbound_key);

        let mut in_out = encrypted.to_vec();
        let decrypted = opening_key.open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|_| anyhow!("Decryption failed"))?;

        String::from_utf8(decrypted.to_vec())
            .map_err(|_| anyhow!("Invalid UTF-8"))
    }
}
