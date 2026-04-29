# Contributing

Thanks for your interest in contributing! This project is an open-source DEX on Stellar and contributions of all kinds are welcome.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)
- Node.js 18+ (for SDK and scripts)

```bash
# Install Rust wasm target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Setup

```bash
git clone https://github.com/your-org/dex-protocol
cd dex-protocol

# Build contracts
cargo build

# Run contract tests
cargo test

# Install SDK dependencies
cd sdk && npm install
```

---

## Project Structure

```
contracts/amm/        # Core AMM contract (swap, liquidity)
contracts/lp-token/   # LP token contract (SEP-41)
sdk/                  # TypeScript SDK
scripts/              # Deploy and admin scripts
tests/integration/    # End-to-end tests
docs/                 # Documentation
```

---

## Good First Issues

Look for issues tagged `good first issue`. Some starting points:

- Implement `math::sqrt` (Babylonian integer square root)
- Implement `math::get_amount_out` (constant-product formula)
- Implement `math::calc_lp_tokens_to_mint`
- Write unit tests for the math module
- Implement `PoolClient.getReserves()` in the SDK

---

## Guidelines

- **One concern per PR.** Keep PRs small and focused.
- **Tests required.** Any new contract logic needs a test in `tests/`.
- **No unsafe code** in contracts.
- **Document public functions** with a short doc comment explaining what it does and what it panics on.
- Run `cargo fmt` and `cargo clippy` before submitting.

---

## Submitting a PR

1. Fork the repo and create a branch: `git checkout -b feat/your-feature`
2. Make your changes
3. Run `cargo test` — all tests must pass
4. Open a PR with a clear description of what you changed and why
