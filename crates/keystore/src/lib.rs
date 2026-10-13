//! Key custody abstraction: the only place a signing key is ever used.
//!
//! Defines the [`KeyBackend`] trait with two slots for rotation overlap. No real backend
//! (HSM, encrypted keyfile) is implemented yet. Key material must never be logged.
#![forbid(unsafe_code)]

use zeroize::Zeroizing;

/// Which key slot to use. Two slots exist so a rotation can overlap old and new keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySlot {
    /// The key currently in service.
    Primary,
    /// The incoming (or outgoing) key during a rotation overlap window.
    Secondary,
}

/// Errors from a key backend. Never carries key material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// The requested slot holds no key.
    SlotEmpty,
    /// The backend failed to produce a signature.
    SignFailed,
}

/// A 32-byte ed25519 public key.
pub type PublicKey = [u8; 32];
/// A 64-byte ed25519 signature.
pub type Signature = [u8; 64];

/// A signing key custodian. Implementations must never export private key bytes.
pub trait KeyBackend {
    /// Public key held in `slot`.
    fn public_key(&self, slot: KeySlot) -> Result<PublicKey, KeyError>;

    /// Sign `message` with the key in `slot`.
    fn sign(&self, slot: KeySlot, message: &[u8]) -> Result<Signature, KeyError>;
}

/// Wrapper for secret bytes held in memory; zeroed on drop.
pub type Secret = Zeroizing<Vec<u8>>;

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

    /// In-memory backend with throwaway keys generated at test runtime.
    struct MemBackend {
        primary: SigningKey,
        secondary: Option<SigningKey>,
    }

    fn fresh_key() -> SigningKey {
        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed).expect("os randomness");
        SigningKey::from_bytes(&seed)
    }

    impl MemBackend {
        fn slot(&self, slot: KeySlot) -> Result<&SigningKey, KeyError> {
            match slot {
                KeySlot::Primary => Ok(&self.primary),
                KeySlot::Secondary => self.secondary.as_ref().ok_or(KeyError::SlotEmpty),
            }
        }
    }

    impl KeyBackend for MemBackend {
        fn public_key(&self, slot: KeySlot) -> Result<PublicKey, KeyError> {
            Ok(self.slot(slot)?.verifying_key().to_bytes())
        }

        fn sign(&self, slot: KeySlot, message: &[u8]) -> Result<Signature, KeyError> {
            Ok(self.slot(slot)?.sign(message).to_bytes())
        }
    }

    #[test]
    fn signs_and_verifies_per_slot() {
        let backend = MemBackend {
            primary: fresh_key(),
            secondary: None,
        };
        let sig = backend.sign(KeySlot::Primary, b"msg").unwrap();
        let pk = VerifyingKey::from_bytes(&backend.public_key(KeySlot::Primary).unwrap()).unwrap();
        assert!(
            pk.verify(b"msg", &ed25519_dalek::Signature::from_bytes(&sig))
                .is_ok()
        );
        assert_eq!(
            backend.sign(KeySlot::Secondary, b"msg"),
            Err(KeyError::SlotEmpty)
        );
    }
}
