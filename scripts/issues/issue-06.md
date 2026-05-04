**Gospel Dex — Contract (Soroban / Stellar)**

Implement admin-only `burn` on the LP token contract (`contracts/lp-token/src/lib.rs`). This is called by the pair contract during `remove_liquidity`. Auth comes from the pair (admin), not from the LP holder — the pair burns tokens it holds itself.

Depends on GD-LP-001 and GD-LP-002 being merged first.

## Tasks

- Implement `burn(env, from, amount)`:
  1. Read `admin` from `env.storage().instance()`
  2. `admin.require_auth()` — only the pair contract can burn
  3. Read `from` balance from persistent storage
  4. Assert balance >= amount, panic `"insufficient balance"`
  5. Deduct `amount` from `from` balance
  6. Deduct `amount` from `TotalSupply` in instance storage

- Add tests: admin can burn (balance and total supply decrease), non-admin cannot burn, insufficient balance panics

## Additional Requirements

**Important:** auth must come from `admin` (the pair contract), not from `from`. In normal operation `from` will be the pair contract's own address — the pair sends LP tokens to itself then burns them. No unsafe code.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-006)
- [ ] `burn` decreases `from` balance and total supply
- [ ] `burn` requires auth from admin, not from `from`
- [ ] `burn` panics if balance is insufficient
- [ ] `cargo test` passes for the workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass
