**Gospel Dex — Contract (Soroban / Stellar)**

Implement `initialize` on the LP token contract (`contracts/lp-token/src/lib.rs`). This is a one-time setup function that stores the admin (the pair contract address), token name, symbol, and sets total supply to 0. Only the admin can later call `mint` and `burn`.

This item is part of the Gospel Dex Layer 1 engineering batch. Keep changes small, reviewable, and covered by tests.

## Tasks

- Scope and implement in `contracts/lp-token/src/lib.rs`: the `initialize` function
- Steps:
  1. Assert not already initialized (check if `Admin` key exists in `env.storage().instance()`)
  2. Store `admin`, `name`, `symbol` in instance storage
  3. Store `total_supply = 0_i128` in instance storage
- Add tests in `contracts/lp-token/tests/` covering the happy path and double-init panic
- Document the function with a doc comment stating what it does and what it panics on

## Additional Requirements

Follow Soroban best practices. Use `env.storage().instance()` for contract-level state. No unsafe code.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-001)
- [ ] `initialize` stores admin, name, symbol, and sets total supply to 0
- [ ] Second call panics with `"lp token already initialized"`
- [ ] `cargo test` passes for the workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass

See `docs/contracts.md` — LP Token Contract section for full storage layout and implementation details.
