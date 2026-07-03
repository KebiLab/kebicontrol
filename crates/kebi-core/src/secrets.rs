//! Windows DPAPI-based secret storage.
//!
//! Plain API keys are never written to disk — only DPAPI-encrypted
//! ciphertext. Made by KebiLab

use base64::Engine;
use thiserror::Error;

#[cfg(windows)]
mod imp {
    use super::*;
    use windows::Win32::Foundation::LocalFree;
    use windows::Win32::Security::Cryptography::{
        CryptProtectData, CryptUnprotectData, CRYPT_INTEGER_BLOB, CRYPTPROTECT_LOCAL_MACHINE,
    };

    pub fn protect(plaintext: &str) -> Result<String, SecretError> {
        let input = plaintext.as_bytes();
        let mut input_blob = CRYPT_INTEGER_BLOB {
            cbData: input.len() as u32,
            pbData: input.as_ptr() as *mut _,
        };
        let mut output_blob = CRYPT_INTEGER_BLOB::default();
        unsafe {
            CryptProtectData(
                &mut input_blob,
                None,
                None,
                None,
                None,
                CRYPTPROTECT_LOCAL_MACHINE,
                &mut output_blob,
            )
        }
        .map_err(|e| SecretError::Protect(e.to_string()))?;

        let slice = unsafe {
            std::slice::from_raw_parts(output_blob.pbData, output_blob.cbData as usize)
        };
        let encoded = base64::engine::general_purpose::STANDARD.encode(slice);
        unsafe { let _ = LocalFree(output_blob.pbData); }
        Ok(encoded)
    }

    pub fn unprotect(ciphertext_b64: &str) -> Result<String, SecretError> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(ciphertext_b64)
            .map_err(|e| SecretError::Base64(e.to_string()))?;
        let mut input_blob = CRYPT_INTEGER_BLOB {
            cbData: bytes.len() as u32,
            pbData: bytes.as_ptr() as *mut _,
        };
        let mut output_blob = CRYPT_INTEGER_BLOB::default();
        unsafe {
            CryptUnprotectData(
                &mut input_blob,
                None,
                None,
                None,
                None,
                CRYPTPROTECT_LOCAL_MACHINE,
                &mut output_blob,
            )
        }
        .map_err(|e| SecretError::Unprotect(e.to_string()))?;
        let slice = unsafe {
            std::slice::from_raw_parts(output_blob.pbData, output_blob.cbData as usize)
        };
        let s = String::from_utf8_lossy(slice).to_string();
        unsafe { let _ = LocalFree(output_blob.pbData); }
        Ok(s)
    }
}

#[cfg(not(windows))]
mod imp {
    use super::*;
    pub fn protect(plaintext: &str) -> Result<String, SecretError> {
        Ok(plaintext.to_string())
    }
    pub fn unprotect(ciphertext_b64: &str) -> Result<String, SecretError> {
        Ok(ciphertext_b64.to_string())
    }
}

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("DPAPI protect failed: {0}")]
    Protect(String),
    #[error("DPAPI unprotect failed: {0}")]
    Unprotect(String),
    #[error("Base64 decode failed: {0}")]
    Base64(String),
    #[error("Empty ciphertext")]
    Empty,
}

pub fn protect(plaintext: &str) -> Result<String, SecretError> {
    imp::protect(plaintext)
}

pub fn unprotect(ciphertext: &str) -> Result<String, SecretError> {
    if ciphertext.is_empty() {
        return Err(SecretError::Empty);
    }
    imp::unprotect(ciphertext)
}
