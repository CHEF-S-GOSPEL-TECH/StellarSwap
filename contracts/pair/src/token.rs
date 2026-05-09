use soroban_sdk::{contractclient, token::Client as TokenClient, Address, Env};

/// Minimal client for LP token admin functions not in the standard SEP-41 interface.
#[contractclient(name = "LpTokenClient")]
pub trait LpTokenTrait {
    fn mint(env: Env, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn total_supply(env: Env) -> i128;
}

pub fn transfer(env: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    TokenClient::new(env, token).transfer(from, to, &amount);
}

pub fn balance(env: &Env, token: &Address, account: &Address) -> i128 {
    TokenClient::new(env, token).balance(account)
}

pub fn mint(env: &Env, lp_token: &Address, to: &Address, amount: i128) {
    LpTokenClient::new(env, lp_token).mint(to, &amount);
}

pub fn burn(env: &Env, lp_token: &Address, from: &Address, amount: i128) {
    LpTokenClient::new(env, lp_token).burn(from, &amount);
}

pub fn total_supply(env: &Env, lp_token: &Address) -> i128 {
    LpTokenClient::new(env, lp_token).total_supply()
}
