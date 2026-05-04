//! Constant-Product Math
//!
//! Implements the core AMM pricing formula: x * y = k
//!
//! ## Key formulas:
//!
//! ### Output amount (with fee):
//!   amount_out = (amount_in_with_fee * reserve_out) / (reserve_in * 1000 + amount_in_with_fee)
//!   where amount_in_with_fee = amount_in * 997  (i.e. 0.3% fee)
//!
//! ### LP tokens to mint on first deposit:
//!   lp_minted = sqrt(amount_a * amount_b) - MINIMUM_LIQUIDITY
//!
//! ### LP tokens to mint on subsequent deposits:
//!   lp_minted = min(
//!       (amount_a / reserve_a) * lp_supply,
//!       (amount_b / reserve_b) * lp_supply,
//!   )

/// Permanently locked liquidity on the first deposit.
///
/// This mirrors the Uniswap V2 design decision that prevents the total LP
/// supply from ever reaching zero and reduces rounding-error edge cases.
pub const MINIMUM_LIQUIDITY: i128 = 1_000;

/// Given an input amount and current reserves, returns the output amount after fee.
/// Panics if reserves are zero or amount_in is zero.
pub fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    assert!(amount_in > 0, "amount_in must be positive");
    assert!(reserve_in > 0, "reserve_in must be positive");
    assert!(reserve_out > 0, "reserve_out must be positive");

    let amount_in_with_fee = amount_in * 997;
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * 1000 + amount_in_with_fee;

    numerator / denominator
}

/// Calculate LP tokens to mint for a given deposit.
/// `lp_supply` is the current total supply of LP tokens (0 on first deposit).
pub fn calc_lp_tokens_to_mint(
    amount_a: i128,
    amount_b: i128,
    reserve_a: i128,
    reserve_b: i128,
    lp_supply: i128,
) -> i128 {
    assert!(amount_a > 0, "amount_a must be positive");
    assert!(amount_b > 0, "amount_b must be positive");

    if lp_supply == 0 {
        // Use checked_mul to guard against overflow on very large initial deposits.
        let product = amount_a
            .checked_mul(amount_b)
            .expect("initial deposit product overflows i128");
        let initial_liquidity = sqrt(product);
        assert!(
            initial_liquidity > MINIMUM_LIQUIDITY,
            "insufficient initial liquidity"
        );

        return initial_liquidity - MINIMUM_LIQUIDITY;
    }

    assert!(reserve_a > 0, "reserve_a must be positive");
    assert!(reserve_b > 0, "reserve_b must be positive");

    let mint_from_a = amount_a * lp_supply / reserve_a;
    let mint_from_b = amount_b * lp_supply / reserve_b;

    if mint_from_a < mint_from_b {
        mint_from_a
    } else {
        mint_from_b
    }
}

/// Verifies the post-swap constant-product invariant (whitepaper eq. 11).
///
/// After a swap, the pair reads its actual new balances and asserts:
///   (1000·balance_a − 3·amount_in_a) · (1000·balance_b − 3·amount_in_b) >= 1_000_000 · reserve_a · reserve_b
///
/// `amount_in_a` / `amount_in_b` are the observed inflows (balance delta minus any outflow).
/// Pass 0 for the token that was not the input.
/// Panics if the invariant is violated (i.e. k decreased after fees).
pub fn check_invariant(
    balance_a: i128,
    balance_b: i128,
    amount_in_a: i128,
    amount_in_b: i128,
    reserve_a: i128,
    reserve_b: i128,
) {
    let adjusted_a = balance_a * 1000 - amount_in_a * 3;
    let adjusted_b = balance_b * 1000 - amount_in_b * 3;
    assert!(
        adjusted_a.checked_mul(adjusted_b).expect("invariant check overflow")
            >= reserve_a.checked_mul(reserve_b * 1_000_000).expect("invariant check overflow"),
        "invariant violated: k decreased after swap"
    );
}

/// Integer square root (Babylonian method).
pub fn sqrt(n: i128) -> i128 {
    assert!(n >= 0, "sqrt input must be non-negative");

    if n < 2 {
        return n;
    }

    let mut x = n;
    let mut y = (x + n / x) / 2;

    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }

    x
}
