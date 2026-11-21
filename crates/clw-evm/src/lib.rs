//! EVM adapter boundary. No RPC, transaction parsing, address derivation, or signing yet.

use clw_core::Blockchain;

pub const CHAIN: Blockchain = Blockchain::Evm;
pub const IMPLEMENTED: bool = false;
