# Contract Reference

## AMM Contract

### `initialize(token_a, token_b, lp_token)`
Sets up the pool. Must be called once before any other function.

- `token_a` — Address of the first token (e.g. wrapped XLM)
- `token_b` — Address of the second token (any SEP-41 token)
- `lp_token` — Address of the LP token contract (AMM must be its admin)

Panics if already initialized.

---

### `add_liquidity(to, amount_a, amount_b) → lp_minted`
Deposits both tokens into the pool and mints LP tokens to `to`.

- Requires auth from `to`
- On first deposit: `lp_minted = sqrt(amount_a * amount_b)`
- On subsequent deposits: proportional to current reserves

---

### `remove_liquidity(to, lp_amount) → (amount_a, amount_b)`
Burns `lp_amount` LP tokens and returns the proportional share of reserves to `to`.

- Requires auth from `to`
- Returns `(amount_a, amount_b)` transferred back

---

### `swap(to, token_in, amount_in, min_amount_out) → amount_out`
Swaps `amount_in` of `token_in` for the other token.

- Requires auth from `to`
- Applies 0.3% fee
- Panics if `amount_out < min_amount_out` (slippage protection)

---

### `get_reserves() → (reserve_a, reserve_b)`
Read-only. Returns current pool reserves.

---

### `get_quote(token_in, amount_in) → amount_out`
Read-only. Simulates a swap and returns the expected output. Does not execute anything.

---

## LP Token Contract

Implements the full SEP-41 interface:

| Function         | Description                        |
|------------------|------------------------------------|
| `name()`         | Token name                         |
| `symbol()`       | Token symbol                       |
| `decimals()`     | Always 7 (Stellar standard)        |
| `total_supply()` | Total LP tokens in circulation     |
| `balance(id)`    | LP token balance of an address     |
| `transfer`       | Transfer LP tokens                 |
| `approve`        | Approve a spender                  |
| `allowance`      | Read allowance                     |
| `mint`           | Admin-only: mint LP tokens         |
| `burn`           | Burn caller's LP tokens            |
