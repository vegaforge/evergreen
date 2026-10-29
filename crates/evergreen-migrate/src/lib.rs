//! Upgrade and storage-migration engine (stub).
//!
//! Soroban supports upgrading a contract's Wasm, but changing the shape of
//! stored data across that upgrade has no tooling. This crate fills that gap
//! with declarative migrations: describe how stored entries should change, see
//! the full diff against current ledger state before committing, then execute
//! with a recorded rollback plan.
//!
//! The dry-run is the safety property. No migration runs without producing the
//! diff first, the same discipline a database migration tool provides.
//!
//! This is a Phase 4 deliverable (PRD section 6.3, 7, 11 and 10). The types
//! here are the intended interface; the bodies return
//! [`MigrationError::NotImplemented`].

/// The storage tier an entry lives in.
///
/// Mirrors `evergreen_core::Tier`. Kept local so the engine does not pull in
/// the no_std on-chain crate; TODO(PRD 9): unify the two once the shared types
/// are factored out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Persistent,
    Temporary,
    Instance,
}

/// A single transformation applied to stored data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Transform {
    /// Move an entry from one key to another, value unchanged.
    RenameKey { from: String, to: String },
    /// Rewrite the value stored under a key into a new layout.
    RelayoutValue { key: String },
    /// Move an entry from one tier to another.
    MoveTier { key: String, from: Tier, to: Tier },
}

/// An ordered set of transformations applied as one migration.
#[derive(Clone, Debug, Default)]
pub struct Migration {
    /// Monotonic version this migration moves the contract to.
    pub to_version: u32,
    /// Transforms, applied in order.
    pub transforms: Vec<Transform>,
}

/// One concrete storage change a migration would make, for the dry-run diff.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// Human-readable before state.
    pub before: String,
    /// Human-readable after state.
    pub after: String,
}

/// The full set of changes a migration would make, produced before execution.
#[derive(Clone, Debug, Default)]
pub struct DryRunReport {
    /// Every change, in apply order.
    pub changes: Vec<Change>,
}

/// The information needed to undo a migration.
#[derive(Clone, Debug, Default)]
pub struct RollbackPlan {
    /// Inverse transforms, in reverse order.
    pub inverse: Vec<Transform>,
}

/// Errors the engine can return.
#[derive(Debug, PartialEq, Eq)]
pub enum MigrationError {
    /// Not implemented yet.
    NotImplemented,
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::NotImplemented => write!(f, "not implemented yet"),
        }
    }
}

impl std::error::Error for MigrationError {}

/// Plans and runs migrations against a contract's storage.
#[derive(Debug, Default)]
pub struct MigrationEngine;

impl MigrationEngine {
    /// New engine.
    pub fn new() -> Self {
        Self
    }

    /// Compute what a migration would change, without touching the ledger.
    ///
    /// TODO(PRD 6.3, 7): read current entries, apply each transform in memory,
    /// and emit a `Change` per affected entry.
    pub fn dry_run(&self, migration: &Migration) -> Result<DryRunReport, MigrationError> {
        let _ = migration;
        Err(MigrationError::NotImplemented)
    }

    /// Execute a migration and return the plan to undo it.
    ///
    /// TODO(PRD 6.3, 15): decide whether the storage transform shares the Wasm
    /// upgrade transaction or runs as a sequenced pair behind a guard.
    pub fn execute(&self, migration: &Migration) -> Result<RollbackPlan, MigrationError> {
        let _ = migration;
        Err(MigrationError::NotImplemented)
    }

    /// Apply a rollback plan to undo a migration.
    ///
    /// TODO(PRD 11): replay the inverse transforms in order.
    pub fn rollback(&self, plan: &RollbackPlan) -> Result<(), MigrationError> {
        let _ = plan;
        Err(MigrationError::NotImplemented)
    }
}
