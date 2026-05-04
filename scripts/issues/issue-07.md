**Gospel Dex — Contract (Soroban / Stellar)**

Write a full test suite for the LP token contract in `contracts/lp-token/tests/`. This issue covers test coverage for the complete SEP-41 interface once GD-LP-001 through GD-LP-006 are merged.

## Tasks

- Create `contracts/lp-token/tests/lp_token_test.rs`
- Add the test module to `contracts/lp-token/Cargo.toml` under `[[test]]`
- Write tests using `Env::default()` and `env.register_contract` — see `contracts/pair/tests/initialize_test.rs` for the pattern
- Cover:
  - `initialize` happy path and double-init panic
  - `mint` happy path and non-admin panic
  - `burn` happy path, non-admin panic, insufficient balance panic
  - `transfer` happy path and insufficient balance panic
  - `transfer_from` happy path and insufficient allowance panic
  - `approve` + `allowance` happy path and expiry

## Additional Requirements

No unsafe code. All tests must be deterministic.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-007)
- [ ] All listed test cases are present and pass
- [ ] `cargo test` passes for the full workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass
