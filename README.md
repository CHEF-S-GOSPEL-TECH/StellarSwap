# DEX Protocol — Stellar AMM

A minimal constant-product AMM (like Uniswap v2) built on [Stellar](https://stellar.org) using [Soroban](https://soroban.stellar.org) smart contracts.

> **Status: Under active development. Core contracts are scaffolded — contributors welcome!**

---

## What is this?

This is a decentralized exchange (DEX) built on Stellar. It lets anyone swap between two tokens directly on-chain — no order books, no middlemen. Prices are determined automatically by a mathematical formula based on how much of each token the contract holds.

It's modeled after Uniswap v2, one of the most battle-tested DeFi protocols on Ethereum, but rebuilt from scratch for Stellar using Soroban smart contracts.

---

## How does pricing work?

The contract always holds two tokens — call them A and B. The rule is simple:

```
reserve_A × reserve_B = k
```

`k` never goes down. When you swap token A in, the contract gives you token B out — but only as much as keeps `k` constant. The more you try to buy, the worse your price gets. This is called a **constant-product curve**.

**Example:** pool has 100 A and 100 B (k = 10,000). You send in 10 A. The contract now has 110 A, so it can only hold ~90.9 B to keep k = 10,000. You get ~9.1 B out. A 0.3% fee is taken on the way in, which stays in the pool.

---

## Who provides the tokens?

**Liquidity providers (LPs)** deposit equal value of both tokens into the pool. In return they receive **LP tokens** — a receipt that represents their percentage share of the pool.

When the pool earns fees from swaps, those fees grow the reserves. When an LP burns their LP tokens to withdraw, they get back their share of the (now larger) reserves. That's how LPs earn yield.

---

## Project Structure

```
contracts/amm/        # Core AMM contract (swap, add/remove liquidity)
contracts/lp-token/   # LP token (SEP-41 fungible token)
sdk/                  # TypeScript SDK for frontends and scripts
scripts/              # Deploy and seed-liquidity scripts
tests/integration/    # End-to-end tests
docs/                 # Architecture and contract reference
```

---

## Quick Start

```bash
# Prerequisites: Rust, wasm32 target, Stellar CLI
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt

# Build
cargo build

# Test
cargo test
```

---

## Why Stellar?

Stellar is fast (5s finality), cheap (fractions of a cent per transaction), and has a growing DeFi ecosystem. Soroban — Stellar's smart contract platform — lets you write contracts in Rust, which compiles to WASM and runs on-chain. There's no native open-source DEX infrastructure like this yet, which is exactly why it's worth building.

---

## Documentation

- [Architecture](docs/architecture.md) — how the AMM works, formulas, token flows
- [Contract Reference](docs/contracts.md) — all public contract functions
- [Contributing](docs/contributing.md) — how to get started as a contributor

---

## Contributing

This project is open-source and actively looking for contributors. See [docs/contributing.md](docs/contributing.md) for how to get involved.

All PRs should target the `develop` branch. The `main` branch is stable/release only.

Good first issues are tagged in the issue tracker — many core functions are stubbed with `todo!()` and ready to be implemented:

- Implement the constant-product formula in `contracts/amm/src/math.rs`
- Implement pool swap and liquidity logic in `contracts/amm/src/pool.rs`
- Write integration tests in `tests/integration/`
- Build out the TypeScript SDK in `sdk/`

---

## License

MIT
