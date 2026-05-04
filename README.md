# Dex Protocol — Stellar AMM

A constant-product AMM for [Stellar](https://stellar.org), built with [Soroban](https://soroban.stellar.org) smart contracts.

> **Status:** Core math, pair initialization, and invariant enforcement are implemented and tested. Swap execution, liquidity flows, LP token, SDK, and deployment are open for contributors.

---

## What is this?

A decentralized exchange protocol built natively on Stellar. A pair contract holds two SEP-41 tokens as reserves and prices trades using the constant-product formula. Liquidity providers deposit tokens, receive LP tokens representing their share, and earn fees from every swap.

The protocol follows the same core/periphery architecture used by constant-product AMMs — a minimal factory and pair in core, with routing and SDK tooling in periphery — implemented in Rust on Soroban.

---

## Why Stellar?

Stellar's properties are genuinely better for an AMM than most chains:

- **5-second finality.** Trades settle in seconds, not minutes. No waiting for block confirmations.
- **Sub-cent fees.** A swap costs a fraction of a cent. On Ethereum, gas fees can exceed the trade value for small amounts.
- **Soroban smart contracts.** Stellar's contract platform runs Rust compiled to WASM. Rust's type system and ownership model make it significantly harder to write the class of bugs that have drained hundreds of millions from EVM contracts.
- **No front-running within a ledger.** Stellar's consensus (SCP) closes ledgers with a defined transaction set — there's no miner/validator who can reorder transactions within a ledger for profit the way EVM miners can.
- **No native open-source AMM.** There is no constant-product DEX on Stellar today. This is the foundation for one.

---

## How pricing works

The pair holds two tokens — A and B. The invariant is:

```
reserve_A × reserve_B = k
```

When you swap token A in, the contract gives you token B out — only as much as keeps `k` constant. A 0.3% fee is taken on the way in and stays in the pair, growing `k` slightly with each trade. That fee growth is how liquidity providers earn yield.

**Example:** pair has 1,000 A and 1,000 B (k = 1,000,000). You send in 100 A. After the 0.3% fee: effective input = 99.7 A. New reserve_A = 1,099.7, so reserve_B must be 1,000,000 / 1,099.7 ≈ 909.3. You receive ~90.7 B.

---

## How liquidity works

Liquidity providers deposit both tokens into a pair and receive **LP tokens** — a receipt representing their percentage share of the reserves. When the pair earns fees, the reserves grow. When an LP burns their LP tokens to withdraw, they get back their share of the larger reserves. That's the yield.

First deposit mints `sqrt(amount_a × amount_b)` LP tokens. A small amount (`MINIMUM_LIQUIDITY = 1,000` base units) is permanently locked on the first deposit to prevent a share price manipulation attack.

---

## Architecture

The protocol is split into **core** and **periphery**:

- **Core** — factory and pair contracts. Hold user funds, enforce the invariant. Must stay minimal.
- **Periphery** — TypeScript SDK and scripts. No on-chain privileges. Handles routing, slippage, and transaction building.

```
Factory ──deploys──► Pair ──mint/burn──► LP Token

SDK (periphery) ──calls──► Factory / Pair via Soroban RPC
```

See [docs/architecture.md](docs/architecture.md) for the full system map, data flows, and implementation plan.

---

## Repository Structure

```
contracts/
  factory/          # Pair registry — create_pair, get_pair
  pair/
    src/
      lib.rs        # Contract entry point
      pair.rs       # State, reserves, liquidity and swap logic
      math.rs       # Pure AMM math (implemented + tested)
      token.rs      # SEP-41 cross-contract call helpers
    tests/
      math_test.rs        # 7 passing tests
      initialize_test.rs  # 5 passing tests
  lp-token/         # SEP-41 LP token contract

sdk/src/
  library.ts        # sortTokens, proportional quote helper
  pair.ts           # PairClient scaffold
  router.ts         # RouterClient scaffold

scripts/
  deploy.ts         # Testnet deploy scaffold
  seed-liquidity.ts # Liquidity seeding scaffold

docs/
  architecture.md   # System design, data flows, issue plan
  contracts.md      # Every function, storage layout, implementation steps
  contributing.md   # How to contribute
```

---

## What's implemented

| Component | Status |
|---|---|
| Constant-product math (`get_amount_out`) | ✅ Done + tested |
| LP token mint math (`calc_lp_tokens_to_mint`) | ✅ Done + tested |
| Post-swap invariant check (`check_invariant`) | ✅ Done + tested |
| Integer square root | ✅ Done + tested |
| Pair initialization | ✅ Done + tested |
| Reserve reads + quote simulation | ✅ Done + tested |
| Factory initialization | ✅ Done + tested |
| Token sorting in factory | ✅ Done |
| `add_liquidity` | 🔲 Stubbed — open issue |
| `remove_liquidity` | 🔲 Stubbed — open issue |
| `swap` execution | 🔲 Stubbed — open issue |
| LP token contract (SEP-41) | 🔲 Stubbed — open issue |
| Factory `create_pair` / `get_pair` | 🔲 Stubbed — open issue |
| TypeScript SDK | 🔲 Scaffolded — open issues |
| Testnet deployment | 🔲 Scaffolded — open issue |

---

## Quick Start

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt

# Build
cargo build

# Test (14 tests, all passing)
cargo test
```

---

## Contributing

See [docs/contributing.md](docs/contributing.md) for setup and guidelines.

Work is broken into layers in [docs/architecture.md](docs/architecture.md#issue-planning). The first open layer is the LP token contract — it has no dependencies and is a good starting point.

All PRs target `develop`. `main` is stable only.

---

## License

MIT
