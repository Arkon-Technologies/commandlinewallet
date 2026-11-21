# Threat model

This document describes intended controls. **Phase 1 mitigates none of the wallet threats below because no wallet operations exist.** “Can mitigate” means a future implementation could reduce the risk after review and testing; it does not assert an active guarantee.

| Threat | Intended mitigation | Residual risk / limit |
| --- | --- | --- |
| Attacker-controlled RPC responses | Treat all responses as untrusted; validate chain ID, transaction data, confirmations, and balances independently where possible. | A malicious or eclipsed endpoint can hide transactions, distort fee advice, or delay service. |
| Malicious transaction payloads | Strict, chain-specific canonical parsing and allowlisted transaction classes inside the signer; reject unknown fields and blind signing. | Parser bugs or complex contract effects can still mislead review. |
| Malicious dApps and transaction substitution | Bind approval to exact parsed bytes and display signer-derived details. No generic message-signing interface. | A user can approve a malicious but accurately displayed action. Trusted display is unresolved. |
| Compromised terminal or coordinator | Separate signer process and vault; signer independently validates proposals. No private keys returned over IPC. | The same host can alter the displayed approval or interfere with processes. Process isolation is limited. |
| Stolen powered-off device | Encrypted vault, hardware-assisted local unwrap, rate-limited platform authentication where available. | Weak recovery material, platform flaws, or stolen unlocked backups can defeat protection. |
| Host malware | Minimize key lifetime and process privileges; prevent secret logging; narrow IPC. | Malware with user privileges may read signer memory, capture approvals, or inject input. No strong guarantee. |
| Compromised OS/kernel | None reliable on the same host. | Kernel-level control can inspect or manipulate processes, memory, and display. Outside CLW's security guarantees. |
| Dependency and supply-chain compromise | Pin lockfile, minimize dependencies, audit updates, verify CI and release provenance, arrange external audit. | Maintainer, registry, build-tool, or update compromise remains possible. |
| Backup and recovery failure | Portable encrypted backup with explicit restore verification and independent recovery path. | Loss of backup or passphrase can make funds unrecoverable; exposed backup may enable theft. |

The model assumes the user can verify transaction details on the host display, but that display is not independent of the operating system. CLW cannot offer dedicated hardware wallet equivalence. Future security reviews must specify concrete attacker capabilities, supported transaction classes, and test evidence before any protection claim is made.
