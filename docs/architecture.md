# Architecture

## Overview

This is a minimal constant-product AMM (Automated Market Maker) built on Stellar using Soroban smart contracts. It is inspired by Uniswap v2 but adapted for the Stellar ecosystem.

---

## Contracts

### AMM Contract (`contracts/amm`)

The core contract. It holds reserves of two tokens and enforces the invariant:

```
reserve_a * reserve_b = k
```

Every swap, liquidity deposit, and withdrawal must keep `k` constant (or increase it via fees).

**Key functions:**

| Function           | Description                                              |
|--------------------|----------------------------------------------------------|
| `initialize`       | Set up the pool with two token addresses + LP token      |
| `add_liquidity`    | Deposit both tokens, receive LP tokens                   |
| `remove_liquidity` | Burn LP tokens, receive proportional share of reserves   |
| `swap`             | Trade one token for the other                            |
| `get_reserves`     | Read current reserve balances                            |
| `get_quote`        | Simulate a swap without executing it                     |

### LP Token Contract (`contracts/lp-token`)

A SEP-41 compliant fungible token. Represents a liquidity provider's share of the pool.

- Minted by the AMM when liquidity is added
- Burned by the AMM when liquidity is removed
- Freely transferable between accounts

---

## Pricing Formula

### Swap output (with 0.3% fee):

```
amount_in_with_fee = amount_in * 997
amount_out = (amount_in_with_fee * reserve_out) / (reserve_in * 1000 + amount_in_with_fee)
```

The 0.3% fee stays in the pool, increasing `k` slightly with each trade. This accrues to LPs proportionally.

### LP tokens on first deposit:

```
lp_minted = sqrt(amount_a * amount_b)
```

### LP tokens on subsequent deposits:

```
lp_minted = min(
    amount_a / reserve_a * lp_supply,
    amount_b / reserve_b * lp_supply
)
```

---

## Token Flow

```
Add Liquidity:
  User ──[token_a, token_b]──► AMM Contract
  AMM  ──[LP tokens]─────────► User

Swap:
  User ──[token_in]──► AMM Contract
  AMM  ──[token_out]──► User

Remove Liquidity:
  User ──[LP tokens]──────────► AMM Contract (burns them)
  AMM  ──[token_a, token_b]──► User
```

---

## Stellar-Specific Notes

- **XLM** is Stellar's native asset. It can be wrapped as a SEP-41 token to be used as one side of a pair.
- **Soroban** contracts are written in Rust and compiled to WASM.
- **Authorization** uses Soroban's `require_auth()` — users sign transactions that authorize the contract to move their tokens.
- There is no concept of `msg.sender` like in Solidity; auth is explicit.
