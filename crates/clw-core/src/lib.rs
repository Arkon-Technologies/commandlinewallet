//! Shared, non-secret domain vocabulary. No network or cryptographic code belongs here.

use std::fmt;

/// Chains targeted by the project. A target is not an implemented integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Blockchain {
    Bitcoin,
    Evm,
    Solana,
}

impl Blockchain {
    pub const ALL: [Self; 3] = [Self::Bitcoin, Self::Evm, Self::Solana];

    pub const fn name(self) -> &'static str {
        match self {
            Self::Bitcoin => "Bitcoin",
            Self::Evm => "Ethereum / EVM",
            Self::Solana => "Solana",
        }
    }
}

/// Stable error categories; no secret material may appear in error text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WalletError {
    Unsupported,
    InvalidRequest,
    VaultUnavailable,
    ApprovalRejected,
    Internal,
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Unsupported => "operation is not implemented",
            Self::InvalidRequest => "invalid wallet request",
            Self::VaultUnavailable => "vault is unavailable",
            Self::ApprovalRejected => "transaction approval was rejected",
            Self::Internal => "internal wallet error",
        };
        f.write_str(message)
    }
}

impl std::error::Error for WalletError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_are_named_without_claiming_support() {
        assert_eq!(
            Blockchain::ALL.map(Blockchain::name),
            ["Bitcoin", "Ethereum / EVM", "Solana"]
        );
    }

    #[test]
    fn errors_do_not_echo_request_data() {
        assert_eq!(
            WalletError::InvalidRequest.to_string(),
            "invalid wallet request"
        );
    }
}
