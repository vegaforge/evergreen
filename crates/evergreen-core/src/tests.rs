use soroban_sdk::testutils::storage::Persistent as _;
use soroban_sdk::testutils::Ledger as _;
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

use crate::{EvergreenError, InstanceStore, PersistentStore, Tier, TtlPolicy};

// A bare contract so the stores have a contract context to run in.
#[contract]
struct Harness;

#[contractimpl]
impl Harness {}

// Threshold sits above the default minimum entry TTL, so the first write always
// extends and gives us something to assert on.
const THRESHOLD: u32 = 100_000;
const EXTEND_TO: u32 = 1_000_000;

const POLICY: TtlPolicy = match TtlPolicy::new(THRESHOLD, EXTEND_TO) {
    Ok(policy) => policy,
    Err(_) => panic!("test policy constants are invalid"),
};

const KEY: Symbol = symbol_short!("k");

#[test]
fn new_rejects_zero_target() {
    assert_eq!(TtlPolicy::new(10, 0), Err(EvergreenError::InvalidPolicy));
}

#[test]
fn new_rejects_threshold_above_target() {
    assert_eq!(TtlPolicy::new(500, 100), Err(EvergreenError::InvalidPolicy));
}

#[test]
fn resolve_clamps_target_to_network_max() {
    let env = Env::default();
    let id = env.register(Harness, ());
    env.as_contract(&id, || {
        let max = env.storage().max_ttl();
        // Ask for far more than the network allows.
        let policy = TtlPolicy::new(10, max.saturating_add(1_000_000)).unwrap();
        let resolved = policy.resolve(&env);
        assert_eq!(resolved.extend_to, max);
        assert!(resolved.threshold <= resolved.extend_to);
    });
}

#[test]
fn persistent_set_extends_ttl_to_target() {
    let env = Env::default();
    let id = env.register(Harness, ());
    env.as_contract(&id, || {
        let store = PersistentStore::new(&env, POLICY);
        assert_eq!(store.tier(), Tier::Persistent);

        store.set(&KEY, &123i128);

        let value: Option<i128> = store.get(&KEY);
        assert_eq!(value, Some(123));

        let max = env.storage().max_ttl();
        let ttl = env.storage().persistent().get_ttl(&KEY);
        assert_eq!(ttl, EXTEND_TO.min(max));
    });
}

#[test]
fn persistent_read_keeps_entry_alive() {
    let env = Env::default();
    let id = env.register(Harness, ());
    env.as_contract(&id, || {
        let store = PersistentStore::new(&env, POLICY);
        store.set(&KEY, &7i128);

        let max = env.storage().max_ttl();
        let target = EXTEND_TO.min(max);

        // Move forward until the entry is close to archival but still alive.
        let start = env.ledger().sequence();
        env.ledger().set_sequence_number(start + target - 50);

        // Remaining TTL is now far below the threshold.
        assert!(env.storage().persistent().get_ttl(&KEY) < THRESHOLD);

        // A read through the store should extend it back to the target.
        let value: Option<i128> = store.get(&KEY);
        assert_eq!(value, Some(7));
        assert_eq!(env.storage().persistent().get_ttl(&KEY), target);
    });
}

#[test]
fn instance_store_round_trips_value() {
    let env = Env::default();
    let id = env.register(Harness, ());
    env.as_contract(&id, || {
        let store = InstanceStore::new(&env, POLICY);
        assert_eq!(store.tier(), Tier::Instance);

        assert!(!store.has(&KEY));
        store.set(&KEY, &42u32);
        assert!(store.has(&KEY));

        let value: Option<u32> = store.get(&KEY);
        assert_eq!(value, Some(42));

        store.remove(&KEY);
        assert!(!store.has(&KEY));
    });
}

#[test]
fn tier_restorability_matches_soroban_rules() {
    assert!(Tier::Persistent.is_restorable());
    assert!(Tier::Instance.is_restorable());
    assert!(!Tier::Temporary.is_restorable());
}
