**Gospel Dex — Contract (Soroban / Stellar)**

Implement the SEP-41 read-only functions on the LP token contract (`contracts/lp-token/src/lib.rs`). These are the standard token interface reads: `name`, `symbol`, `total_supply`, `balance`, and `allowance`.

Depends on GD-LP-001 (`initialize`) being merged first.

## Tasks

- Implement in `contracts/lp-token/src/lib.rs`:
  - `name(env) -> String` — read from `env.storage().instance()`
  - `symbol(env) -> String` — read from `env.storage().instance()`
  - `decimals(_env) -> u32` — always return `7` (already stubbed)
  - `total_supply(env) -> i128` — read from `env.storage().instance()`
  - `balance(env, id: Address) -> i128` — read from `env.storage().persistent()`, default 0
  - `allowance(env, from: Address, spender: Address) -> i128` — read from `env.storage().persistent()`, default 0
- Add tests covering all six functions

## Additional Requirements

Use `env.storage().instance()` for contract-level state. Use `env.storage().persistent()` for per-account state (`balance`, `allowance`) — these entries must survive ledger expiry. No unsafe code.

## Acceptance Criteria

- [ ] PR references Gospel Dex and this issue id (GD-LP-002)
- [ ] All six functions return correct values after `initialize`
- [ ] `balance` and `allowance` return 0 for unset accounts
- [ ] `cargo test` passes for the workspace
- [ ] `cargo fmt --check` and `cargo clippy` pass

See `docs/contracts.md` — LP Token Contract section for full storage layout.
