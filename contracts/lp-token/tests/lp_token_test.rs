#![cfg(test)]

use lp_token::LpTokenClient;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, LpTokenClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, lp_token::LpToken);
    let client = LpTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(
        &admin,
        &String::from_str(&env, "StellarSwap LP"),
        &String::from_str(&env, "SSLP"),
    );
    (env, client, admin)
}

#[test]
fn initialize_stores_metadata() {
    let (env, client, _) = setup();
    assert_eq!(client.name(), String::from_str(&env, "StellarSwap LP"));
    assert_eq!(client.symbol(), String::from_str(&env, "SSLP"));
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.total_supply(), 0);
}

#[test]
#[should_panic(expected = "lp token already initialized")]
fn initialize_rejects_second_call() {
    let (env, client, admin) = setup();
    client.initialize(
        &admin,
        &String::from_str(&env, "x"),
        &String::from_str(&env, "x"),
    );
}

#[test]
fn mint_increases_balance_and_supply() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.mint(&user, &1000);
    assert_eq!(client.balance(&user), 1000);
    assert_eq!(client.total_supply(), 1000);
}

#[test]
fn burn_decreases_balance_and_supply() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.mint(&user, &1000);
    client.burn(&user, &400);
    assert_eq!(client.balance(&user), 600);
    assert_eq!(client.total_supply(), 600);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn burn_panics_if_insufficient_balance() {
    let (env, client, _) = setup();
    let user = Address::generate(&env);
    client.mint(&user, &100);
    client.burn(&user, &200);
}

#[test]
fn transfer_moves_tokens() {
    let (env, client, _) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    client.mint(&alice, &1000);
    client.transfer(&alice, &bob, &300);
    assert_eq!(client.balance(&alice), 700);
    assert_eq!(client.balance(&bob), 300);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_panics_if_insufficient_balance() {
    let (env, client, _) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    client.mint(&alice, &100);
    client.transfer(&alice, &bob, &200);
}

#[test]
fn transfer_from_uses_allowance() {
    let (env, client, _) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let spender = Address::generate(&env);
    client.mint(&alice, &1000);
    client.approve(&alice, &spender, &500, &0);
    client.transfer_from(&spender, &alice, &bob, &300);
    assert_eq!(client.balance(&alice), 700);
    assert_eq!(client.balance(&bob), 300);
    assert_eq!(client.allowance(&alice, &spender), 200);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_panics_if_insufficient_allowance() {
    let (env, client, _) = setup();
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let spender = Address::generate(&env);
    client.mint(&alice, &1000);
    client.approve(&alice, &spender, &100, &0);
    client.transfer_from(&spender, &alice, &bob, &200);
}

#[test]
fn approve_and_allowance() {
    let (env, client, _) = setup();
    let alice = Address::generate(&env);
    let spender = Address::generate(&env);
    assert_eq!(client.allowance(&alice, &spender), 0);
    client.approve(&alice, &spender, &750, &0);
    assert_eq!(client.allowance(&alice, &spender), 750);
}
