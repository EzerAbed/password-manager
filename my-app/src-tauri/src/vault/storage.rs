use crate::vault::{Vault};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use sha2::Sha256;
use pbkdf2::pbkdf2_hmac;
use rand::{RngCore, OsRng};
use std::{fs, path::PathBuf};

const PBKDF2_ROUNDS: u32 = 100_000;

pub fn save_vault(vault: &Vault, password: &str, path: &PathBuf) -> Result<(), String>{
    
    //random salt and nonce
    let mut salt = [0u8; 16]; 
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut salt); 
    OsRng.fill_bytes(&mut nonce_bytes);

    //derive key
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, PBKDF2_ROUNDS, &mut key); 

    //serialize vault to JSON
    let json = serde_json::to_string(vault).map-err(|e| e.to_string())?;
    
    //encrypt
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, json.as_bytes()).map_err(|e| e.to_string())?;

    // write salt + nonce + ciphertext to file
    let mut file_contents = Vec::new();
    file_contents.extend_from_slice(&salt);
    file_contents.extend_from_slice(&nonce_bytes);
    file_contents.extend_from_slice(&ciphertext);

    fs::write(path, file_contents).map_err(|e| e.to_string())?; 

    Ok(())
}

pub fn load_vault(password: &str, path: &PathBuf) -> Result<Vault, String> {
    // 1. read file bytes
    // 2. split out salt, nonce, ciphertext
    // 3. derive key from password + salt
    // 4. decrypt ciphertext
    // 5. deserialize JSON back into Vault
}