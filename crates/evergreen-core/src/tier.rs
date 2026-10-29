/// The three Soroban storage tiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tier {
    /// Survives archival and can be restored. Use for balances and long-lived state.
    Persistent,
    /// Cheapest tier. Gone for good once its TTL lapses.
    Temporary,
    /// Shares the contract instance TTL. Use for small global state like an admin.
    Instance,
}

impl Tier {
    /// Whether an entry in this tier can be restored after archival.
    pub const fn is_restorable(self) -> bool {
        match self {
            Tier::Persistent | Tier::Instance => true,
            Tier::Temporary => false,
        }
    }
}
