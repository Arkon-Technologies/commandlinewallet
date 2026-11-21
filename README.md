# CommandLineWallet (CLW)

CLW is an open-source, terminal-first, self-custodial wallet project targeting Bitcoin, Ethereum/EVM, and Solana on macOS, Windows, and Linux. The executable is named `clw`.

**Phase 1 is an architecture preview. It cannot create or recover a wallet, hold keys, show balances, send transactions, or sign anything. Do not use it to protect funds.**

```sh
cargo run -p clw-tui --
cargo run -p clw-tui -- --help
cargo run -p clw-tui -- --version
cargo run -p clw-tui -- security
```

After building with `cargo build -p clw-tui`, run `target/debug/clw`. The default command opens a small terminal screen; in non-interactive output it prints the same information and exits. Press `q` or `Esc` in the screen.

## Workspace

- `apps/clw-tui`: CLI and terminal screen.
- `crates/clw-core`: shared non-secret types.
- `crates/clw-vault`: future encrypted vault lifecycle boundary.
- `crates/clw-signer`: future isolated transaction signing boundary.
- `crates/clw-security`: capability and review concepts.
- `crates/clw-bitcoin`, `clw-evm`, `clw-solana`: future chain adapter boundaries.
- `platform/`: platform integration notes; no native bindings yet.
- `docs/`: architecture, threat model, security model, and roadmap.

The proposed layout is used. Crates contain only the smallest useful contracts, and platform directories contain notes instead of code because no hardware integration exists yet.

## Development

Rust 1.96 or newer is required. Run:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The initial dependency set is Clap, Crossterm, and Ratatui for the CLI and screen. No cryptographic or RPC dependencies are included in Phase 1. `Cargo.lock` is included locally for reproducible application builds.

See [SECURITY.md](SECURITY.md) before contributing to security-sensitive code. CLW does not claim security equivalent to a dedicated hardware wallet.
