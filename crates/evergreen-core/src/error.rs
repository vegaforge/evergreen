use soroban_sdk::contracterror;

/// Errors Evergreen returns from a contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EvergreenError {
    /// Policy values cannot produce a valid extension: a zero target, or a
    /// threshold above the target.
    InvalidPolicy = 1,
}
