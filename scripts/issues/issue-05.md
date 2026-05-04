**Gospel Dex — Contract (Soroban / Stellar)**

Implement admin-only `mint` on the LP token contract (`contracts/lp-token/src/lib.rs`). This is called by the pair contract during `add_liquidity` to issue LP tokens to liquidity providers.

Depends on GD-LP-001 and GD-LP-002 being merged first.

## Tasks

- Implement `mint(env, to, amount)`:
  1. Read `admin` from `env.storage().instance()`
  2. `admin.require_auth()` — only the pair contract can mint
  3. Add `amount` to `to` balance in persistent storage
  4. Add `amount` to `TotalSupply` in instance storage

- Add tests: admin can mint (balance and total supply increase), non-admin cannot mint

## Additional Requirements

Auth must come from the stored `admin` address (the pair contract), not from the `to` address. No unsafe code.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-005)
- [ ] `mint` increases recipient balance and total supply
- [ ] `mint` requires auth from the admin address
- [ ] A non-admin call panics
- [ ] `cargo test` passes for the workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass
