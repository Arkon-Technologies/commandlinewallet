//! Bitcoin adapter boundary. No RPC, PSBT parsing, address derivation, or signing yet.

use clw_core::Blockchain;

pub const CHAIN: Blockchain = Blockchain::Bitcoin;
pub const IMPLEMENTED: bool = false;
