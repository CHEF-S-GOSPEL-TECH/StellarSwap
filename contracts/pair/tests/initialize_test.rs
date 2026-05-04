use dex_pair::{PairContract, PairContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn deploy_client(env: &Env) -> PairContractClient<'_> {
    let contract_id = env.register_contract(None, PairContract);
    PairContractClient::new(env, &contract_id)
}

#[test]
fn initialize_sets_public_reserves_to_zero() {
    let env = Env::default();
    let client = deploy_client(&env);
    let token_a = Address::generate(&env);
    let token_b = Address::generate(&env);
    let lp_token = Address::generate(&env);

    client.initialize(&token_a, &token_b, &lp_token);

    assert_eq!(client.get_reserves(), (0, 0));
}

#[test]
#[should_panic(expected = "pair tokens must be different")]
fn initialize_rejects_same_token_pair() {
    let env = Env::default();
    let client = deploy_client(&env);
    let token = Address::generate(&env);
    let lp_token = Address::generate(&env);

    client.initialize(&token, &token, &lp_token);
}

#[test]
#[should_panic(expected = "pair has already been initialized")]
fn initialize_rejects_second_call() {
    let env = Env::default();
    let client = deploy_client(&env);
    let token_a = Address::generate(&env);
    let token_b = Address::generate(&env);
    let lp_token = Address::generate(&env);
    let next_token_a = Address::generate(&env);
    let next_token_b = Address::generate(&env);
    let next_lp_token = Address::generate(&env);

    client.initialize(&token_a, &token_b, &lp_token);
    client.initialize(&next_token_a, &next_token_b, &next_lp_token);
}

#[test]
fn get_quote_returns_none_on_empty_reserves() {
    let env = Env::default();
    let client = deploy_client(&env);
    let token_a = Address::generate(&env);
    let token_b = Address::generate(&env);
    let lp_token = Address::generate(&env);

    client.initialize(&token_a, &token_b, &lp_token);
    assert_eq!(client.get_quote(&token_a, &1_000), None);
}

#[test]
#[should_panic(expected = "token_in is not part of this pair")]
fn get_quote_rejects_unknown_token() {
    let env = Env::default();
    let client = deploy_client(&env);
    let token_a = Address::generate(&env);
    let token_b = Address::generate(&env);
    let lp_token = Address::generate(&env);
    let unknown = Address::generate(&env);

    client.initialize(&token_a, &token_b, &lp_token);
    client.get_quote(&unknown, &1_000);
}
