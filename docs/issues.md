# Issues

This file is the source of truth for all GitHub issues on this project. Each issue has a title, label, description, and acceptance criteria. Copy these directly into GitHub.

Completed issues are documented at the bottom with what was implemented and which tests cover them.

---

## Open Issues

---

### Issue #1 — Implement LP token `initialize`

**Labels:** `layer-1` `good first issue`

**File:** `contracts/lp-token/src/lib.rs`

**Description:**

The LP token contract needs a one-time initialization function that stores the admin (the pair contract), the token name, symbol, and sets total supply to 0.

The `admin` address is the pair contract that will be deployed alongside this LP token. Only the admin can call `mint` and `burn`.

**Implementation:**

In `lp-token/src/lib.rs`, implement `initialize`:

```rust
pub fn initialize(env: Env, admin: Address, name: String, symbol: String)
```

Steps:
1. Assert the contract has not already been initialized (check if `Admin` key exists in storage)
2. Store `admin` under `DataKey::Admin` using `env.storage().instance()`
3. Store `name` under `DataKey::Name` using `env.storage().instance()`
4. Store `symbol` under `DataKey::Symbol` using `env.storage().instance()`
5. Store `0_i128` under `DataKey::TotalSupply` using `env.storage().instance()`

**Acceptance criteria:**
- [ ] `initialize` stores admin, name, symbol, and sets total supply to 0
- [ ] A second call to `initialize` panics with `"lp token already initialized"`
- [ ] Tests cover both cases

---

### Issue #2 — Implement LP token SEP-41 read functions

**Labels:** `layer-1` `good first issue`

**File:** `contracts/lp-token/src/lib.rs`

**Depends on:** Issue #1

**Description:**

Implement the read-only SEP-41 functions on the LP token contract.

**Implementation:**

```rust
pub fn name(env: Env) -> String          // read DataKey::Name from instance storage
pub fn symbol(env: Env) -> String        // read DataKey::Symbol from instance storage
pub fn decimals(_env: Env) -> u32        // always return 7 — already done
pub fn total_supply(env: Env) -> i128    // read DataKey::TotalSupply from instance storage
pub fn balance(env: Env, id: Address) -> i128  // read DataKey::Balance(id) from persistent storage, default 0
pub fn allowance(env: Env, from: Address, spender: Address) -> i128  // read DataKey::Allowance(from, spender) from persistent storage, default 0
```

Use `env.storage().instance()` for `name`, `symbol`, `total_supply`.
Use `env.storage().persistent()` for `balance` and `allowance` — these are per-account entries.

**Acceptance criteria:**
- [ ] All six functions return correct values after `initialize`
- [ ] `balance` returns 0 for an address with no balance
- [ ] `allowance` returns 0 for a pair with no approval
- [ ] Tests cover all functions

---

### Issue #3 — Implement LP token `transfer` and `transfer_from`

**Labels:** `layer-1`

**File:** `contracts/lp-token/src/lib.rs`

**Depends on:** Issue #2

**Description:**

Implement token transfers on the LP token contract.

**Implementation:**

`transfer(from, to, amount)`:
1. `from.require_auth()`
2. Read `from` balance from persistent storage
3. Assert `from` balance >= `amount`, panic `"insufficient balance"`
4. Deduct `amount` from `from` balance
5. Add `amount` to `to` balance
6. Write both balances back to persistent storage

`transfer_from(spender, from, to, amount)`:
1. `spender.require_auth()`
2. Read allowance for `(from, spender)`
3. Assert allowance >= `amount`, panic `"insufficient allowance"`
4. Deduct `amount` from allowance, write back
5. Perform the same balance transfer as above

**Acceptance criteria:**
- [ ] `transfer` moves tokens and requires auth from `from`
- [ ] `transfer` panics if balance is insufficient
- [ ] `transfer_from` deducts from allowance and moves tokens
- [ ] `transfer_from` panics if allowance is insufficient
- [ ] Tests cover happy path and both panic cases

---

### Issue #4 — Implement LP token `approve` and `allowance`

**Labels:** `layer-1` `good first issue`

**File:** `contracts/lp-token/src/lib.rs`

**Depends on:** Issue #2

**Description:**

Implement the approval mechanism on the LP token contract.

**Implementation:**

`approve(from, spender, amount, expiration_ledger)`:
1. `from.require_auth()`
2. Store `(amount, expiration_ledger)` under `DataKey::Allowance(from, spender)` in persistent storage

`allowance(from, spender) → i128`:
1. Read `DataKey::Allowance(from, spender)` from persistent storage
2. If not set, return 0
3. If `expiration_ledger < env.ledger().sequence()`, return 0 (expired)
4. Return the stored amount

**Acceptance criteria:**
- [ ] `approve` stores the allowance with expiry
- [ ] `allowance` returns 0 for an expired or unset allowance
- [ ] `allowance` returns the correct amount for a valid approval
- [ ] Tests cover all cases

---

### Issue #5 — Implement LP token `mint`

**Labels:** `layer-1`

**File:** `contracts/lp-token/src/lib.rs`

**Depends on:** Issue #1, Issue #2

**Description:**

Implement admin-only minting on the LP token contract. This is called by the pair contract during `add_liquidity`.

**Implementation:**

`mint(to, amount)`:
1. Read `admin` from instance storage
2. `admin.require_auth()` — only the pair contract can mint
3. Add `amount` to `to` balance in persistent storage
4. Add `amount` to `TotalSupply` in instance storage

**Acceptance criteria:**
- [ ] `mint` increases the recipient's balance and total supply
- [ ] `mint` requires auth from the admin address
- [ ] A non-admin caller cannot mint
- [ ] Tests cover both cases

---

### Issue #6 — Implement LP token `burn`

**Labels:** `layer-1`

**File:** `contracts/lp-token/src/lib.rs`

**Depends on:** Issue #1, Issue #2

**Description:**

Implement admin-only burning on the LP token contract. This is called by the pair contract during `remove_liquidity`. Auth comes from the pair (admin), not from the LP holder.

**Implementation:**

`burn(from, amount)`:
1. Read `admin` from instance storage
2. `admin.require_auth()` — only the pair contract can burn
3. Read `from` balance from persistent storage
4. Assert balance >= `amount`, panic `"insufficient balance"`
5. Deduct `amount` from `from` balance
6. Deduct `amount` from `TotalSupply`

**Acceptance criteria:**
- [ ] `burn` decreases the `from` balance and total supply
- [ ] `burn` requires auth from the admin (pair contract), not from `from`
- [ ] `burn` panics if balance is insufficient
- [ ] Tests cover all cases

---

### Issue #7 — LP token tests

**Labels:** `layer-1` `tests`

**File:** `contracts/lp-token/tests/` *(create this directory)*

**Depends on:** Issues #1–6

**Description:**

Write a full test suite for the LP token contract covering the complete SEP-41 interface. Use the Soroban test environment (`Env::default()`) and register the contract with `env.register_contract`.

Look at `contracts/pair/tests/initialize_test.rs` for the test pattern.

**Acceptance criteria:**
- [ ] Tests for `initialize` (happy path + double-init panic)
- [ ] Tests for `mint` (happy path + non-admin panic)
- [ ] Tests for `burn` (happy path + insufficient balance panic)
- [ ] Tests for `transfer` (happy path + insufficient balance panic)
- [ ] Tests for `transfer_from` (happy path + insufficient allowance panic)
- [ ] Tests for `approve` + `allowance` (happy path + expiry)
- [ ] All tests pass with `cargo test`

---

### Issue #8 — Implement `pair/token.rs` helpers

**Labels:** `layer-2`

**File:** `contracts/pair/src/token.rs`

**Depends on:** Issues #1–6 (LP token interface must be defined)

**Description:**

Implement the five cross-contract call helpers in `token.rs`. These are thin wrappers — no logic, just forwarding calls to SEP-41 token contracts using `soroban_sdk::token::Client`.

**Implementation:**

```rust
use soroban_sdk::token::Client as TokenClient;
```

`transfer(env, token, from, to, amount)`:
- `TokenClient::new(env, token).transfer(from, to, amount)`

`mint(env, lp_token, to, amount)`:
- Call `lp_token.mint(to, amount)` — use the LP token's client interface

`burn(env, lp_token, from, amount)`:
- Call `lp_token.burn(from, amount)`

`total_supply(env, lp_token) → i128`:
- Call `lp_token.total_supply()`

`balance(env, token, account) → i128`:
- `TokenClient::new(env, token).balance(account)`

**Acceptance criteria:**
- [ ] All five functions are implemented with no `todo!()`
- [ ] No logic in these functions — pure call forwarding
- [ ] Compiles cleanly with `cargo build`

---

### Issue #9 — Implement `pair::add_liquidity`

**Labels:** `layer-3`

**File:** `contracts/pair/src/pair.rs`

**Depends on:** Issue #8

**Description:**

Implement `add_liquidity` in `pair.rs`. The caller must transfer token_a and token_b to the pair contract address before calling this function. The pair derives deposited amounts from balance deltas — it never trusts a caller-supplied amount.

**Implementation steps** (from `docs/contracts.md`):

1. `to.require_auth()`
2. Read `reserve_a`, `reserve_b` from storage
3. Read `token_a`, `token_b`, `lp_token` addresses from storage
4. `balance_a = token::balance(env, &token_a, &env.current_contract_address())`
5. `balance_b = token::balance(env, &token_b, &env.current_contract_address())`
6. `amount_a = balance_a - reserve_a`
7. `amount_b = balance_b - reserve_b`
8. `lp_supply = token::total_supply(env, &lp_token)`
9. `lp_minted = math::calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply)`
10. If `lp_supply == 0`: `token::mint(env, &lp_token, &Address::zero(env), MINIMUM_LIQUIDITY)`
11. `token::mint(env, &lp_token, &to, lp_minted)`
12. Write `balance_a` to `DataKey::ReserveA`, `balance_b` to `DataKey::ReserveB`
13. Return `lp_minted`

**Acceptance criteria:**
- [ ] First deposit mints `sqrt(a*b) - MINIMUM_LIQUIDITY` LP tokens to `to`
- [ ] First deposit locks `MINIMUM_LIQUIDITY` to the zero address
- [ ] Subsequent deposits mint proportional LP tokens
- [ ] Reserves are updated to the new balances after deposit
- [ ] Panics if nothing was sent in (amount_a or amount_b is 0)
- [ ] Tests using mock SEP-41 tokens cover all cases

---

### Issue #10 — Implement `pair::remove_liquidity`

**Labels:** `layer-3`

**File:** `contracts/pair/src/pair.rs`

**Depends on:** Issue #8

**Description:**

Implement `remove_liquidity` in `pair.rs`. The caller must transfer LP tokens to the pair contract address before calling. The pair reads how many LP tokens it holds and burns them.

**Implementation steps** (from `docs/contracts.md`):

1. `to.require_auth()`
2. Read `reserve_a`, `reserve_b`, `token_a`, `token_b`, `lp_token` from storage
3. `lp_amount = token::balance(env, &lp_token, &env.current_contract_address())`
4. `lp_supply = token::total_supply(env, &lp_token)`
5. `amount_a = lp_amount * reserve_a / lp_supply`
6. `amount_b = lp_amount * reserve_b / lp_supply`
7. `token::burn(env, &lp_token, &env.current_contract_address(), lp_amount)`
8. `token::transfer(env, &token_a, &env.current_contract_address(), &to, amount_a)`
9. `token::transfer(env, &token_b, &env.current_contract_address(), &to, amount_b)`
10. Write `reserve_a - amount_a` to `DataKey::ReserveA`, `reserve_b - amount_b` to `DataKey::ReserveB`
11. Return `(amount_a, amount_b)`

**Acceptance criteria:**
- [ ] Burns LP tokens and returns proportional reserves to `to`
- [ ] Reserves are updated after withdrawal
- [ ] Panics if no LP tokens were sent in
- [ ] Tests cover happy path and edge cases

---

### Issue #11 — Implement `pair::swap`

**Labels:** `layer-3`

**File:** `contracts/pair/src/pair.rs`

**Depends on:** Issue #8

**Description:**

Implement `swap` in `pair.rs`. The caller must transfer `token_in` to the pair contract address before calling. The pair derives `amount_in` from the balance delta and asserts the post-swap invariant before updating reserves.

**Implementation steps** (from `docs/contracts.md`):

1. `to.require_auth()`
2. Read `token_a`, `token_b`, `reserve_a`, `reserve_b` from storage
3. Determine `token_out`, `reserve_in`, `reserve_out` from `token_in`
4. `amount_in = token::balance(env, &token_in, &env.current_contract_address()) - reserve_in`
5. `amount_out = math::get_amount_out(amount_in, reserve_in, reserve_out)`
6. Assert `amount_out >= min_amount_out`, panic `"slippage: amount_out below minimum"`
7. `token::transfer(env, &token_out, &env.current_contract_address(), &to, amount_out)`
8. Read new live balances `balance_a`, `balance_b`
9. Determine `amount_in_a`, `amount_in_b` (one is `amount_in`, the other is 0)
10. `math::check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)`
11. Write `balance_a` to `DataKey::ReserveA`, `balance_b` to `DataKey::ReserveB`
12. Return `amount_out`

**Acceptance criteria:**
- [ ] Swaps token_a → token_b and token_b → token_a correctly
- [ ] Applies 0.3% fee (output matches `get_amount_out`)
- [ ] Panics if `amount_out < min_amount_out`
- [ ] Panics if `token_in` is not part of the pair
- [ ] Panics if nothing was sent in
- [ ] Post-swap invariant is checked — a manipulated swap that violates `k` panics
- [ ] Reserves are updated after the swap
- [ ] Tests cover all cases using mock tokens

---

### Issue #12 — Pair integration tests

**Labels:** `layer-3` `tests`

**File:** `contracts/pair/tests/`

**Depends on:** Issues #9, #10, #11

**Description:**

Write integration tests for the full pair flow using mock SEP-41 tokens deployed in the Soroban test environment. Tests should cover the complete lifecycle: initialize → add liquidity → swap → remove liquidity.

Use `soroban_sdk::token::StellarAssetClient` or register a mock token contract to simulate real token balances.

**Acceptance criteria:**
- [ ] Test: first deposit mints correct LP tokens, locks MINIMUM_LIQUIDITY
- [ ] Test: second deposit mints proportional LP tokens
- [ ] Test: swap A→B produces correct output and updates reserves
- [ ] Test: swap B→A produces correct output and updates reserves
- [ ] Test: remove liquidity returns correct proportional amounts
- [ ] Test: full lifecycle — deposit, swap, withdraw — leaves reserves consistent
- [ ] All tests pass with `cargo test`

---

### Issue #13 — Implement `factory::create_pair`

**Labels:** `layer-4`

**File:** `contracts/factory/src/lib.rs`

**Depends on:** Issues #9–12 (pair must be deployable)

**Description:**

Implement `create_pair` in the factory. This function deploys a new pair contract and registers it in the factory's registry. Tokens are sorted before storage so `(A,B)` and `(B,A)` always resolve to the same pair.

**Implementation steps:**

1. Assert `token_a != token_b`
2. Sort: `let (token0, token1) = if token_a < token_b { (token_a, token_b) } else { (token_b, token_a) }`
3. Assert no pair exists yet for `(token0, token1)` — read `DataKey::Pair(token0.clone(), token1.clone())` from instance storage, panic `"pair already exists"` if set
4. Deploy LP token contract using `env.deployer()` with a salt derived from `(token0, token1)`
5. Deploy pair contract using `env.deployer()` with a salt derived from `(token0, token1)`
6. Call `pair.initialize(token0, token1, lp_token_address)`
7. Store pair address under `DataKey::Pair(token0, token1)` in instance storage
8. Return pair address

**Acceptance criteria:**
- [ ] Deploys a new pair and returns its address
- [ ] `(A,B)` and `(B,A)` produce the same pair address
- [ ] Panics if a pair for this combination already exists
- [ ] Pair address is deterministic (same tokens always produce same address)
- [ ] Tests cover all cases

---

### Issue #14 — Implement `factory::get_pair`

**Labels:** `layer-4` `good first issue`

**File:** `contracts/factory/src/lib.rs`

**Depends on:** Issue #13

**Description:**

Implement `get_pair` in the factory. Returns the pair address for a token combination, or `None` if no pair exists. Tokens are sorted before lookup.

**Implementation:**

```rust
pub fn get_pair(env: Env, token_a: Address, token_b: Address) -> Option<Address>
```

1. Sort tokens: `let (token0, token1) = if token_a < token_b { ... }`
2. Read `DataKey::Pair(token0, token1)` from instance storage
3. Return the value, or `None` if not set

**Acceptance criteria:**
- [ ] Returns the correct pair address for a registered pair
- [ ] Returns `None` for an unregistered pair
- [ ] `get_pair(A, B)` and `get_pair(B, A)` return the same result
- [ ] Tests cover all cases

---

### Issue #15 — Implement `sdk/pair.ts` — `getReserves` and `getQuote`

**Labels:** `layer-5`

**File:** `sdk/src/pair.ts`

**Depends on:** Issues #9–12

**Description:**

Implement the read-only methods on `PairClient` using Soroban RPC `simulateTransaction`.

`getReserves()`:
- Build a transaction invoking `get_reserves()` on the pair contract
- Simulate it via `SorobanRpc.Server.simulateTransaction`
- Parse and return `[bigint, bigint]`

`getQuote(tokenIn, amountIn)`:
- Build a transaction invoking `get_quote(token_in, amount_in)`
- Simulate and return `bigint | null` (null if reserves are zero)

**Acceptance criteria:**
- [ ] `getReserves` returns current reserves from a deployed pair
- [ ] `getQuote` returns the expected output amount
- [ ] `getQuote` returns `null` when reserves are zero
- [ ] Both methods use `simulateTransaction` — no state changes

---

### Issue #16 — Implement `sdk/pair.ts` — `addLiquidity` and `removeLiquidity` transaction builders

**Labels:** `layer-5`

**File:** `sdk/src/pair.ts`

**Depends on:** Issue #15

**Description:**

Implement transaction builders for liquidity operations. These return unsigned XDR transactions that the caller signs and submits.

`addLiquidity(sourceAccount, amountA, amountB)`:
1. Build two token transfer operations: `token_a.transfer(source, pair, amountA)` and `token_b.transfer(source, pair, amountB)`
2. Build a `pair.add_liquidity(source)` invocation
3. Combine into one transaction and return XDR

`removeLiquidity(sourceAccount, lpAmount)`:
1. Build `lp_token.transfer(source, pair, lpAmount)`
2. Build `pair.remove_liquidity(source)`
3. Combine and return XDR

**Acceptance criteria:**
- [ ] Returns valid unsigned XDR
- [ ] Token transfers and contract call are in the same transaction
- [ ] Caller can sign and submit the returned XDR

---

### Issue #17 — Implement `sdk/router.ts`

**Labels:** `layer-5`

**File:** `sdk/src/router.ts`

**Depends on:** Issue #15

**Description:**

Implement `RouterClient`.

`quote(tokenIn, amountIn)`:
- Delegate to `PairClient.getQuote`

`buildSwapTx(sourceAccount, tokenIn, amountIn, slippageBps)`:
1. Get quote via `getQuote`
2. `minAmountOut = quote * (10000 - slippageBps) / 10000`
3. Build `token_in.transfer(source, pair, amountIn)`
4. Build `pair.swap(source, token_in, minAmountOut)`
5. Combine and return XDR

**Acceptance criteria:**
- [ ] `quote` returns the expected output
- [ ] `buildSwapTx` applies slippage correctly
- [ ] Returns valid unsigned XDR

---

### Issue #18 — End-to-end integration test

**Labels:** `layer-6` `tests`

**File:** `tests/integration/`

**Depends on:** All Layer 1–5 issues

**Description:**

Write an end-to-end test that exercises the full protocol lifecycle on the Soroban test environment:

1. Deploy factory
2. Call `factory.create_pair(token_a, token_b)` — gets pair address
3. Add liquidity to the pair
4. Swap token_a for token_b
5. Swap token_b for token_a
6. Remove liquidity
7. Assert final balances are consistent

**Acceptance criteria:**
- [ ] Full lifecycle runs without panic
- [ ] Reserves after all operations are consistent with expected math
- [ ] LP token total supply returns to `MINIMUM_LIQUIDITY` after full withdrawal

---

### Issue #19 — Testnet deployment script

**Labels:** `layer-6`

**File:** `scripts/deploy.ts`

**Depends on:** Issue #18

**Description:**

Implement `deploy.ts` to deploy the factory and a seed pair to Stellar testnet.

Steps:
1. Load a funded testnet keypair from env
2. Build and deploy the factory contract using Stellar CLI or SDK
3. Call `factory.initialize(admin)`
4. Deploy a seed pair (e.g. wrapped XLM / USDC) via `factory.create_pair`
5. Log all deployed contract addresses

**Acceptance criteria:**
- [ ] Script runs end-to-end on testnet without error
- [ ] Logs factory address and seed pair address
- [ ] Deployed contracts are queryable via Stellar explorer

---

### Issue #20 — Testnet liquidity seeding script

**Labels:** `layer-6`

**File:** `scripts/seed-liquidity.ts`

**Depends on:** Issue #19

**Description:**

Implement `seed-liquidity.ts` to add initial liquidity to a deployed pair on testnet.

Steps:
1. Load keypair and pair address from env/args
2. Transfer token_a and token_b to the pair address
3. Call `pair.add_liquidity(source)`
4. Log LP tokens received and new reserves

**Acceptance criteria:**
- [ ] Script adds liquidity to a live testnet pair
- [ ] Logs LP tokens minted and resulting reserves

---

## Completed Issues

---

### ✅ C-1 — Constant-product swap math (`get_amount_out`)

**File:** `contracts/pair/src/math.rs`

**What was implemented:**

The core AMM pricing formula with 0.3% fee:

```rust
pub fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128
```

Formula: `(amount_in * 997 * reserve_out) / (reserve_in * 1000 + amount_in * 997)`

Panics if any input is zero or negative.

**Tests:** `contracts/pair/tests/math_test.rs`
- `amount_out_applies_constant_product_fee` — verifies output for a known input/reserve pair

---

### ✅ C-2 — LP token mint math (`calc_lp_tokens_to_mint`)

**File:** `contracts/pair/src/math.rs`

**What was implemented:**

```rust
pub fn calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply) -> i128
```

- First deposit: `sqrt(amount_a * amount_b) - MINIMUM_LIQUIDITY`
- Subsequent: `min(amount_a * lp_supply / reserve_a, amount_b * lp_supply / reserve_b)`
- Uses `checked_mul` on the initial product to guard against overflow

**Tests:** `contracts/pair/tests/math_test.rs`
- `first_lp_deposit_mints_geometric_mean_minus_minimum_liquidity`
- `first_lp_deposit_must_exceed_minimum_liquidity`
- `later_lp_deposit_mints_min_proportional_share`

---

### ✅ C-3 — Post-swap invariant check (`check_invariant`)

**File:** `contracts/pair/src/math.rs`

**What was implemented:**

```rust
pub fn check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)
```

Asserts: `(1000·balance_a − 3·amount_in_a) · (1000·balance_b − 3·amount_in_b) >= 1_000_000 · reserve_a · reserve_b`

This is the fee-adjusted constant-product safety check. Called at the end of every swap before reserves are updated. Panics `"invariant violated: k decreased after swap"` if the check fails.

**Tests:** `contracts/pair/tests/math_test.rs`
- `check_invariant_passes_after_valid_swap`
- `check_invariant_fails_if_k_decreases`

---

### ✅ C-4 — Integer square root (`sqrt`) and `MINIMUM_LIQUIDITY`

**File:** `contracts/pair/src/math.rs`

**What was implemented:**

```rust
pub fn sqrt(n: i128) -> i128          // Babylonian method, rounds down
pub const MINIMUM_LIQUIDITY: i128 = 1_000
```

`MINIMUM_LIQUIDITY` is permanently locked on the first deposit (burned to the zero address). Prevents the LP share price manipulation attack.

**Tests:** `contracts/pair/tests/math_test.rs`
- `sqrt_rounds_down_to_integer_root` — covers 0, 1, non-perfect square, perfect square

---

### ✅ C-5 — Pair contract initialization

**File:** `contracts/pair/src/pair.rs`, `contracts/pair/src/lib.rs`

**What was implemented:**

```rust
pub fn initialize(env: Env, token_a: Address, token_b: Address, lp_token: Address)
```

Stores `token_a`, `token_b`, `lp_token` and sets `reserve_a = reserve_b = 0`. Rejects identical tokens and double initialization.

**Tests:** `contracts/pair/tests/initialize_test.rs`
- `initialize_sets_public_reserves_to_zero`
- `initialize_rejects_same_token_pair`
- `initialize_rejects_second_call`

---

### ✅ C-6 — Pair reserve reads and quote simulation

**File:** `contracts/pair/src/pair.rs`, `contracts/pair/src/lib.rs`

**What was implemented:**

```rust
pub fn get_reserves(env: Env) -> (i128, i128)
pub fn get_quote(env: Env, token_in: Address, amount_in: i128) -> Option<i128>
```

`get_reserves` returns cached `(reserve_a, reserve_b)`.

`get_quote` simulates a swap with fee. Returns `None` if reserves are zero. Panics if `token_in` is not part of the pair.

**Tests:** `contracts/pair/tests/initialize_test.rs`
- `get_quote_returns_none_on_empty_reserves`
- `get_quote_rejects_unknown_token`

---

### ✅ C-7 — Factory initialization and admin

**File:** `contracts/factory/src/lib.rs`

**What was implemented:**

```rust
pub fn initialize(env: Env, admin: Address)
pub fn admin(env: Env) -> Address
```

Stores the factory admin on first call. Rejects double initialization.

**Tests:** `contracts/factory/tests/initialize_test.rs`
- `initialize_sets_admin`
- `initialize_rejects_second_call`

---

### ✅ C-8 — Token sorting in factory

**File:** `contracts/factory/src/lib.rs`

**What was implemented:**

Both `create_pair` and `get_pair` sort token addresses (`token0 = min(a,b)`, `token1 = max(a,b)`) before any storage or lookup. This ensures `(A,B)` and `(B,A)` always resolve to the same canonical pair.

**Tests:** Covered by factory tests. Full sorting tests will be added in Issue #13/#14.
