**Gospel Dex — Contract (Soroban / Stellar)**

Implement `transfer` and `transfer_from` on the LP token contract (`contracts/lp-token/src/lib.rs`). These are the standard SEP-41 transfer functions.

Depends on GD-LP-002 (read functions) being merged first.

## Tasks

- Implement `transfer(env, from, to, amount)`:
  1. `from.require_auth()`
  2. Read `from` balance from persistent storage
  3. Assert balance >= amount, panic `"insufficient balance"`
  4. Deduct from `from`, add to `to`, write both back to persistent storage

- Implement `transfer_from(env, spender, from, to, amount)`:
  1. `spender.require_auth()`
  2. Read allowance for `(from, spender)` from persistent storage
  3. Assert allowance >= amount, panic `"insufficient allowance"`
  4. Deduct from allowance, write back
  5. Perform the same balance transfer as above

- Add tests covering happy path and both panic cases for each function

## Additional Requirements

Auth is explicit in Soroban — use `address.require_auth()`, there is no `msg.sender`. No unsafe code.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-003)
- [ ] `transfer` moves tokens and requires auth from `from`
- [ ] `transfer` panics if balance is insufficient
- [ ] `transfer_from` deducts allowance and moves tokens
- [ ] `transfer_from` panics if allowance is insufficient
- [ ] `cargo test` passes for the workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass
