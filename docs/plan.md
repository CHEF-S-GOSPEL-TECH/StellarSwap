# StellarSwap — Build Plan

A constant-product AMM on Stellar, built with Soroban smart contracts.
This is StellarSwap v1 — the foundational DEX layer for the Stellar ecosystem.

---

## What StellarSwap Is

StellarSwap follows the same architecture as Uniswap V2 — a constant-product AMM with a factory/pair/LP-token model — implemented natively on Stellar using Soroban (Rust → WASM). It is not a fork or reference to Uniswap. It is an independent implementation of the constant-product formula designed for Stellar's execution model, auth system, and token standard (SEP-41).

**StellarSwap v1** = constant-product AMM (this repo)
**StellarSwap v2** = concentrated liquidity (future — not in scope)
**StellarSwap v3** = singleton pool manager + hooks (future — not in scope)

---

## Current State (as of May 2026)

### Done — 29 tests passing

| Component | File | Tests | Status |
|---|---|---|---|
| Swap math (`get_amount_out`) | `pair/src/math.rs` | 7 | ✅ |
| LP mint math (`calc_lp_tokens_to_mint`) | `pair/src/math.rs` | — | ✅ |
| Invariant check (`check_invariant`) | `pair/src/math.rs` | — | ✅ |
| Integer sqrt + `MINIMUM_LIQUIDITY` | `pair/src/math.rs` | — | ✅ |
| Pair initialization | `pair/src/pair.rs` | 5 | ✅ |
| Reserve reads + quote simulation | `pair/src/pair.rs` | — | ✅ |
| Factory initialization + admin | `factory/src/lib.rs` | 2 | ✅ |
| Token sorting in factory | `factory/src/lib.rs` | — | ✅ |
| LP token contract (SEP-41) | `lp-token/src/lib.rs` | 10 | ✅ |
| Token cross-contract helpers | `pair/src/token.rs` | — | ✅ |
| `swap` execution | `pair/src/pair.rs` | 5 | ✅ |

### Open for contributors

| Component | File | Issue | Status |
|---|---|---|---|
| `add_liquidity` | `pair/src/pair.rs` | #9 | 🔲 |
| `remove_liquidity` | `pair/src/pair.rs` | #10 | 🔲 |
| Pair integration tests (full lifecycle) | `pair/tests/` | #12 | 🔲 |
| Factory `create_pair` | `factory/src/lib.rs` | #13 | 🔲 |
| Factory `get_pair` | `factory/src/lib.rs` | #14 | 🔲 |
| SDK `pair.ts` reads | `sdk/src/pair.ts` | #15 | 🔲 |
| SDK `pair.ts` tx builders | `sdk/src/pair.ts` | #16 | 🔲 |
| SDK `router.ts` | `sdk/src/router.ts` | #17 | 🔲 |
| End-to-end integration test | `tests/integration/` | #18 | 🔲 |
| Testnet deploy script | `scripts/deploy.ts` | #19 | 🔲 |
| Testnet seed-liquidity script | `scripts/seed-liquidity.ts` | #20 | 🔲 |

---

## What We Implement (Maintainer Scope)

The maintainer implements enough to prove the project is real and technically
credible. The rest is left as well-scoped contributor issues.

### Layer 1 — LP Token contract
The SEP-41 fungible token that represents an LP's share of a pair.
No dependencies — self-contained. Proves we can ship a complete Soroban contract.

- `initialize(admin, name, symbol)`
- SEP-41 reads: `name`, `symbol`, `decimals`, `total_supply`, `balance`, `allowance`
- `transfer`, `transfer_from`
- `approve`
- `mint` (admin-only — called by pair)
- `burn` (admin-only — called by pair)
- Full test suite

### Layer 2 — token.rs helpers
Five thin cross-contract call wrappers (~20 lines total). Unblocks all pair logic.

- `transfer`, `mint`, `burn`, `total_supply`, `balance`

### Layer 3 — swap (partial pair logic)
The most visible function. A working swap + passing tests is the strongest
signal that this is a real DEX.

- `swap(to, token_in, min_amount_out)` — balance delta → fee → slippage check → invariant check → update reserves
- Tests: A→B, B→A, slippage rejection, invariant violation rejection

---

## What Contributors Implement (Wave Issues)

The following are left as well-scoped GitHub issues for wave contributors.

### Rust (Soroban)

| Issue | Description | Difficulty |
|---|---|---|
| #9 | `pair::add_liquidity` | Medium |
| #10 | `pair::remove_liquidity` | Medium |
| #12 | Pair integration tests (full lifecycle) | Medium |
| #13 | `factory::create_pair` (deterministic deploy + registry) | Hard |
| #14 | `factory::get_pair` | Easy |
| #18 | End-to-end integration test | Hard |

### TypeScript (SDK)

| Issue | Description | Difficulty |
|---|---|---|
| #15 | `sdk/pair.ts` — `getReserves`, `getQuote` | Easy |
| #16 | `sdk/pair.ts` — `addLiquidity`, `removeLiquidity` tx builders | Medium |
| #17 | `sdk/router.ts` — `quote`, `buildSwapTx` with slippage | Medium |
| #19 | Testnet deployment script | Medium |
| #20 | Testnet liquidity seeding script | Easy |

That's 10 well-scoped issues across Rust and TypeScript, covering easy through
hard difficulty. Enough to fill a wave without padding.

---

## Full Issue Map (all 20 issues)

```
Layer 1 — LP Token (no deps)
  #1  initialize
  #2  SEP-41 reads
  #3  transfer + transfer_from
  #4  approve + allowance
  #5  mint
  #6  burn
  #7  LP token tests

Layer 2 — token.rs helpers (depends on Layer 1 interface)
  #8  transfer, mint, burn, total_supply, balance

Layer 3 — Pair logic (depends on Layer 2)
  #9  add_liquidity          ← contributor
  #10 remove_liquidity       ← contributor
  #11 swap                   ← maintainer
  #12 pair integration tests ← contributor

Layer 4 — Factory (depends on Layer 3)
  #13 create_pair            ← contributor
  #14 get_pair               ← contributor

Layer 5 — SDK (depends on Layer 3)
  #15 pair.ts reads          ← contributor
  #16 pair.ts tx builders    ← contributor
  #17 router.ts              ← contributor

Layer 6 — Integration + Deployment (depends on all above)
  #18 end-to-end test        ← contributor
  #19 deploy script          ← contributor
  #20 seed-liquidity script  ← contributor
```

---

## Roadmap

### v1 — Constant-Product AMM (this repo)
- [x] Core math
- [x] Pair initialization + read functions
- [x] Factory initialization
- [ ] LP token contract (SEP-41)
- [ ] token.rs cross-contract helpers
- [ ] swap execution
- [ ] add_liquidity / remove_liquidity
- [ ] Factory create_pair / get_pair
- [ ] TypeScript SDK
- [ ] Testnet deployment

### v2 — Concentrated Liquidity (future)
- Range-based liquidity positions
- NFT position tokens
- Tick-based pricing
- Separate repo, builds on v1 as reference

### v3 — Singleton Pool Manager + Hooks (future)
- Single contract holds all pools
- Hook interface for custom pool logic
- Flash accounting
- Separate repo, builds on v2 as reference

---

## Architecture Summary

```
Factory ──deploys──► Pair ──mint/burn──► LP Token

SDK (periphery) ──calls──► Factory / Pair via Soroban RPC
```

- **Core** — factory, pair, LP token. Hold user funds. Enforce the invariant. Must stay minimal.
- **Periphery** — TypeScript SDK and scripts. No on-chain privileges. Handles routing, slippage, tx building.

See `docs/architecture.md` for the full system map and data flows.
See `docs/contracts.md` for every function's exact specification.
See `docs/issues.md` for the full issue list with acceptance criteria.

---

## Key Design Decisions

**Why V2 architecture and not V3/V4?**
V2 is the right foundation for a chain with no existing AMM. Concentrated
liquidity (V3) requires a working constant-product base to exist first.
StellarSwap v1 is that base.

**Why Soroban/Rust and not a Solidity port?**
Soroban's WASM execution model, explicit auth system, and Rust's type safety
make it significantly harder to write the class of bugs that have drained
hundreds of millions from EVM contracts. This is a native Stellar implementation,
not a port.

**Why the balance-delta model for token amounts?**
The pair never trusts a caller-supplied amount. It reads its actual token
balance and diffs against stored reserves. This means the pair is agnostic
to how tokens arrived — direct transfer, router, or any future mechanism.

**Why MINIMUM_LIQUIDITY?**
1,000 base units are permanently locked on the first deposit. This prevents
a share price manipulation attack where an attacker inflates the LP token
price by donating to an empty pool before anyone else deposits.
