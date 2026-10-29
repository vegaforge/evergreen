//! Tier-misuse lints (stub).
//!
//! The sharpest mistake in Soroban storage is putting durable data, such as a
//! balance, in temporary storage, where it vanishes for good once its TTL
//! lapses. These lints turn that from a silent bug into a flagged, visible
//! choice that shows up in code review.
//!
//! This is a Phase 1 deliverable (PRD section 6.1, 7.1 and 10). The types here
//! are the intended interface; [`TierLint::check`] is not implemented yet.

/// How serious a finding is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// Worth a look, not necessarily wrong.
    Warning,
    /// Almost certainly a bug, such as durable data in temporary storage.
    Error,
}

/// One thing a lint flagged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// The rule that fired.
    pub rule: String,
    /// What is wrong and where.
    pub message: String,
    /// How serious it is.
    pub severity: Severity,
}

/// Errors the linter can return.
#[derive(Debug, PartialEq, Eq)]
pub enum LintError {
    /// Not implemented yet.
    NotImplemented,
}

impl std::fmt::Display for LintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintError::NotImplemented => write!(f, "not implemented yet"),
        }
    }
}

impl std::error::Error for LintError {}

/// Checks a contract's source for tier misuse.
pub trait TierLint {
    /// Run the lint over the source at `path` and return what it found.
    ///
    /// TODO(PRD 7.1): parse the source, find storage writes, and flag values
    /// classified as durable that route to the temporary tier without an
    /// explicit override.
    fn check(&self, path: &str) -> Result<Vec<Finding>, LintError>;
}

/// The default lint, flagging durable data written to temporary storage.
#[derive(Debug, Default)]
pub struct DurableInTemporary;

impl TierLint for DurableInTemporary {
    fn check(&self, path: &str) -> Result<Vec<Finding>, LintError> {
        let _ = path;
        Err(LintError::NotImplemented)
    }
}
