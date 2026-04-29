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
//!   lp_minted = sqrt(amount_a * amount_b)
//!
//! ### LP tokens to mint on subsequent deposits:
//!   lp_minted = min(
//!       (amount_a / reserve_a) * lp_supply,
//!       (amount_b / reserve_b) * lp_supply,
//!   )

#![allow(unused_variables, dead_code)]

/// Given an input amount and current reserves, returns the output amount after fee.
/// Panics if reserves are zero or amount_in is zero.
pub fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    // TODO: implement constant-product formula with 0.3% fee
    todo!("implement get_amount_out")
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
    // TODO: first deposit → sqrt(amount_a * amount_b)
    //       subsequent    → min proportional share
    todo!("implement calc_lp_tokens_to_mint")
}

/// Integer square root (Babylonian method).
pub fn sqrt(n: i128) -> i128 {
    // TODO: implement integer sqrt
    todo!("implement sqrt")
}
