# Security model

## Current state

Phase 1 has no wallet, vault, cryptographic signing, hardware detection, transaction validation, approval enforcement, or network access. All capability output says “not implemented.” Interfaces are design boundaries only.

## Planned hardware assistance

Hardware-assisted vault unlocking means a device-bound key or platform credential helps protect an encrypted secret at rest. It is different from hardware-isolated transaction signing: conventional Bitcoin/Ethereum transactions use secp256k1, and Solana uses Ed25519. Apple's Secure Enclave generally does not directly sign these conventional transactions. CLW must not market local wrapping as hardware-isolated signing or as equivalent to a dedicated hardware wallet.

TPM 2.0 behavior differs among Windows and Linux devices, firmware, drivers, access policies, and available algorithms. Capability detection must be explicit and tested per device. Failure or absence of hardware support must be reported honestly; fallback behavior needs a separate reviewed policy.

For conventional chains, decrypted seed or private-key material may temporarily exist in signer-process memory. The signer must never send it to the coordinator, terminal, logs, crash diagnostics, or network. Memory locking and zeroization can reduce exposure but cannot guarantee erasure from registers, copies, swap, crash dumps, or a compromised OS. Rust memory safety alone does not protect secrets from host malware.

## Signing boundary

The future signer must independently decode exact transaction bytes and derive the review details itself. It must check chain/network identity, recipients, amounts, fees, and relevant chain-specific fields before approval. It should reject unsupported transaction types and blind signing. Approval should bind to an exact digest of the validated transaction. The coordinator's text summary is untrusted. A host-controlled approval display is not a trusted display; malware can alter what the user sees or types.

## Vault and recovery review gate

Before vault implementation, review the threat model and select established, reviewed cryptographic libraries. Specify authenticated encryption, KDF parameters, randomness, versioned file format, backup encryption, restore verification, corruption handling, and key migration. A backup must be recoverable without the original laptop, while a local hardware-bound wrapper must not become the only recovery path. No custom cryptographic primitive should be introduced. Test on every supported platform and seek independent security review before handling real funds.
