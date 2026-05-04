use dex_factory::{FactoryContract, FactoryContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn deploy_client(env: &Env) -> FactoryContractClient<'_> {
    let contract_id = env.register_contract(None, FactoryContract);
    FactoryContractClient::new(env, &contract_id)
}

#[test]
fn initialize_sets_admin() {
    let env = Env::default();
    let client = deploy_client(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    assert_eq!(client.admin(), admin);
}

#[test]
#[should_panic(expected = "factory has already been initialized")]
fn initialize_rejects_second_call() {
    let env = Env::default();
    let client = deploy_client(&env);
    let admin = Address::generate(&env);
    let next_admin = Address::generate(&env);

    client.initialize(&admin);
    client.initialize(&next_admin);
}
