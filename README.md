# StellarSwap — Stellar AMM

A constant-product AMM for [Stellar](https://stellar.org), built with [Soroban](https://soroban.stellar.org) smart contracts.

> **Status:** LP token (SEP-41), swap execution, core math, and pair initialization are implemented and tested (29 tests passing). Liquidity flows (`add_liquidity`, `remove_liquidity`), factory deployment, SDK, and testnet scripts are open for contributors.

---

## What is this?

StellarSwap is a decentralized exchange built natively on Stellar. A pair contract holds two SEP-41 tokens as reserves and prices trades using the constant-product formula. Liquidity providers deposit tokens, receive LP tokens representing their share, and earn fees from every swap.

The protocol follows the same core/periphery architecture used by constant-product AMMs — a minimal factory and pair in core, with routing and SDK tooling in periphery — implemented in Rust on Soroban.

---

## Why Stellar?

- **5-second finality.** Trades settle in seconds, not minutes.
- **Sub-cent fees.** A swap costs a fraction of a cent.
- **Soroban smart contracts.** Rust compiled to WASM. Significantly harder to write the class of bugs that have drained hundreds of millions from EVM contracts.
- **No front-running within a ledger.** Stellar's consensus (SCP) closes ledgers with a defined transaction set — no validator can reorder transactions for profit.
- **No native open-source AMM.** There is no constant-product DEX on Stellar today. StellarSwap is the foundation for one.

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

- **Core** — factory, pair, and LP token contracts. Hold user funds, enforce the invariant. Must stay minimal.
- **Periphery** — TypeScript SDK and scripts. No on-chain privileges. Handles routing, slippage, and transaction building.

```
Factory ──deploys──► Pair ──mint/burn──► LP Token

SDK (periphery) ──calls──► Factory / Pair via Soroban RPC
```

See [docs/architecture.md](docs/architecture.md) for the full system map and data flows.

---

## Repository Structure

```
contracts/
  factory/          # Pair registry — create_pair, get_pair
  pair/
    src/
      lib.rs        # Contract entry point
      pair.rs       # State, reserves, swap logic
      math.rs       # Pure AMM math (implemented + tested)
      token.rs      # SEP-41 cross-contract call helpers
    tests/
      math_test.rs        # 7 passing tests
      initialize_test.rs  # 5 passing tests
      swap_test.rs        # 5 passing tests
  lp-token/
    src/lib.rs      # Full SEP-41 LP token (implemented + tested)
    tests/
      lp_token_test.rs    # 10 passing tests

sdk/src/
  library.ts        # sortTokens, proportional quote helper
  pair.ts           # PairClient scaffold
  router.ts         # RouterClient scaffold

scripts/
  deploy.ts         # Testnet deploy scaffold
  seed-liquidity.ts # Liquidity seeding scaffold

docs/
  architecture.md       # System design, data flows, issue plan
  contracts.md          # Every function, storage layout, implementation steps
  contributing.md       # How to contribute
  plan.md               # Build plan, roadmap, maintainer vs contributor split
  implementation-log.md # Detailed record of what was built and how
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
| LP token contract (SEP-41) | ✅ Done + tested |
| Token cross-contract helpers | ✅ Done |
| `swap` execution | ✅ Done + tested |
| `add_liquidity` | 🔲 Stubbed — open issue #9 |
| `remove_liquidity` | 🔲 Stubbed — open issue #10 |
| Factory `create_pair` / `get_pair` | 🔲 Stubbed — open issues #13, #14 |
| TypeScript SDK | 🔲 Scaffolded — open issues #15–17 |
| Testnet deployment | 🔲 Scaffolded — open issues #19, #20 |

---

## Quick Start

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt

# Build
cargo build

# Test (29 tests, all passing)
cargo test
```

---

## Contributing

See [docs/contributing.md](docs/contributing.md) for setup and guidelines.

Work is broken into layers in [docs/plan.md](docs/plan.md). The first open layer is `add_liquidity` and `remove_liquidity` — the math and token helpers they depend on are already implemented.

All PRs target `develop`. `main` is stable only.

---

## Roadmap

- **StellarSwap v1** — constant-product AMM (this repo)
- **StellarSwap v2** — concentrated liquidity (future)
- **StellarSwap v3** — singleton pool manager + hooks (future)

---

## License

MIT
