# Architecture

## What We're Building

A constant-product AMM on Stellar. Two SEP-41 tokens sit in a pair contract. The pair prices trades using the invariant `reserve_a * reserve_b = k`. Liquidity providers deposit both tokens and receive LP tokens representing their share. Fees from swaps grow the reserves, which means LP token holders earn yield when they withdraw.

The protocol is split into **core** and **periphery**:

- **Core** — the factory and pair contracts. They hold user funds and enforce the invariant. Must stay minimal. Bugs here are catastrophic.
- **Periphery** — the TypeScript SDK and scripts. No special on-chain privileges. Handles user-facing concerns like routing, slippage calculation, and transaction building.

---

## System Map

```
┌──────────────────────────────────────────────────────────────┐
│                         PERIPHERY                            │
│                                                              │
│   sdk/src/                                                   │
│   ┌─────────────┐   ┌─────────────┐   ┌──────────────────┐  │
│   │ RouterClient│   │  PairClient │   │   library.ts     │  │
│   │ router.ts   │   │  pair.ts    │   │   sortTokens     │  │
│   │             │   │             │   │   quote (no fee) │  │
│   └──────┬──────┘   └──────┬──────┘   └──────────────────┘  │
│          │                 │                                  │
└──────────┼─────────────────┼──────────────────────────────────┘
           │   Soroban RPC   │
┌──────────┼─────────────────┼──────────────────────────────────┐
│          ▼      CORE       ▼                                  │
│   ┌─────────────┐   ┌───────────────────────────────────┐    │
│   │   Factory   │   │          Pair Contract             │    │
│   │             │   │                                    │    │
│   │ create_pair │   │  reserve_a * reserve_b = k         │    │
│   │ get_pair    │   │                                    │    │
│   │             │   │  add_liquidity  remove_liquidity   │    │
│   └─────────────┘   │  swap  get_reserves  get_quote     │    │
│                     └─────────────┬───────────────────────┘   │
│                                   │ mint / burn               │
│                     ┌─────────────▼───────────────────────┐   │
│                     │        LP Token Contract             │   │
│                     │        (SEP-41 token)                │   │
│                     │  LP share of pair reserves           │   │
│                     └─────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

---

## Contracts

### Factory (`contracts/factory/`)

Registers all pairs. Ensures exactly one pair exists per token combination.

**Implemented:**
- `initialize(admin)` — one-time setup, stores the factory admin
- `admin()` — returns the admin address

**To implement:**
- `create_pair(token_a, token_b) → Address`
  - Sort token addresses so `(A,B)` and `(B,A)` always produce the same pair
  - Reject if a pair for this combination already exists
  - Deploy a new pair contract using `env.deployer().with_address(deployer, salt)` where salt is derived from the sorted token addresses — this gives deterministic, off-chain-computable pair addresses
  - Store the pair address in a registry keyed by `(token0, token1)`
  - Return the new pair address
- `get_pair(token_a, token_b) → Option<Address>`
  - Sort token addresses before lookup
  - Return the registered pair address, or `None` if it doesn't exist

---

### Pair (`contracts/pair/`)

The invariant-critical contract. One deployed instance per token pair.

#### Files

| File | Responsibility |
|---|---|
| `lib.rs` | Public contract entry point — thin wrappers delegating to `pair.rs` |
| `pair.rs` | State, reserve tracking, orchestration of math and token calls |
| `math.rs` | Pure math — no storage access |
| `token.rs` | SEP-41 cross-contract call helpers |

#### How tokens move in and out

The pair never calls `transferFrom`. Instead:

1. The caller transfers tokens **to the pair contract address** before calling any state-changing function.
2. The pair reads its actual token balances and diffs them against stored reserves to determine how much arrived.
3. Amounts are never trusted from the caller — always derived from the balance delta.

This means the pair is agnostic to how tokens were transferred. It works with direct transfers, router-mediated transfers, or any future mechanism.

#### Invariant enforcement

After every swap, the pair reads its new balances and asserts:

```
(1000 · balance_a − 3 · amount_in_a) · (1000 · balance_b − 3 · amount_in_b)
    >= 1_000_000 · reserve_a · reserve_b
```

This is the fee-adjusted constant-product check. It is implemented in `math::check_invariant()` and must be called at the end of every swap before reserves are updated. This is the core safety guarantee — it ensures `k` cannot decrease even if something unexpected happens.

#### Implemented:
- `initialize(token_a, token_b, lp_token)` — one-time setup; rejects identical tokens; sets reserves to 0
- `get_reserves() → (i128, i128)` — returns current reserves
- `get_quote(token_in, amount_in) → Option<i128>` — simulates a swap with fee; returns `None` if no liquidity

#### To implement:
- `add_liquidity(to) → lp_minted`
- `remove_liquidity(to) → (amount_a, amount_b)`
- `swap(to, token_in, min_amount_out) → amount_out`

#### Math (`math.rs`) — all done

| Function | Purpose |
|---|---|
| `get_amount_out(amount_in, reserve_in, reserve_out)` | Swap output with 0.3% fee |
| `calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply)` | LP tokens to mint on deposit |
| `check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)` | Post-swap safety check |
| `sqrt(n)` | Integer square root (Babylonian) |
| `MINIMUM_LIQUIDITY = 1_000` | Permanently locked on first deposit |

**Pricing formula:**
```
amount_in_with_fee = amount_in * 997
amount_out = (amount_in_with_fee * reserve_out) / (reserve_in * 1000 + amount_in_with_fee)
```

**First deposit LP tokens:**
```
lp_minted = sqrt(amount_a * amount_b) - MINIMUM_LIQUIDITY
```
`MINIMUM_LIQUIDITY` is burned to the zero address and permanently locked. This prevents a share price manipulation attack where an attacker inflates the value of a single LP token by donating to the pool before anyone else deposits.

**Subsequent deposit LP tokens:**
```
lp_minted = min(amount_a / reserve_a, amount_b / reserve_b) * lp_supply
```

---

### LP Token (`contracts/lp-token/`)

A SEP-41 fungible token. Represents a liquidity provider's proportional share of the pair's reserves.

**Rules:**
- Only the pair contract (set as `admin` at initialization) can call `mint` and `burn`
- `burn` is called by the pair on LP tokens the pair itself holds — not by the LP holder directly
- Any holder can `transfer` freely
- `decimals()` is always `7` (Stellar standard)

**To implement:**
- `initialize(admin, name, symbol)` — store admin, name, symbol; total supply = 0
- SEP-41 reads: `name`, `symbol`, `decimals`, `total_supply`, `balance`
- `transfer(from, to, amount)` — requires auth from `from`
- `transfer_from(spender, from, to, amount)` — requires auth from `spender`, checks allowance
- `approve(from, spender, amount, expiration_ledger)` — requires auth from `from`
- `allowance(from, spender) → i128`
- `mint(to, amount)` — requires auth from admin (pair contract)
- `burn(from, amount)` — requires auth from admin (pair contract)

---

## Periphery SDK (`sdk/`)

TypeScript. Talks to deployed contracts via Soroban RPC. No on-chain privileges.

| File | What it does | Status |
|---|---|---|
| `library.ts` | `sortTokens` — canonical token ordering. `quote` — proportional amount for liquidity sizing (no fee). | ✅ Done |
| `pair.ts` | `PairClient` — reads reserves and quotes, builds add/remove liquidity transactions | 🔲 To implement |
| `router.ts` | `RouterClient` — builds swap transactions with slippage protection | 🔲 To implement |

`library.ts::quote` is for calculating how much of token B to deposit alongside a given amount of token A. It does not apply the trading fee. `PairClient::getQuote` calls the on-chain `get_quote` which does apply the fee. Do not confuse them.

---

## Data Flows

### Add Liquidity

```
1. Caller transfers token_a and token_b → pair contract address
2. Caller calls pair.add_liquidity(to)
3. Pair reads balance_a = token_a.balance(this), balance_b = token_b.balance(this)
4. amount_a = balance_a - reserve_a
5. amount_b = balance_b - reserve_b
6. lp_supply = lp_token.total_supply()
7. lp_minted = calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply)
8. If lp_supply == 0: burn MINIMUM_LIQUIDITY LP tokens to zero address
9. lp_token.mint(to, lp_minted)
10. reserve_a = balance_a, reserve_b = balance_b
11. Return lp_minted
```

### Remove Liquidity

```
1. Caller transfers LP tokens → pair contract address
2. Caller calls pair.remove_liquidity(to)
3. lp_amount = lp_token.balance(this)
4. lp_supply = lp_token.total_supply()
5. amount_a = lp_amount * reserve_a / lp_supply
6. amount_b = lp_amount * reserve_b / lp_supply
7. lp_token.burn(this, lp_amount)
8. token_a.transfer(this, to, amount_a)
9. token_b.transfer(this, to, amount_b)
10. reserve_a -= amount_a, reserve_b -= amount_b
11. Return (amount_a, amount_b)
```

### Swap

```
1. Caller transfers token_in → pair contract address
2. Caller calls pair.swap(to, token_in, min_amount_out)
3. Identify (reserve_in, reserve_out, token_out) from token_in
4. amount_in = token_in.balance(this) - reserve_in
5. amount_out = get_amount_out(amount_in, reserve_in, reserve_out)
6. Require amount_out >= min_amount_out  (slippage guard)
7. token_out.transfer(this, to, amount_out)
8. Read new balances: balance_a, balance_b
9. check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)
10. reserve_a = balance_a, reserve_b = balance_b
11. Return amount_out
```

---

## Stellar Platform Notes

These are Soroban-specific implementation details every contributor needs to know.

**Tokens** — use SEP-41. The `soroban_sdk::token::Client` provides the interface for cross-contract token calls. Stellar's native XLM must be wrapped as a SEP-41 token before it can be used in a pair. Wrapping is handled by the caller or router — never by the pair.

**Authorization** — use `env.require_auth(&address)`. There is no `msg.sender`. Auth is explicit: the caller signs a transaction that authorizes specific contract invocations. The pair calls `to.require_auth()` at the start of any function that moves funds on behalf of `to`.

**Deterministic contract addresses** — use `env.deployer().with_address(deployer, salt)` in the factory. The salt should be a hash of the sorted token addresses. This makes pair addresses computable off-chain without querying the chain.

**No reentrancy risk** — Soroban's WASM execution model does not allow reentrant calls. No lock is needed.

**Storage** — use `env.storage().instance()` for contract-level state (reserves, token addresses, admin). Use `env.storage().persistent()` for per-account state (balances, allowances) in the LP token contract.

**Decimals** — Stellar tokens use 7 decimal places. All amounts in the contracts are in base units (stroops equivalent). `MINIMUM_LIQUIDITY = 1_000` base units is the correct value.

---

## Issue Planning

Ordered by dependency. Each item is one GitHub issue.

### Layer 1 — LP Token (no dependencies)

- [ ] `lp-token`: implement `initialize`
- [ ] `lp-token`: implement SEP-41 reads — `name`, `symbol`, `total_supply`, `balance`
- [ ] `lp-token`: implement `transfer` and `transfer_from`
- [ ] `lp-token`: implement `approve` and `allowance`
- [ ] `lp-token`: implement `mint` (admin-only)
- [ ] `lp-token`: implement `burn` (admin-only)
- [ ] `lp-token`: tests for all functions

### Layer 2 — Pair token helpers (depends on LP token interface)

- [ ] `pair/token.rs`: implement `transfer`
- [ ] `pair/token.rs`: implement `mint`
- [ ] `pair/token.rs`: implement `burn`
- [ ] `pair/token.rs`: implement `total_supply`
- [ ] `pair/token.rs`: implement `balance`

### Layer 3 — Pair core logic (depends on Layer 2)

- [ ] `pair/pair.rs`: implement `add_liquidity`
- [ ] `pair/pair.rs`: implement `remove_liquidity`
- [ ] `pair/pair.rs`: implement `swap`
- [ ] `pair`: tests for `add_liquidity`, `remove_liquidity`, `swap` using mock tokens

### Layer 4 — Factory (depends on pair being deployable)

- [ ] `factory`: implement `create_pair` — sort tokens, deterministic deploy, registry storage, reject duplicates
- [ ] `factory`: implement `get_pair` — sort tokens, registry lookup
- [ ] `factory`: tests including duplicate rejection and order-independence

### Layer 5 — SDK (depends on Layer 3)

- [ ] `sdk/pair.ts`: implement `getReserves`
- [ ] `sdk/pair.ts`: implement `getQuote`
- [ ] `sdk/pair.ts`: implement `addLiquidity` transaction builder
- [ ] `sdk/pair.ts`: implement `removeLiquidity` transaction builder
- [ ] `sdk/router.ts`: implement `quote`
- [ ] `sdk/router.ts`: implement `buildSwapTx` with slippage

### Layer 6 — Integration and deployment (depends on all above)

- [ ] `tests/integration`: end-to-end — deploy factory, create pair, add liquidity, swap, remove liquidity
- [ ] `scripts/deploy.ts`: deploy factory and seed pair to testnet
- [ ] `scripts/seed-liquidity.ts`: seed initial liquidity on testnet

### Future scope (not in current roadmap)

- `sync()` recovery function on the pair — force-syncs reserves to live balances
- Protocol fee (`fee_to`) on the factory — 1/6 cut of LP fees, collected as LP tokens on liquidity events
- TWAP price oracle on the pair — requires design for Soroban's ledger timing model
- Flash swaps — requires callback design for Soroban cross-contract calls
