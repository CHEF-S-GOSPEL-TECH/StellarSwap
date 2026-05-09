# Contributing

StellarSwap is an open-source constant-product AMM on Stellar. Contributions are welcome. Read this document before opening a PR.

---

## Before You Start

Read these documents first:

- [Architecture](architecture.md) — how the system works, data flows, and the full issue plan ordered by dependency
- [Contract Reference](contracts.md) — every contract, every function, storage layout, and implementation steps
- [Build Plan](plan.md) — what's done, what's open, and the contributor issue map

Understanding the architecture will save you from building something that conflicts with how the rest of the system works.

---

## Setup

### Prerequisites

- [Rust](https://rustup.rs/) with the `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)
- Node.js 18+ (for SDK and scripts)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### Local setup

```bash
git clone https://github.com/CHEF-S-GOSPEL-TECH/StellarSwap.git
cd StellarSwap

# Build all contracts
cargo build

# Run all tests (29 passing)
cargo test

# SDK
cd sdk && npm install
```

All tests must pass before you start. If they don't, open an issue.

---

## Repository Structure

```
contracts/
  factory/        # Pair registry — create_pair, get_pair
  pair/           # Core AMM — swap, add_liquidity, remove_liquidity
    src/
      lib.rs      # Contract entry point (thin wrappers only)
      pair.rs     # State and orchestration logic
      math.rs     # Pure AMM math — no storage
      token.rs    # SEP-41 cross-contract call helpers
  lp-token/       # SEP-41 LP token — mint/burn controlled by pair
    src/lib.rs
    tests/lp_token_test.rs

sdk/              # TypeScript periphery — transaction builders, quotes
scripts/          # Testnet deploy and seed scripts
tests/integration/# Future end-to-end tests
docs/             # Architecture, contract reference, this file
```

---

## What to Work On

Work is planned in layers in [plan.md](plan.md). Each layer depends on the one before it. Pick an issue from the earliest incomplete layer.

### Completed layers

**Layer 1 — LP Token** ✅
Full SEP-41 token contract implemented and tested.

**Layer 2 — Pair token helpers** ✅
All five cross-contract call helpers implemented.

**Layer 3 (partial) — Swap** ✅
`swap` is implemented and tested.

### Open layers

**Layer 3 (remaining) — Liquidity flows** (start here)
- Issue #9: `pair/pair.rs` — `add_liquidity`
- Issue #10: `pair/pair.rs` — `remove_liquidity`
- Issue #12: pair integration tests (full lifecycle)

**Layer 4 — Factory**
- Issue #13: `factory` — `create_pair`
- Issue #14: `factory` — `get_pair`

**Layer 5 — SDK**
- Issue #15: `sdk/pair.ts` — `getReserves`, `getQuote`
- Issue #16: `sdk/pair.ts` — `addLiquidity`, `removeLiquidity` tx builders
- Issue #17: `sdk/router.ts` — `quote`, `buildSwapTx`

**Layer 6 — Integration and deployment**
- Issue #18: end-to-end integration test
- Issue #19: testnet deploy script
- Issue #20: testnet seed-liquidity script

---

## Guidelines

**One concern per PR.** One function, one module, one layer. Don't bundle unrelated changes.

**Tests are required.** Every new function needs at least one test. Tests live in the `tests/` directory of each contract package. Test the happy path and the panics. Look at existing tests for the pattern.

**No unsafe code** in contracts.

**Document public functions.** Every `pub fn` needs a doc comment that says what it does and what it panics on.

**Run before pushing:**
```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

All three must pass cleanly.

---

## Implementation Notes

Things that will trip you up if you don't know them:

**Token amounts are never trusted from the caller.** The pair derives amounts from balance deltas — read the live token balance, subtract the cached reserve, that's the amount that arrived. Never add an `amount` parameter to `add_liquidity`, `remove_liquidity`, or `swap`. See `contracts.md` for the exact steps.

**Auth is explicit in Soroban.** Use `address.require_auth()` at the top of any function that moves funds on behalf of an address. There is no `msg.sender`.

**LP token `burn` is admin-only.** The pair contract calls `burn`, not the LP holder. Auth comes from the pair (admin), not from the `from` address.

**Storage types matter.** Use `env.storage().instance()` for contract-level state (reserves, token addresses, admin, name, symbol, total supply). Use `env.storage().persistent()` for per-account state (balances, allowances) in the LP token.

**`soroban_sdk::token::Client`** is how you make cross-contract token calls. Use `token::Client::new(env, &token_address)` to get a client, then call `.transfer`, `.balance`, etc. For LP-specific calls (`mint`, `burn`, `total_supply`), use the inline `LpTokenClient` defined in `pair/src/token.rs`.

**Do not import the lp-token crate into the pair's non-dev dependencies.** This causes duplicate WASM symbol errors at link time. The `LpTokenClient` in `token.rs` is defined via `#[contractclient]` trait — use that instead.

---

## Submitting a PR

1. Branch off `develop`: `git checkout -b feat/your-feature`
2. Make your changes
3. Run `cargo fmt && cargo clippy -- -D warnings && cargo test` — all must pass
4. Open a PR targeting `develop` with a clear description of what you implemented and how you tested it

`main` is stable/release only. All PRs go to `develop`.
