# Contributing

This project is an open-source DEX on Stellar. Contributions are welcome. Read this document before opening a PR.

---

## Before You Start

Read these two documents first:

- [Architecture](architecture.md) — how the system works, data flows, and the full issue plan ordered by dependency
- [Contract Reference](contracts.md) — every contract, every function, storage layout, and implementation steps

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
git clone https://github.com/your-org/dex-protocol
cd dex-protocol

# Build all contracts
cargo build

# Run all tests
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

sdk/              # TypeScript periphery — transaction builders, quotes
scripts/          # Testnet deploy and seed scripts
tests/integration/# Future end-to-end tests
docs/             # Architecture, contract reference, this file
```

---

## What to Work On

Work is planned in layers in [architecture.md](architecture.md#issue-planning). Each layer depends on the one before it. Pick an issue from the earliest incomplete layer.

Current open layers:

**Layer 1 — LP Token** (start here, no dependencies)
- `lp-token`: `initialize`, SEP-41 reads, `transfer`, `transfer_from`, `approve`, `allowance`, `mint`, `burn`

**Layer 2 — Pair token helpers** (after LP token interface is defined)
- `pair/token.rs`: `transfer`, `mint`, `burn`, `total_supply`, `balance`

**Layer 3 — Pair core logic** (after Layer 2)
- `pair/pair.rs`: `add_liquidity`, `remove_liquidity`, `swap`

**Layer 4 — Factory** (after pair is deployable)
- `factory`: `create_pair`, `get_pair`

**Layer 5 — SDK** (after Layer 3)
- `sdk/pair.ts`, `sdk/router.ts`

**Layer 6 — Integration and deployment** (after all above)
- End-to-end tests, testnet scripts

---

## Guidelines

**One concern per PR.** One function, one module, one layer. Don't bundle unrelated changes.

**Tests are required.** Every new function needs at least one test. Tests live in the `tests/` directory of each contract package:
- `contracts/pair/tests/`
- `contracts/factory/tests/`
- `contracts/lp-token/tests/` *(create this when implementing the LP token)*

Test the happy path and the panics. Look at `contracts/pair/tests/math_test.rs` and `contracts/pair/tests/initialize_test.rs` for examples.

**No unsafe code** in contracts.

**Document public functions.** Every `pub fn` needs a doc comment that says what it does and what it panics on.

**Run before pushing:**
```bash
cargo fmt
cargo clippy
cargo test
```

All three must pass cleanly.

---

## Implementation Notes

A few things that will trip you up if you don't know them:

**Token amounts are never trusted from the caller.** The pair derives amounts from balance deltas — read the live token balance, subtract the cached reserve, that's the amount that arrived. Never add an `amount` parameter to `add_liquidity`, `remove_liquidity`, or `swap`. See `contracts.md` for the exact steps.

**Auth is explicit in Soroban.** Use `address.require_auth()` at the top of any function that moves funds on behalf of an address. There is no `msg.sender`.

**LP token `burn` is admin-only.** The pair contract calls `burn`, not the LP holder. Auth comes from the pair (admin), not from the `from` address.

**Storage types matter.** Use `env.storage().instance()` for contract-level state (reserves, token addresses, admin, name, symbol, total supply). Use `env.storage().persistent()` for per-account state (balances, allowances) in the LP token — these entries need to survive ledger expiry.

**`soroban_sdk::token::Client`** is how you make cross-contract token calls. Use `token::Client::new(env, &token_address)` to get a client, then call `.transfer`, `.balance`, `.total_supply`, etc.

---

## Submitting a PR

1. Branch off `develop`: `git checkout -b feat/your-feature`
2. Make your changes
3. Run `cargo fmt && cargo clippy && cargo test` — all must pass
4. Open a PR targeting `develop` with a clear description of what you implemented and how you tested it

`main` is stable/release only. All PRs go to `develop`.
