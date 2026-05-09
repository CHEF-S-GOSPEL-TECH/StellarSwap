# Changelog

All notable changes to this project will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased]

### Added
- `contracts/lp-token`: full SEP-41 LP token implementation — `initialize`, `transfer`, `transfer_from`, `approve`, `allowance`, `mint` (admin-only), `burn` (admin-only) with 10 passing tests
- `contracts/pair/src/token.rs`: cross-contract call helpers — `transfer`, `balance`, `mint`, `burn`, `total_supply` using `soroban_sdk::token::Client` and inline `contractclient` trait
- `contracts/pair/src/pair.rs`: `swap` execution — balance delta model, 0.3% fee, slippage guard, post-swap invariant check, reserve update with 5 passing tests
- `docs/plan.md`: full build plan, roadmap (v1/v2/v3), maintainer vs contributor split, issue map
- `docs/implementation-log.md`: detailed record of every implementation decision and how each layer works
- CI: `develop` branch added to push trigger

### Changed
- README updated to reflect StellarSwap name, 29 passing tests, and current implementation status
- `contributing.md` updated to reflect completed layers and current open work
- `CHANGELOG.md` updated

---

## [0.1.0] — 2026-05-04

### Added
- Project scaffolded: AMM contract, LP token contract, TypeScript SDK, deploy scripts, integration test stubs
- `contracts/pair/src/math.rs`: `get_amount_out`, `calc_lp_tokens_to_mint`, `check_invariant`, `sqrt`, `MINIMUM_LIQUIDITY` — 7 tests
- `contracts/pair/src/pair.rs`: `initialize`, `get_reserves`, `get_quote` — 5 tests
- `contracts/factory/src/lib.rs`: `initialize`, `admin`, token sorting — 2 tests
- Architecture, contract reference, and contributing docs
- GitHub Actions CI (build, test, clippy, fmt)
