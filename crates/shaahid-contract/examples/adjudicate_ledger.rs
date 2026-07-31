//! Composition smoke: a minimal idempotency-gate consumer.
//!
//! This example is the first real consumer of the adjudication contract. It holds its own
//! witnessed ledger, presents a stream of deeds, and turns each [`Outcome`] into an action
//! — **record** a clean create, **deduplicate** a clean attach, **quarantine** anything
//! contradictory — using only the public API. It exists to prove two things (see the
//! crate's `BACKLOG.md`):
//!
//! - **The seam composes:** an idempotency gate is buildable over the shipped contract
//!   without reaching inside it.
//! - **The core owes the consumer no extra read:** disposition needs exactly the shipped
//!   `attestation` match and `contradictions` list. `Outcome` carries a single
//!   `contradictions` collection, so `contradictions.is_empty()` has no compound-read trap
//!   that a wrapper would need to close — the read stays the consumer's one-liner.
//!
//! Admission is the consumer's: the core witnesses and alarms, and this consumer decides
//! what enters its ledger. The ledger here is the consumer's, never the core's.
//!
//! To have teeth, the stub domain drives four trajectories in one run: a fresh `Create`,
//! an idempotent `Attach`, a `DriftedFingerprint` contradiction, and a `SplitSeal`
//! contradiction. The run self-checks its end state and panics (non-zero exit) if
//! composition or the expected outcome breaks.

use shaahid_contract::{Attestation, Contradiction, Deed, Fingerprint, witness};

/// The consumer's disposition for a presented deed. This is edge policy — it lives in the
/// consumer, never in the core.
#[derive(Debug, PartialEq, Eq)]
enum Disposition {
    /// A clean `Create`: fresh work, appended to the ledger.
    Recorded,
    /// A clean `Attach`: already witnessed, nothing appended — the idempotency win.
    Deduplicated,
    /// A contradiction was raised: the deed is not admitted.
    Quarantined,
}

fn fp(bytes: &[u8]) -> Fingerprint {
    Fingerprint::new(bytes.to_vec())
}

fn main() {
    // The consumer's own ledger. The core never owns, mutates, or persists it.
    let mut ledger: Vec<Deed<&'static str>> = Vec::new();
    let mut log: Vec<(&'static str, Disposition)> = Vec::new();

    // A stream spanning the four trajectories. "amount=100" content is shared by the
    // fresh charge and the split-identity deed so the latter collides on fingerprint.
    let stream = [
        (
            "fresh charge",
            Deed::new("seal:charge-1", fp(b"amount=100")),
        ),
        (
            "idempotent retry",
            Deed::new("seal:charge-1", fp(b"amount=100")),
        ),
        (
            "drifted retry",
            Deed::new("seal:charge-1", fp(b"amount=999")),
        ),
        (
            "split identity",
            Deed::new("seal:charge-2", fp(b"amount=100")),
        ),
    ];

    for (label, incoming) in stream {
        // Witness a clone so the deed survives for a possible record: `witness` consumes its
        // incoming, and the core bounds `Seal` by `Eq` alone — retaining is the consumer's
        // choice, so the consumer (not the core) opts into `Clone`.
        let outcome = witness(&ledger, incoming.clone());

        let disposition = if outcome.contradictions.is_empty() {
            match outcome.attestation {
                Attestation::Create => {
                    ledger.push(incoming);
                    Disposition::Recorded
                }
                Attestation::Attach(_seal) => Disposition::Deduplicated,
            }
        } else {
            // Quarantine wins over the attestation axis: an `Attach` that also drifts is
            // never silently deduplicated. Report each structural fact for the operator.
            for contradiction in &outcome.contradictions {
                let (kind, index) = match contradiction {
                    Contradiction::DriftedFingerprint { witnessed_index } => {
                        ("drift", *witnessed_index)
                    }
                    Contradiction::SplitSeal { witnessed_index } => ("split", *witnessed_index),
                };
                println!("  quarantine '{label}': {kind} vs ledger[{index}]");
            }
            Disposition::Quarantined
        };

        println!("{label}: {disposition:?}  (ledger size = {})", ledger.len());
        log.push((label, disposition));
    }

    check(&ledger, &log);
    println!("composition smoke OK: witness seam composes; four trajectories resolved");
}

/// Assert the expected dispositions and final ledger; panic (non-zero exit) on any drift.
fn check(ledger: &[Deed<&'static str>], log: &[(&'static str, Disposition)]) {
    let find = |label: &str| log.iter().find(|(l, _)| *l == label).map(|(_, d)| d);

    assert_eq!(
        find("fresh charge"),
        Some(&Disposition::Recorded),
        "a fresh create must be recorded"
    );
    assert_eq!(
        find("idempotent retry"),
        Some(&Disposition::Deduplicated),
        "a clean repeat must be deduplicated, not recorded again"
    );
    assert_eq!(
        find("drifted retry"),
        Some(&Disposition::Quarantined),
        "an attach that drifts must be quarantined, not deduplicated"
    );
    assert_eq!(
        find("split identity"),
        Some(&Disposition::Quarantined),
        "a create that splits an identity must be quarantined"
    );

    // Only the first clean create was admitted; every repeat and contradiction was withheld.
    assert_eq!(
        ledger.len(),
        1,
        "only the fresh create should be recorded in the ledger"
    );
    assert_eq!(ledger[0].seal, "seal:charge-1");
}
