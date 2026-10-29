use soroban_sdk::testutils::storage::Persistent as _;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{Address, Env};

use crate::{DataKey, Token, TokenClient, TokenError};

// The policy target balances land on, in ledgers (~30 days).
const TARGET: u32 = 518_400;
// The policy threshold (~1 day).
const THRESHOLD: u32 = 17_280;

fn admin_and_client(env: &Env) -> (Address, Address) {
    let admin = Address::generate(env);
    let id = env.register(Token, (admin.clone(),));
    (admin, id)
}

#[test]
fn mint_then_read_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, id) = admin_and_client(&env);
    let client = TokenClient::new(&env, &id);
    let user = Address::generate(&env);

    client.mint(&user, &1_000);
    assert_eq!(client.balance(&user), 1_000);
}

#[test]
fn transfer_moves_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, id) = admin_and_client(&env);
    let client = TokenClient::new(&env, &id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    client.mint(&alice, &1_000);
    client.transfer(&alice, &bob, &400);

    assert_eq!(client.balance(&alice), 600);
    assert_eq!(client.balance(&bob), 400);
}

#[test]
fn transfer_rejects_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, id) = admin_and_client(&env);
    let client = TokenClient::new(&env, &id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let result = client.try_transfer(&alice, &bob, &100);
    assert_eq!(result, Err(Ok(TokenError::InsufficientBalance)));
}

#[test]
fn mint_rejects_non_positive_amount() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, id) = admin_and_client(&env);
    let client = TokenClient::new(&env, &id);
    let user = Address::generate(&env);

    assert_eq!(
        client.try_mint(&user, &0),
        Err(Ok(TokenError::InvalidAmount))
    );
}

#[test]
fn balances_stay_alive_through_reads() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, id) = admin_and_client(&env);
    let client = TokenClient::new(&env, &id);
    let user = Address::generate(&env);

    client.mint(&user, &500);

    let key = DataKey::Balance(user.clone());
    let target = env.as_contract(&id, || TARGET.min(env.storage().max_ttl()));

    // The mint left the balance at the policy target.
    let ttl_after_mint = env.as_contract(&id, || env.storage().persistent().get_ttl(&key));
    assert_eq!(ttl_after_mint, target);

    // Move forward until the balance is nearly archived.
    let start = env.ledger().sequence();
    env.ledger().set_sequence_number(start + target - 100);
    let ttl_before_read = env.as_contract(&id, || env.storage().persistent().get_ttl(&key));
    assert!(ttl_before_read < THRESHOLD);

    // A plain balance read brings it back, with no extend_ttl call in the contract.
    let _ = client.balance(&user);
    let ttl_after_read = env.as_contract(&id, || env.storage().persistent().get_ttl(&key));
    assert_eq!(ttl_after_read, target);
}
