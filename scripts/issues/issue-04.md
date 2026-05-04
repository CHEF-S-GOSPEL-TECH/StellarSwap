**Gospel Dex — Contract (Soroban / Stellar)**

Implement `approve` and the expiry-aware `allowance` read on the LP token contract (`contracts/lp-token/src/lib.rs`).

Depends on GD-LP-002 (read functions) being merged first.

## Tasks

- Implement `approve(env, from, spender, amount, expiration_ledger)`:
  1. `from.require_auth()`
  2. Store `(amount, expiration_ledger)` under `DataKey::Allowance(from, spender)` in persistent storage

- Update `allowance(env, from, spender) -> i128`:
  1. Read `DataKey::Allowance(from, spender)` from persistent storage
  2. If not set, return 0
  3. If `expiration_ledger < env.ledger().sequence()`, return 0 (expired)
  4. Return the stored amount

- Add tests: valid approval, expired approval, unset allowance

## Additional Requirements

`expiration_ledger` is a Soroban ledger sequence number, not a timestamp. Store it alongside the amount as a tuple or struct. No unsafe code.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-004)
- [ ] `approve` stores the allowance with expiry
- [ ] `allowance` returns 0 for expired or unset allowances
- [ ] `allowance` returns the correct amount for a valid approval
- [ ] `cargo test` passes for the workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass
