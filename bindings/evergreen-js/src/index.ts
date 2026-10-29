/**
 * TypeScript bindings for Evergreen (stub).
 *
 * A thin surface so teams whose operational tooling is Node-based can drive the
 * monitor and CLI without adopting Rust on the ops side. This mirrors the Rust
 * `evergreen-monitor` interface. The methods are not implemented yet; they will
 * call through to the Rust core or to js-stellar-sdk for the read and submit
 * paths. See PRD section 6.4 and 8.
 */

/** How often the monitor runs. */
export type Cadence =
  | { kind: "one-shot" }
  | { kind: "interval"; everyMs: number };

/** What the monitor needs to watch a contract. */
export interface MonitorConfig {
  /** Stellar RPC endpoint. */
  rpcUrl: string;
  /** Contract whose entries are watched. */
  contractId: string;
  /** Ceiling on rent the monitor may spend, in stroops. */
  rentBudgetStroops: bigint;
  /** How often to run. */
  cadence: Cadence;
}

/** A live entry and how close it is to archival. */
export interface EntryStatus {
  /** Identifier of the entry. TODO(PRD 6.2): use the XDR LedgerKey form. */
  key: string;
  /** Ledger at which the entry is archived if not extended. */
  liveUntilLedger: number;
  /** Ledgers remaining before archival. */
  ttlRemaining: number;
}

/** How urgent an entry is. */
export enum ArchivalRisk {
  Safe = "safe",
  Approaching = "approaching",
  Archived = "archived",
}

/** Thrown by every stub method until the bindings are implemented. */
export class NotImplementedError extends Error {
  constructor(what: string) {
    super(`${what} is not implemented yet`);
    this.name = "NotImplementedError";
  }
}

/** Watches a contract and keeps its entries alive. */
export class Monitor {
  constructor(public readonly config: MonitorConfig) {}

  /** Read the current TTL of every tracked entry over RPC. */
  async scan(): Promise<EntryStatus[]> {
    // TODO(PRD 6.2): query entries via js-stellar-sdk and map liveUntilLedgerSeq.
    throw new NotImplementedError("scan");
  }

  /** Classify an entry's archival risk from its status. */
  predict(status: EntryStatus): ArchivalRisk {
    // TODO(PRD 6.2): compare ttlRemaining against the policy threshold.
    void status;
    throw new NotImplementedError("predict");
  }

  /** Extend every entry that is approaching archival, within budget. */
  async extendDue(): Promise<string[]> {
    // TODO(PRD 6.2, 7.2): submit extend transactions until the budget is spent.
    throw new NotImplementedError("extendDue");
  }

  /** Restore entries that have already archived. */
  async restoreArchived(): Promise<string[]> {
    // TODO(PRD 6.2, 7.3): use the Protocol 23 restore list, then RestoreFootprintOp.
    throw new NotImplementedError("restoreArchived");
  }
}
