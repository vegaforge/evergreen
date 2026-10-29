//! Evergreen command line tool (stub).
//!
//! The command surface is real and wired to the backing crates. The features
//! behind each command are not implemented yet; every handler reports what it
//! will do and which PRD section covers it. See PRD section 6.3 and 10.

use clap::{Args, Parser, Subcommand};

use evergreen_migrate::{Migration, MigrationEngine};
use evergreen_monitor::{Cadence, Monitor, MonitorConfig};

#[derive(Parser)]
#[command(name = "evergreen", version, about = "Keep Soroban storage evergreen.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List entries with TTL, time to archival, and projected cost.
    Inspect(Target),
    /// Extend entries approaching archival.
    Extend(Target),
    /// Restore archived entries.
    Restore(Target),
    /// Project rent cost over a horizon.
    Forecast(Target),
    /// Upgrade a contract's Wasm.
    Upgrade {
        /// Hex hash of the new Wasm.
        #[arg(long)]
        wasm_hash: String,
    },
    /// Run a declarative storage migration. Dry-run first.
    Migrate {
        /// Version to migrate the contract to.
        #[arg(long)]
        to_version: u32,
    },
}

/// A contract to act on, reached over RPC.
#[derive(Args)]
struct Target {
    /// Stellar RPC endpoint.
    #[arg(long)]
    rpc: String,
    /// Contract id.
    #[arg(long)]
    contract: String,
}

fn main() {
    match Cli::parse().command {
        Command::Inspect(t) => inspect(&t),
        Command::Extend(t) => extend(&t),
        Command::Restore(t) => restore(&t),
        Command::Forecast(t) => forecast(&t),
        Command::Upgrade { wasm_hash } => upgrade(&wasm_hash),
        Command::Migrate { to_version } => migrate(to_version),
    }
}

fn monitor(target: &Target) -> Monitor {
    Monitor::new(MonitorConfig {
        rpc_url: target.rpc.clone(),
        contract_id: target.contract.clone(),
        rent_budget_stroops: 0,
        cadence: Cadence::OneShot,
    })
}

fn inspect(target: &Target) {
    match monitor(target).scan() {
        Ok(entries) => {
            for entry in entries {
                println!("{entry:?}");
            }
        }
        Err(e) => eprintln!("inspect is pending ({e}); see PRD 6.4 and 7.4"),
    }
}

fn extend(target: &Target) {
    match monitor(target).extend_due() {
        Ok(keys) => println!("extended {} entries", keys.len()),
        Err(e) => eprintln!("extend is pending ({e}); see PRD 6.2 and 7.2"),
    }
}

fn restore(target: &Target) {
    match monitor(target).restore_archived() {
        Ok(keys) => println!("restored {} entries", keys.len()),
        Err(e) => eprintln!("restore is pending ({e}); see PRD 6.2 and 7.3"),
    }
}

fn forecast(target: &Target) {
    let _ = monitor(target);
    eprintln!("forecast is pending; see PRD 7.4");
}

fn upgrade(wasm_hash: &str) {
    let _ = wasm_hash;
    eprintln!("upgrade is pending; see PRD 6.3");
}

fn migrate(to_version: u32) {
    let migration = Migration {
        to_version,
        transforms: Vec::new(),
    };
    match MigrationEngine::new().dry_run(&migration) {
        Ok(report) => println!("dry-run: {} changes", report.changes.len()),
        Err(e) => eprintln!("migrate dry-run is pending ({e}); see PRD 6.3"),
    }
}
