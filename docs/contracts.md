# Contract Reference

This document describes every contract in the protocol, how they relate to each other, and the exact behaviour of every function. Read this before writing or reviewing any contract code.

---

## How the Contracts Fit Together

There are three contracts. They have a strict dependency order:

```
Factory
  └── deploys and registers ──► Pair
                                  └── mints/burns ──► LP Token
```

**Factory** is the entry point for creating pairs. It holds a registry of all deployed pairs and ensures no duplicate pairs exist.

**Pair** is the core contract. One instance exists per token pair. It holds the reserves of both tokens, prices trades, and manages liquidity. It calls into the LP Token contract to mint and burn shares.

**LP Token** is a standard SEP-41 token. It has no knowledge of the pair — it just mints and burns on instruction from whoever is set as its admin (the pair contract). One LP Token contract is deployed per pair.

### Deployment order

When creating a new trading pair:

1. Deploy an LP Token contract
2. Deploy a Pair contract, passing both token addresses and the LP Token address
3. Call `pair.initialize(token_a, token_b, lp_token)` — this sets the pair as the LP token's admin
4. Register the pair in the Factory via `factory.create_pair(token_a, token_b)`

In the final implementation, steps 1–4 happen atomically inside `factory.create_pair`.

---

## Factory Contract

**File:** `contracts/factory/src/lib.rs`

The factory is the registry. Its only job is to track which pair contract exists for each token combination, and to be the single place that creates new pairs.

### Storage

| Key | Type | Description |
|---|---|---|
| `Admin` | `Address` | The factory admin — can configure the factory |
| `Pair(token0, token1)` | `Address` | Pair address for a sorted token combination *(to implement)* |

### Functions

---

#### `initialize(admin: Address)`

One-time setup. Stores the factory admin. Panics if called a second time.

```
Panics: "factory has already been initialized"
```

---

#### `admin() → Address`

Returns the stored admin address. Panics if the factory has not been initialized.

---

#### `create_pair(token_a: Address, token_b: Address) → Address` *(to implement)*

Deploys a new pair for the given token combination and registers it.

Rules:
- Panics if `token_a == token_b`
- Sorts tokens before storage: `token0 = min(token_a, token_b)`, `token1 = max(token_a, token_b)` — this ensures `(A,B)` and `(B,A)` always resolve to the same pair
- Panics if a pair for this combination already exists
- Deploys the pair contract using `env.deployer().with_address(deployer, salt)` where salt is derived from `(token0, token1)` — gives deterministic, off-chain-computable addresses
- Stores the pair address under `Pair(token0, token1)`
- Returns the new pair address

---

#### `get_pair(token_a: Address, token_b: Address) → Option<Address>` *(to implement)*

Returns the pair address for the given token combination, or `None` if no pair exists.

- Sorts tokens before lookup — `(A,B)` and `(B,A)` return the same result

---

## Pair Contract

**Files:** `contracts/pair/src/`

One deployed instance per token pair. This is the invariant-critical contract — it holds user funds and enforces `reserve_a * reserve_b = k`. Every function that moves tokens must leave `k` equal to or greater than before.

### Storage

| Key | Type | Description |
|---|---|---|
| `TokenA` | `Address` | Address of token A |
| `TokenB` | `Address` | Address of token B |
| `LpToken` | `Address` | Address of the LP token contract |
| `ReserveA` | `i128` | Cached reserve of token A |
| `ReserveB` | `i128` | Cached reserve of token B |

Reserves are always the *last recorded* balances — not live balances. Live balances are read from the token contracts when needed and diffed against these cached values.

### Internal modules

| Module | Role |
|---|---|
| `lib.rs` | Public contract entry point. Thin wrappers — no logic here. |
| `pair.rs` | All state reads/writes and orchestration. Calls `math` and `token`. |
| `math.rs` | Pure functions. No storage. All AMM math lives here. |
| `token.rs` | Cross-contract call helpers for SEP-41 tokens and the LP token. |

### How token amounts are determined

The pair never accepts an `amount` parameter from the caller for any state-changing function. Instead:

1. The caller transfers tokens to the pair contract address first.
2. The pair reads its live token balance from the token contract.
3. It subtracts the cached reserve to get the delta — that is the amount that just arrived.

This means the pair cannot be tricked by a caller lying about how much they sent. The only source of truth is the actual balance.

### Functions

---

#### `initialize(token_a: Address, token_b: Address, lp_token: Address)`

One-time setup. Stores both token addresses, the LP token address, and sets both reserves to 0.

```
Panics: "pair has already been initialized"
Panics: "pair tokens must be different"
```

---

#### `add_liquidity(to: Address) → i128`

Caller sends token_a and token_b to the pair address, then calls this. Returns the number of LP tokens minted.

Steps (to implement in `pair.rs`):
1. `to.require_auth()`
2. Read `reserve_a`, `reserve_b` from storage
3. Read live `balance_a = token_a.balance(this)`, `balance_b = token_b.balance(this)`
4. `amount_a = balance_a - reserve_a`
5. `amount_b = balance_b - reserve_b`
6. `lp_supply = lp_token.total_supply()`
7. `lp_minted = math::calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply)`
8. If `lp_supply == 0`: mint `MINIMUM_LIQUIDITY` LP tokens to the zero address (permanently locked)
9. Mint `lp_minted` LP tokens to `to`
10. Update `reserve_a = balance_a`, `reserve_b = balance_b`
11. Return `lp_minted`

```
Panics: "amount_a must be positive"  (nothing was sent in)
Panics: "amount_b must be positive"  (nothing was sent in)
Panics: "insufficient initial liquidity"  (first deposit too small)
```

---

#### `remove_liquidity(to: Address) → (i128, i128)`

Caller sends LP tokens to the pair address, then calls this. Returns `(amount_a, amount_b)` sent back to `to`.

Steps (to implement in `pair.rs`):
1. `to.require_auth()`
2. `lp_amount = lp_token.balance(this)` — LP tokens the pair is holding
3. `lp_supply = lp_token.total_supply()`
4. `amount_a = lp_amount * reserve_a / lp_supply`
5. `amount_b = lp_amount * reserve_b / lp_supply`
6. Burn `lp_amount` LP tokens held by this contract
7. Transfer `amount_a` of token_a to `to`
8. Transfer `amount_b` of token_b to `to`
9. Update `reserve_a -= amount_a`, `reserve_b -= amount_b`
10. Return `(amount_a, amount_b)`

```
Panics: if lp_amount == 0  (no LP tokens were sent in)
```

---

#### `swap(to: Address, token_in: Address, min_amount_out: i128) → i128`

Caller sends `token_in` to the pair address, then calls this. Returns the amount of the other token sent to `to`.

Steps (to implement in `pair.rs`):
1. `to.require_auth()`
2. Identify `token_out`, `reserve_in`, `reserve_out` from `token_in`
3. `amount_in = token_in.balance(this) - reserve_in`
4. `amount_out = math::get_amount_out(amount_in, reserve_in, reserve_out)`
5. Require `amount_out >= min_amount_out` — slippage guard
6. Transfer `amount_out` of `token_out` to `to`
7. Read new live balances `balance_a`, `balance_b`
8. `math::check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)`
9. Update `reserve_a = balance_a`, `reserve_b = balance_b`
10. Return `amount_out`

```
Panics: "token_in is not part of this pair"
Panics: "amount_in must be positive"  (nothing was sent in)
Panics: "slippage: amount_out below minimum"
Panics: "invariant violated: k decreased after swap"
```

---

#### `get_reserves() → (i128, i128)`

Read-only. Returns `(reserve_a, reserve_b)` from storage. Returns `(0, 0)` before any liquidity is added.

---

#### `get_quote(token_in: Address, amount_in: i128) → Option<i128>`

Read-only. Simulates a swap and returns the expected output amount including the 0.3% fee. Does not execute anything, does not change state.

Returns `None` if the pair has no liquidity (reserves are zero).

```
Panics: "token_in is not part of this pair"
```

---

### Math (`math.rs`)

Pure functions — no storage, no side effects. All AMM calculations live here.

---

#### `get_amount_out(amount_in, reserve_in, reserve_out) → i128`

Calculates swap output with 0.3% fee applied.

```
amount_in_with_fee = amount_in * 997
amount_out = (amount_in_with_fee * reserve_out) / (reserve_in * 1000 + amount_in_with_fee)
```

```
Panics: if amount_in, reserve_in, or reserve_out <= 0
```

---

#### `calc_lp_tokens_to_mint(amount_a, amount_b, reserve_a, reserve_b, lp_supply) → i128`

Calculates how many LP tokens to mint for a deposit.

First deposit (`lp_supply == 0`):
```
lp_minted = sqrt(amount_a * amount_b) - MINIMUM_LIQUIDITY
```

Subsequent deposits:
```
lp_minted = min(amount_a * lp_supply / reserve_a, amount_b * lp_supply / reserve_b)
```

```
Panics: "insufficient initial liquidity"  (if sqrt(a*b) <= MINIMUM_LIQUIDITY)
Panics: if amount_a or amount_b <= 0
```

---

#### `check_invariant(balance_a, balance_b, amount_in_a, amount_in_b, reserve_a, reserve_b)`

Asserts the fee-adjusted constant-product invariant holds after a swap:

```
(1000 · balance_a − 3 · amount_in_a) · (1000 · balance_b − 3 · amount_in_b)
    >= 1_000_000 · reserve_a · reserve_b
```

Pass `0` for the token that was not the input side. Called at the end of every swap before reserves are updated.

```
Panics: "invariant violated: k decreased after swap"
```

---

#### `sqrt(n) → i128`

Integer square root using the Babylonian method. Rounds down.

#### `MINIMUM_LIQUIDITY: i128 = 1_000`

Permanently locked on the first deposit. Minted to the zero address and never recoverable. Prevents the LP share price manipulation attack where an attacker inflates the value of a single LP token by donating to an empty pool.

---

### Token helpers (`token.rs`)

Thin wrappers around `soroban_sdk::token::Client`. Used by `pair.rs` to make cross-contract calls. No logic — just call forwarding.

| Function | What it calls |
|---|---|
| `transfer(env, token, from, to, amount)` | `token.transfer(from, to, amount)` |
| `mint(env, lp_token, to, amount)` | `lp_token.mint(to, amount)` |
| `burn(env, lp_token, from, amount)` | `lp_token.burn(from, amount)` |
| `total_supply(env, lp_token)` | `lp_token.total_supply()` |
| `balance(env, token, account)` | `token.balance(account)` |

All of these are `todo!()` stubs. Implement them using `soroban_sdk::token::Client::new(env, token_address)`.

---

## LP Token Contract

**File:** `contracts/lp-token/src/lib.rs`

A standard SEP-41 fungible token. Represents a liquidity provider's share of a pair's reserves. One instance is deployed per pair.

The LP token has no knowledge of the pair. It just holds balances and allowances, and exposes mint/burn to its admin (the pair contract).

### Storage

| Key | Type | Description |
|---|---|---|
| `Admin` | `Address` | The pair contract — only address that can mint/burn |
| `Name` | `String` | Token name |
| `Symbol` | `String` | Token symbol |
| `TotalSupply` | `i128` | Total LP tokens in circulation |
| `Balance(address)` | `i128` | LP token balance per account |
| `Allowance(from, spender)` | `i128` | Approved spend amount |

Use `env.storage().instance()` for `Admin`, `Name`, `Symbol`, `TotalSupply`. Use `env.storage().persistent()` for `Balance` and `Allowance` — these are per-account and must survive ledger entry expiry.

### Functions

---

#### `initialize(admin: Address, name: String, symbol: String)`

One-time setup. Stores admin, name, symbol. Sets total supply to 0.

`admin` must be the pair contract address. The pair will call `mint` and `burn` — those calls require auth from admin.

---

#### SEP-41 reads

| Function | Returns | Notes |
|---|---|---|
| `name()` | `String` | Stored at init |
| `symbol()` | `String` | Stored at init |
| `decimals()` | `u32` | Always `7` — Stellar standard |
| `total_supply()` | `i128` | Sum of all balances |
| `balance(id: Address)` | `i128` | Balance of a specific account |
| `allowance(from: Address, spender: Address)` | `i128` | Approved amount |

---

#### `transfer(from: Address, to: Address, amount: i128)`

Moves `amount` from `from` to `to`.

- Requires auth from `from`
- Panics if `from` balance is insufficient

---

#### `transfer_from(spender: Address, from: Address, to: Address, amount: i128)`

Moves `amount` from `from` to `to` on behalf of `spender`.

- Requires auth from `spender`
- Deducts from `spender`'s allowance for `from`
- Panics if allowance or balance is insufficient

---

#### `approve(from: Address, spender: Address, amount: i128, expiration_ledger: u32)`

Sets `spender`'s allowance to spend `amount` of `from`'s tokens.

- Requires auth from `from`
- `expiration_ledger` is the ledger number after which the allowance expires — store this alongside the amount

---

#### `mint(to: Address, amount: i128)`

Mints `amount` new LP tokens to `to`. Increases `total_supply`.

- Requires auth from admin (the pair contract)
- Called by the pair during `add_liquidity`

---

#### `burn(from: Address, amount: i128)`

Burns `amount` LP tokens from `from`. Decreases `total_supply`.

- Requires auth from admin (the pair contract)
- Called by the pair during `remove_liquidity`
- The pair burns tokens it holds itself — `from` will be the pair's own address in normal operation
