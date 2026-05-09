# Implementation Log

A running record of everything implemented, how it works, and what decisions were made.
Update this file every time a layer is completed.

---

## Session 1 — May 9, 2026

### What was already done before this session (14 tests)

These were implemented before contributor issues were opened:

**`contracts/pair/src/math.rs`** — pure AMM math, no storage
- `get_amount_out(amount_in, reserve_in, reserve_out)` — constant-product swap output with 0.3% fee
  - Formula: `(amount_in * 997 * reserve_out) / (reserve_in * 1000 + amount_in * 997)`
- `calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply)` — LP tokens for a deposit
  - First deposit: `sqrt(amount_a * amount_b) - MINIMUM_LIQUIDITY`
  - Subsequent: `min(amount_a * lp_supply / reserve_a, amount_b * lp_supply / reserve_b)`
- `check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)` — post-swap safety check
  - Asserts: `(1000·balance_a − 3·amount_in_a) · (1000·balance_b − 3·amount_in_b) >= 1_000_000 · reserve_a · reserve_b`
- `sqrt(n)` — Babylonian integer square root
- `MINIMUM_LIQUIDITY = 1_000` — permanently locked on first deposit, prevents share price manipulation

**`contracts/pair/src/pair.rs`** — pair state and read functions
- `initialize(token_a, token_b, lp_token)` — one-time setup, stores addresses, sets reserves to 0
- `get_reserves()` — returns `(reserve_a, reserve_b)` from instance storage
- `get_quote(token_in, amount_in)` — simulates swap with fee, returns `None` if no liquidity

**`contracts/factory/src/lib.rs`** — factory registry
- `initialize(admin)` — stores factory admin, rejects double-init
- `admin()` — returns stored admin address
- Token sorting in `create_pair` / `get_pair` stubs — `token0 = min(a,b)`, `token1 = max(a,b)`

Tests: `math_test.rs` (7), `initialize_test.rs` pair (5), `initialize_test.rs` factory (2)

---

### Layer 1 — LP Token contract (10 new tests)

**File:** `contracts/lp-token/src/lib.rs`

**What it is:** A complete SEP-41 fungible token. Represents a liquidity provider's share of a pair's reserves. One instance is deployed per pair. The pair contract is set as admin at initialization — only the admin can mint or burn.

**Storage layout:**

| Key | Storage type | Description |
|---|---|---|
| `Admin` | `instance` | Pair contract address — only one that can mint/burn |
| `Name` | `instance` | Token name |
| `Symbol` | `instance` | Token symbol |
| `TotalSupply` | `instance` | Total LP tokens in circulation |
| `Balance(Address)` | `persistent` | Per-account LP token balance |
| `Allowance(Address, Address)` | `persistent` | Per-(from, spender) approved spend amount |

`instance` storage for contract-level state. `persistent` for per-account state — survives ledger entry expiry, which matters for user balances on Stellar.

**Functions:**

- `initialize(admin, name, symbol)` — stores metadata, sets total_supply = 0, panics `"lp token already initialized"` on second call
- `name / symbol / decimals / total_supply / balance / allowance` — pure reads from storage; `decimals` always returns 7 (Stellar standard)
- `approve(from, spender, amount, expiration_ledger)` — requires auth from `from`, stores allowance in persistent storage
- `transfer(from, to, amount)` — requires auth from `from`, calls internal `move_balance`
- `transfer_from(spender, from, to, amount)` — requires auth from `spender`, deducts allowance, calls `move_balance`
- `mint(to, amount)` — requires auth from **admin** (pair contract), increases `Balance(to)` and `TotalSupply`
- `burn(from, amount)` — requires auth from **admin**, decreases `Balance(from)` and `TotalSupply`
- `move_balance` (private) — shared debit/credit logic used by both transfer functions

**Key design decision:** `burn` auth comes from the admin (pair contract), not from the token holder. The pair burns LP tokens it holds itself during `remove_liquidity` — so auth must come from the pair, not the LP holder.

**Cargo changes:**
- Added `crate-type = ["cdylib", "rlib"]` — `rlib` needed so other crates (pair) can import it
- Added `[features] testutils = ["soroban-sdk/testutils"]` — needed for pair's test environment

**Tests (`contracts/lp-token/tests/lp_token_test.rs`):**

| Test | What it covers |
|---|---|
| `initialize_stores_metadata` | name, symbol, decimals, total_supply all correct after init |
| `initialize_rejects_second_call` | panics `"lp token already initialized"` |
| `mint_increases_balance_and_supply` | balance and total_supply both increase |
| `burn_decreases_balance_and_supply` | balance and total_supply both decrease |
| `burn_panics_if_insufficient_balance` | panics `"insufficient balance"` |
| `transfer_moves_tokens` | from balance decreases, to balance increases |
| `transfer_panics_if_insufficient_balance` | panics `"insufficient balance"` |
| `transfer_from_uses_allowance` | deducts allowance, moves tokens correctly |
| `transfer_from_panics_if_insufficient_allowance` | panics `"insufficient allowance"` |
| `approve_and_allowance` | allowance starts at 0, set correctly after approve |

---

### Layer 2 — Token cross-contract helpers (no new tests)

**File:** `contracts/pair/src/token.rs`

**What it is:** Five thin wrappers around Soroban cross-contract calls. No logic — pure call forwarding. Used by `pair.rs` to interact with SEP-41 tokens and the LP token.

**Implementation:**

```
transfer(env, token, from, to, amount)   → TokenClient::new(env, token).transfer(from, to, amount)
balance(env, token, account)             → TokenClient::new(env, token).balance(account)
mint(env, lp_token, to, amount)          → LpTokenClient::new(env, lp_token).mint(to, amount)
burn(env, lp_token, from, amount)        → LpTokenClient::new(env, lp_token).burn(from, amount)
total_supply(env, lp_token)              → LpTokenClient::new(env, lp_token).total_supply()
```

`TokenClient` comes from `soroban_sdk::token::Client` — the standard SEP-41 interface.
`LpTokenClient` is generated by the Soroban SDK macros from the lp-token crate.

**Cargo changes to `contracts/pair/Cargo.toml`:**
- Added `lp-token = { path = "../lp-token" }` to `[dependencies]`
- Added `lp-token = { path = "../lp-token", features = ["testutils"] }` to `[dev-dependencies]`

---

### Layer 3 — Swap execution (5 new tests)

**File:** `contracts/pair/src/pair.rs`

**Why swap and not add/remove liquidity:** Swap is the most visible function in a DEX. A working swap with tests is the strongest signal to reviewers that this is a real project. `add_liquidity` and `remove_liquidity` are left as contributor issues — they are well-defined, testable, and good Soroban issues.

**Execution flow:**

```
1. to.require_auth()
2. Read token_a, token_b, reserve_a, reserve_b from storage
3. Identify token_out, reserve_in, reserve_out from token_in
   → panics "token_in is not part of this pair" if neither
4. amount_in = token::balance(token_in, this_contract) - reserve_in
   → balance delta, never trusts caller-supplied amount
5. amount_out = math::get_amount_out(amount_in, reserve_in, reserve_out)
   → applies 0.3% fee
6. assert amount_out >= min_amount_out
   → panics "slippage: amount_out below minimum"
7. token::transfer(token_out, this → to, amount_out)
8. Read new live balances balance_a, balance_b after transfer
9. math::check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)
   → panics "invariant violated: k decreased after swap" if k decreased
10. Update stored reserve_a = balance_a, reserve_b = balance_b
11. Return amount_out
```

**Tests (`contracts/pair/tests/swap_test.rs`):**

| Test | What it covers |
|---|---|
| `swap_a_for_b_returns_correct_amount` | correct output (90,661) for 100k in on 1M/1M pool, reserves updated |
| `swap_b_for_a_returns_correct_amount` | symmetric direction works identically |
| `swap_rejects_slippage` | panics when min_amount_out exceeds pool output |
| `swap_rejects_unknown_token` | panics when token_in is not in the pair |
| `swap_panics_if_nothing_sent` | panics when no tokens transferred before calling |

**Test setup note:** Since `add_liquidity` is a contributor issue, swap tests seed reserves using `env.as_contract()` to write directly to the pair's storage. This is a test-only pattern that simulates what `add_liquidity` will do in production. Real tokens are deployed using `register_stellar_asset_contract_v2`.

---

## Test count by session

| Session | Tests added | Total |
|---|---|---|
| Before session 1 | 14 | 14 |
| Session 1 (LP token + swap) | 15 | 29 |

---

## Open contributor issues

See `docs/issues.md` for full acceptance criteria on each.

| Issue | Layer | Description |
|---|---|---|
| #9 | 3 | `pair::add_liquidity` |
| #10 | 3 | `pair::remove_liquidity` |
| #12 | 3 | Pair integration tests (full lifecycle) |
| #13 | 4 | `factory::create_pair` |
| #14 | 4 | `factory::get_pair` |
| #15 | 5 | `sdk/pair.ts` — getReserves, getQuote |
| #16 | 5 | `sdk/pair.ts` — addLiquidity, removeLiquidity tx builders |
| #17 | 5 | `sdk/router.ts` — quote, buildSwapTx |
| #18 | 6 | End-to-end integration test |
| #19 | 6 | Testnet deploy script |
| #20 | 6 | Testnet seed-liquidity script |
