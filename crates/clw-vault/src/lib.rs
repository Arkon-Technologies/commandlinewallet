//! Vault lifecycle vocabulary only. No encryption, storage, or recovery exists yet.
//!
//! A future vault must use reviewed authenticated encryption, bind local unlock to
//! platform hardware where available, and permit recovery without the original device.

use clw_core::WalletError;

/// Planned lifecycle states. Phase 1 always reports unavailable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VaultState {
    Unavailable,
    Locked,
    Unlocked,
}

/// Non-operational boundary for a future vault implementation.
pub trait VaultLifecycle {
    fn state(&self) -> VaultState;
    fn lock(&mut self) -> Result<(), WalletError>;
}

// A future recovery flow must authenticate and verify backup material before use.
// Backup format, KDF, AEAD, and user verification are deliberately undecided.
