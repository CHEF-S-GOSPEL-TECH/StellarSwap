# StellarSwap — Documentation

A constant-product AMM (decentralized exchange) built natively on Stellar using Soroban smart contracts.

---

## What is StellarSwap?

StellarSwap lets anyone swap between two tokens, or deposit tokens to earn fees as a liquidity provider — all on-chain, with no order book and no intermediary.

It uses the **constant-product formula**: `reserve_A × reserve_B = k`. Every trade must keep this product constant (or grow it slightly via fees). This single rule prices every trade.

**Why Stellar?**
- Trades settle in ~5 seconds
- Transaction fees are fractions of a cent
- Soroban contracts are Rust compiled to WASM — a safer execution environment than the EVM
- No validator can reorder transactions within a ledger (no front-running)
- There is no open-source constant-product DEX on Stellar today — StellarSwap is the foundation for one

---

## How It Works

### Swapping

A pair holds two tokens — A and B. When you send token A in, the contract calculates how much token B to give you such that `k` stays constant. A **0.3% fee** is taken on the input and stays in the pool — this is how liquidity providers earn yield.

**Example:** Pool has 1,000 A and 1,000 B. You send in 100 A. After the 0.3% fee, effective input = 99.7 A. You receive ~90.7 B.

### Providing Liquidity

Liquidity providers deposit both tokens and receive **LP tokens** — a receipt representing their share of the pool. As fees accumulate, the reserves grow. When an LP burns their LP tokens to withdraw, they get back their proportional share of the larger reserves. That difference is the yield.

---

## Architecture

The protocol is split into **core** (on-chain contracts) and **periphery** (TypeScript SDK).

```
Factory ──deploys──► Pair ──mint/burn──► LP Token

SDK (periphery) ──calls──► Factory / Pair via Soroban RPC
```

**Core** holds user funds and enforces the invariant. It must stay minimal — bugs here are catastrophic.

**Periphery** has no on-chain privileges. It handles routing, slippage, and transaction building for frontends and scripts.

---

## Contracts

### Factory (`contracts/factory/`)

The pair registry. Ensures exactly one pair exists per token combination. Tokens are always sorted by address so `(A, B)` and `(B, A)` always resolve to the same pair.

| Function | Status |
|---|---|
| `initialize(admin)` | ✅ Done |
| `admin()` | ✅ Done |
| `create_pair(token_a, token_b)` | 🔲 Open — issue #13 |
| `get_pair(token_a, token_b)` | 🔲 Open — issue #14 |

### Pair (`contracts/pair/`)

The core contract. One deployed instance per token pair. Holds reserves, prices trades, and manages liquidity. The pair never trusts amounts from the caller — it always derives them from balance deltas.

| Function | Status |
|---|---|
| `initialize(token_a, token_b, lp_token)` | ✅ Done |
| `get_reserves()` | ✅ Done |
| `get_quote(token_in, amount_in)` | ✅ Done |
| `swap(to, token_in, min_amount_out)` | ✅ Done |
| `add_liquidity(to)` | 🔲 Open — issue #9 |
| `remove_liquidity(to)` | 🔲 Open — issue #10 |

The pair's math lives in `math.rs` (pure functions, fully tested):
- `get_amount_out` — swap output with 0.3% fee
- `calc_lp_tokens_to_mint` — LP tokens to mint on deposit
- `check_invariant` — post-swap safety check that `k` didn't decrease
- `sqrt` — integer square root (Babylonian method)

### LP Token (`contracts/lp-token/`)

A standard SEP-41 fungible token representing a liquidity provider's share of a pair's reserves. Fully implemented and tested. Only the pair contract (set as admin at initialization) can mint and burn.

---

## TypeScript SDK (`sdk/`)

Talks to deployed contracts via Soroban RPC. No on-chain privileges.

| File | Status | Description |
|---|---|---|
| `library.ts` | ✅ Done | `sortTokens`, proportional `quote` helper |
| `pair.ts` | 🔲 Open — issues #15, #16 | `PairClient` — reads reserves/quotes, builds liquidity transactions |
| `router.ts` | 🔲 Open — issue #17 | `RouterClient` — builds swap transactions with slippage protection |

---

## What's Built vs. What's Open

| Component | Status |
|---|---|
| Constant-product math | ✅ Done + tested |
| Post-swap invariant check | ✅ Done + tested |
| LP token mint math | ✅ Done + tested |
| Pair initialization + reserve reads | ✅ Done + tested |
| `swap` execution | ✅ Done + tested |
| LP token contract (SEP-41) | ✅ Done + tested |
| `add_liquidity` | 🔲 Open — issue #9 |
| `remove_liquidity` | 🔲 Open — issue #10 |
| Factory `create_pair` / `get_pair` | 🔲 Open — issues #13, #14 |
| TypeScript SDK | 🔲 Open — issues #15–17 |
| Testnet deployment | 🔲 Open — issues #19, #20 |

29 tests passing. The math and token helpers that `add_liquidity` and `remove_liquidity` depend on are already implemented — those are the natural next issues to pick up.

---

## Getting Started

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt

# Build
cargo build

# Test
cargo test
```

See [docs/contributing.md](contributing.md) for contribution guidelines and [docs/architecture.md](architecture.md) for the full system design.
