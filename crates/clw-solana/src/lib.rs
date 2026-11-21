//! Solana adapter boundary. No RPC, message parsing, address derivation, or signing yet.

use clw_core::Blockchain;

pub const CHAIN: Blockchain = Blockchain::Solana;
pub const IMPLEMENTED: bool = false;
