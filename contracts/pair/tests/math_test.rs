use dex_pair::math::{
    calc_lp_tokens_to_mint, check_invariant, get_amount_out, sqrt, MINIMUM_LIQUIDITY,
};

#[test]
fn amount_out_applies_constant_product_fee() {
    assert_eq!(get_amount_out(1_000, 10_000, 10_000), 906);
}

#[test]
fn first_lp_deposit_mints_geometric_mean_minus_minimum_liquidity() {
    assert_eq!(
        calc_lp_tokens_to_mint(10_000, 40_000, 0, 0, 0),
        20_000 - MINIMUM_LIQUIDITY
    );
}

#[test]
#[should_panic(expected = "insufficient initial liquidity")]
fn first_lp_deposit_must_exceed_minimum_liquidity() {
    calc_lp_tokens_to_mint(100, 400, 0, 0, 0);
}

#[test]
fn later_lp_deposit_mints_min_proportional_share() {
    assert_eq!(calc_lp_tokens_to_mint(50, 120, 100, 200, 1_000), 500);
}

#[test]
fn sqrt_rounds_down_to_integer_root() {
    assert_eq!(sqrt(0), 0);
    assert_eq!(sqrt(1), 1);
    assert_eq!(sqrt(15), 3);
    assert_eq!(sqrt(16), 4);
}

#[test]
fn check_invariant_passes_after_valid_swap() {
    // reserves: 10_000 A, 10_000 B. swap 1_000 A in → 906 B out (from amount_out test above).
    // new balances: 11_000 A, 9_094 B. amount_in_a = 1_000, amount_in_b = 0.
    check_invariant(11_000, 9_094, 1_000, 0, 10_000, 10_000);
}

#[test]
#[should_panic(expected = "invariant violated")]
fn check_invariant_fails_if_k_decreases() {
    // Simulate a bad swap that takes too much out without enough in.
    check_invariant(10_100, 9_000, 100, 0, 10_000, 10_000);
}
