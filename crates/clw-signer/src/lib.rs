//! Contracts for a future isolated signer. No signing implementation exists.
//!
//! The signer must parse canonical chain transaction data itself and produce an
//! independent review before approval. It must not accept arbitrary messages.

use clw_core::{Blockchain, WalletError};

/// Supported transaction classes are intentionally narrow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransactionKind {
    BitcoinTransfer,
    EvmNativeTransfer,
    SolanaSystemTransfer,
}

impl TransactionKind {
    pub const fn chain(self) -> Blockchain {
        match self {
            Self::BitcoinTransfer => Blockchain::Bitcoin,
            Self::EvmNativeTransfer => Blockchain::Evm,
            Self::SolanaSystemTransfer => Blockchain::Solana,
        }
    }
}

/// Opaque transport request; a future signer must independently decode and
/// validate the payload for its declared transaction kind before any approval.
/// This type alone conveys no authorization and has no signing method.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SigningRequest {
    pub kind: TransactionKind,
    pub unsigned_transaction: Vec<u8>,
}

/// Future output is a chain transaction signature, never private key material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignatureResponse {
    pub signed_transaction: Vec<u8>,
}

/// No implementation is supplied in Phase 1.
pub trait TransactionSigner {
    fn sign_transaction(
        &mut self,
        request: SigningRequest,
    ) -> Result<SignatureResponse, WalletError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_kinds_are_bound_to_chains() {
        assert_eq!(
            TransactionKind::BitcoinTransfer.chain(),
            Blockchain::Bitcoin
        );
        assert_eq!(TransactionKind::EvmNativeTransfer.chain(), Blockchain::Evm);
        assert_eq!(
            TransactionKind::SolanaSystemTransfer.chain(),
            Blockchain::Solana
        );
    }
}
