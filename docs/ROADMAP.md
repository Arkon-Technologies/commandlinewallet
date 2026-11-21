# Roadmap

Each phase requires a review of the security claims and tests before advancing. Dates are intentionally unset.

1. **Phase 1 — Workspace and architecture:** Rust workspace, `clw` preview, contracts, threat and security documents, local tests, cross-platform CI. No wallet operations.
2. **Phase 2 — Threat-model review and encrypted-vault prototype:** agree on backup/recovery format, cryptographic library choices, and failure behavior; prototype with test-only material.
3. **Phase 3 — macOS hardware-assisted vault prototype:** explicit Keychain/Secure Enclave capability detection and device-bound local unlock; prove independent restore.
4. **Phase 4 — Isolated signing and EVM testnet transactions:** narrow IPC, signer-derived review, approval binding, and testnet-only transfers.
5. **Phase 5 — Bitcoin and Solana integrations:** chain-specific parsing and transaction review, then testnet/devnet flows.
6. **Phase 6 — Windows/Linux TPM support:** device capability matrix, TPM-backed local unlock where available, and documented fallback policy.
7. **Phase 7 — Hardening, fuzzing, external audit, release preparation:** parser fuzzing, supply-chain review, reproducible builds, independent audit, and release process.
