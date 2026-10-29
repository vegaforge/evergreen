use soroban_sdk::Env;

use crate::error::EvergreenError;

/// When and how far to extend an entry's TTL.
///
/// Maps onto the host call `extend_ttl(threshold, extend_to)`: once an entry's
/// remaining TTL is at or below `extend_threshold`, extend it to `extend_to`.
/// Both are in ledgers, about five seconds each.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TtlPolicy {
    extend_threshold: u32,
    extend_to: u32,
}

impl TtlPolicy {
    /// Build a policy, rejecting values that cannot extend anything.
    ///
    /// `const` so policies can be declared as constants and checked at compile
    /// time:
    ///
    /// ```
    /// use evergreen_core::TtlPolicy;
    ///
    /// const BALANCE: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    ///     Ok(p) => p,
    ///     Err(_) => panic!("policy constants are invalid"),
    /// };
    /// ```
    pub const fn new(extend_threshold: u32, extend_to: u32) -> Result<Self, EvergreenError> {
        if extend_to == 0 {
            return Err(EvergreenError::InvalidPolicy);
        }
        // A threshold above the target can never trigger, so it is a mistake.
        if extend_threshold > extend_to {
            return Err(EvergreenError::InvalidPolicy);
        }
        Ok(Self {
            extend_threshold,
            extend_to,
        })
    }

    /// Extend once remaining TTL is at or below this many ledgers.
    pub const fn extend_threshold(&self) -> u32 {
        self.extend_threshold
    }

    /// Target TTL to extend to.
    pub const fn extend_to(&self) -> u32 {
        self.extend_to
    }

    /// Clamp to the network's current max TTL, read live from the ledger.
    ///
    /// The max is a network parameter, so it is read with
    /// `env.storage().max_ttl()` on every call rather than hardcoded.
    ///
    /// The host already caps persistent and instance extensions at the max on
    /// its own, so for those tiers a target past it is not an error. The host
    /// does return an error in two cases: a temporary entry extended past the
    /// max, and a threshold above the target. `resolve` caps the target first
    /// and then keeps the threshold at or below it, so the host call it feeds
    /// is valid for every tier, and the same policy stays safe for a temporary
    /// store.
    pub fn resolve(&self, env: &Env) -> ResolvedTtl {
        let max = env.storage().max_ttl();
        let extend_to = min_u32(self.extend_to, max);
        let threshold = min_u32(self.extend_threshold, extend_to);
        ResolvedTtl {
            threshold,
            extend_to,
        }
    }
}

/// A policy clamped to the live network limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedTtl {
    /// Extend when remaining TTL is at or below this many ledgers.
    pub threshold: u32,
    /// Extend so the entry lives for this many more ledgers.
    pub extend_to: u32,
}

const fn min_u32(a: u32, b: u32) -> u32 {
    if a < b {
        a
    } else {
        b
    }
}
