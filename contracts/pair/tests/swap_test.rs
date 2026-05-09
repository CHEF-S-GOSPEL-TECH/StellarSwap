use dex_pair::{PairContract, PairContractClient};
use lp_token::LpToken;
use soroban_sdk::{
    testutils::Address as _,
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

struct TestSetup<'a> {
    env: Env,
    pair: PairContractClient<'a>,
    token_a: Address,
    token_b: Address,
    admin: Address,
}

fn setup() -> TestSetup<'static> {
    let env = Env::default();
    env.mock_all_auths();

    let token_a = env
        .register_stellar_asset_contract_v2(Address::generate(&env))
        .address();
    let token_b = env
        .register_stellar_asset_contract_v2(Address::generate(&env))
        .address();

    let lp_id = env.register_contract(None, LpToken);
    lp_token::LpTokenClient::new(&env, &lp_id).initialize(
        &env.register_contract(None, PairContract), // placeholder — overwritten below
        &soroban_sdk::String::from_str(&env, "StellarSwap LP"),
        &soroban_sdk::String::from_str(&env, "SSLP"),
    );

    let pair_id = env.register_contract(None, PairContract);
    let pair = PairContractClient::new(&env, &pair_id);
    pair.initialize(&token_a, &token_b, &lp_id);

    let admin = Address::generate(&env);
    StellarAssetClient::new(&env, &token_a).mint(&admin, &1_000_000);
    StellarAssetClient::new(&env, &token_b).mint(&admin, &1_000_000);

    TestSetup {
        env,
        pair,
        token_a,
        token_b,
        admin,
    }
}

/// Seed stored reserves directly (simulates what add_liquidity will do).
/// Transfers tokens to the pair and sets the cached reserves to match.
fn seed_reserves(s: &TestSetup, amount_a: i128, amount_b: i128) {
    TokenClient::new(&s.env, &s.token_a).transfer(&s.admin, &s.pair.address, &amount_a);
    TokenClient::new(&s.env, &s.token_b).transfer(&s.admin, &s.pair.address, &amount_b);

    s.env.as_contract(&s.pair.address, || {
        use soroban_sdk::contracttype;
        #[derive(Clone)]
        #[contracttype]
        enum DataKey {
            ReserveA,
            ReserveB,
        }
        s.env
            .storage()
            .instance()
            .set(&DataKey::ReserveA, &amount_a);
        s.env
            .storage()
            .instance()
            .set(&DataKey::ReserveB, &amount_b);
    });
}

#[test]
fn swap_a_for_b_returns_correct_amount() {
    let s = setup();
    seed_reserves(&s, 1_000_000, 1_000_000);

    let trader = Address::generate(&s.env);
    StellarAssetClient::new(&s.env, &s.token_a).mint(&trader, &100_000);
    TokenClient::new(&s.env, &s.token_a).transfer(&trader, &s.pair.address, &100_000);

    let amount_out = s.pair.swap(&trader, &s.token_a, &1);

    // get_amount_out(100_000, 1_000_000, 1_000_000) = 90_661
    assert_eq!(amount_out, 90_661);
    assert_eq!(s.pair.get_reserves(), (1_100_000, 1_000_000 - 90_661));
}

#[test]
fn swap_b_for_a_returns_correct_amount() {
    let s = setup();
    seed_reserves(&s, 1_000_000, 1_000_000);

    let trader = Address::generate(&s.env);
    StellarAssetClient::new(&s.env, &s.token_b).mint(&trader, &100_000);
    TokenClient::new(&s.env, &s.token_b).transfer(&trader, &s.pair.address, &100_000);

    assert_eq!(s.pair.swap(&trader, &s.token_b, &1), 90_661);
}

#[test]
#[should_panic(expected = "slippage: amount_out below minimum")]
fn swap_rejects_slippage() {
    let s = setup();
    seed_reserves(&s, 1_000_000, 1_000_000);

    let trader = Address::generate(&s.env);
    StellarAssetClient::new(&s.env, &s.token_a).mint(&trader, &100_000);
    TokenClient::new(&s.env, &s.token_a).transfer(&trader, &s.pair.address, &100_000);

    s.pair.swap(&trader, &s.token_a, &999_999);
}

#[test]
#[should_panic(expected = "token_in is not part of this pair")]
fn swap_rejects_unknown_token() {
    let s = setup();
    seed_reserves(&s, 1_000_000, 1_000_000);
    s.pair
        .swap(&Address::generate(&s.env), &Address::generate(&s.env), &1);
}

#[test]
#[should_panic(expected = "amount_in must be positive")]
fn swap_panics_if_nothing_sent() {
    let s = setup();
    seed_reserves(&s, 1_000_000, 1_000_000);
    // No transfer — amount_in = balance - reserve = 0
    s.pair.swap(&Address::generate(&s.env), &s.token_a, &1);
}
