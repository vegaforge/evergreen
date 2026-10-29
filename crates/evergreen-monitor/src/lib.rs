//! Off-chain archival monitor (stub).
//!
//! The on-chain library keeps frequently accessed entries alive by extending
//! their TTL on access. Entries that are written rarely can still lapse between
//! writes. This crate is the safety net: it watches a deployed contract over
//! Stellar RPC, predicts which entries are approaching archival, and extends or
//! restores them before they lapse, all within a rent budget.
//!
//! This is a Phase 2 deliverable, tracked in issues #1 to #4 on
//! <https://github.com/vegaforge/evergreen/issues>. The types here are
//! the intended interface; the bodies return [`MonitorError::NotImplemented`].

use std::time::Duration;

/// Identifies a single ledger entry the monitor tracks.
///
/// TODO(#1): replace the string with `stellar_xdr::LedgerKey` once the
/// crate takes on stellar-xdr.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryKey(pub String);

/// How often the monitor runs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cadence {
    /// A single pass, then exit. Suits a cron job.
    OneShot,
    /// Repeat on a fixed interval. Suits a long-running service.
    Interval(Duration),
}

/// What the monitor needs to watch a contract.
#[derive(Clone, Debug)]
pub struct MonitorConfig {
    /// Stellar RPC endpoint.
    pub rpc_url: String,
    /// Contract whose entries are watched.
    pub contract_id: String,
    /// Ceiling on rent the monitor may spend, in stroops. Caps total spend so
    /// the monitor cannot be tricked into unbounded extension (see SECURITY.md).
    pub rent_budget_stroops: i128,
    /// How often to run.
    pub cadence: Cadence,
}

/// A live entry and how close it is to archival.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntryStatus {
    /// The entry this describes.
    pub key: EntryKey,
    /// Ledger at which the entry is archived if not extended.
    pub live_until_ledger: u32,
    /// Ledgers remaining before archival, against the current ledger.
    pub ttl_remaining: u32,
}

/// How urgent an entry is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArchivalRisk {
    /// TTL is comfortably above the policy threshold.
    Safe,
    /// TTL is near the threshold and the entry should be extended soon.
    Approaching,
    /// The entry is already archived and needs restoring.
    Archived,
}

/// Errors the monitor can return.
#[derive(Debug, PartialEq, Eq)]
pub enum MonitorError {
    /// Not implemented yet.
    NotImplemented,
}

impl std::fmt::Display for MonitorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitorError::NotImplemented => write!(f, "not implemented yet"),
        }
    }
}

impl std::error::Error for MonitorError {}

/// Watches a contract and keeps its entries alive.
pub struct Monitor {
    config: MonitorConfig,
}

impl Monitor {
    /// Build a monitor from its config.
    pub fn new(config: MonitorConfig) -> Self {
        Self { config }
    }

    /// The config this monitor runs with.
    pub fn config(&self) -> &MonitorConfig {
        &self.config
    }

    /// Read the current TTL of every tracked entry over RPC.
    ///
    /// TODO(#1): query the entries via stellar-rpc-client and map the
    /// returned `liveUntilLedgerSeq` against the current ledger.
    pub fn scan(&self) -> Result<Vec<EntryStatus>, MonitorError> {
        Err(MonitorError::NotImplemented)
    }

    /// Classify an entry's archival risk from its status.
    ///
    /// TODO(#2): compare `ttl_remaining` against the policy threshold and
    /// the usage trend.
    pub fn predict(&self, status: &EntryStatus) -> Result<ArchivalRisk, MonitorError> {
        let _ = status;
        Err(MonitorError::NotImplemented)
    }

    /// Extend every entry that is approaching archival, within budget.
    ///
    /// TODO(#3): submit extend transactions for entries at
    /// [`ArchivalRisk::Approaching`], stopping when the rent budget is spent.
    pub fn extend_due(&self) -> Result<Vec<EntryKey>, MonitorError> {
        Err(MonitorError::NotImplemented)
    }

    /// Restore entries that have already archived.
    ///
    /// TODO(#4): prefer the Protocol 23 simulation-driven restore
    /// list, and fall back to an explicit `RestoreFootprintOp` with an empty
    /// read-only footprint.
    pub fn restore_archived(&self) -> Result<Vec<EntryKey>, MonitorError> {
        Err(MonitorError::NotImplemented)
    }
}
