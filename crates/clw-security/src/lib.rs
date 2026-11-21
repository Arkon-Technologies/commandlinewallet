//! Descriptive capability and review types. They enforce no security in Phase 1.

use clw_core::Blockchain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityStatus {
    NotImplemented,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformTarget {
    MacOs,
    Windows,
    Linux,
    Other,
}

impl PlatformTarget {
    pub const fn current() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOs
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else {
            Self::Other
        }
    }

    pub const fn planned_hardware(self) -> &'static str {
        match self {
            Self::MacOs => "Apple Secure Enclave / Keychain (planned, not detected)",
            Self::Windows => "TPM 2.0 / Windows security APIs (planned, not detected)",
            Self::Linux => "TPM 2.0 where supported (planned, not detected)",
            Self::Other => "no hardware integration planned for this platform",
        }
    }
}

/// Data a future signer must derive from parsed transaction bytes, not UI text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionReview {
    pub chain: Blockchain,
    pub destination: String,
    pub amount: String,
    pub fee: String,
}

/// Future approval must bind the human decision to reviewed transaction bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_one_capability_is_unimplemented() {
        assert!(
            PlatformTarget::current()
                .planned_hardware()
                .contains("not detected")
                || PlatformTarget::current() == PlatformTarget::Other
        );
    }
}
