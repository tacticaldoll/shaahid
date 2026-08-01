//! Composition proof: an idempotency-gate composer driven through the `shaahid` facade.
//!
//! This is the first real composer of the adjudication contract. It holds its own
//! witnessed ledger, presents a stream of deeds, and turns each `Outcome` into an action
//! — **record** a clean create, **deduplicate** a clean attach, **quarantine** anything
//! contradictory — using **only** the `shaahid` facade's public API, never
//! `shaahid-contract` directly. So it doubles as proof that the facade re-exports every
//! type a composing system needs. It proves two things:
//!
//! - **The seam composes:** an idempotency gate is buildable over the facade without
//!   reaching inside the core.
//! - **Admission is the composer's:** the core witnesses and alarms; this composer decides
//!   what enters its ledger. The ledger here is the composer's, never the core's.
//!
//! To have teeth, the stub domain drives four trajectories in one run: a fresh `Create`,
//! an idempotent `Attach`, a `DriftedFingerprint` contradiction, and a `SplitSeal`
//! contradiction.

use shaahid::{Attestation, Contradiction, Deed, Fingerprint, witness};

/// The composer's disposition for a presented deed. This is edge policy — it lives in the
/// composer, never in the core.
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

#[test]
fn idempotency_gate_resolves_four_trajectories() {
    // The composer's own ledger. The core never owns, mutates, or persists it.
    let mut ledger: Vec<Deed<&'static str>> = Vec::new();
    let mut log: Vec<(&'static str, Disposition, Vec<String>)> = Vec::new();

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
        // incoming, and the core bounds `Seal` by `Eq` alone — retaining is the composer's
        // choice, so the composer (not the core) opts into `Clone`.
        let outcome = witness(&ledger, incoming.clone());

        let mut messages: Vec<String> = Vec::new();
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
            // never silently deduplicated. Display each contradiction for the operator log
            // — using only the facade's public API, proving Contradiction's Display
            // composes through it, not just in shaahid-contract's own unit tests.
            messages = outcome
                .contradictions
                .iter()
                .map(Contradiction::to_string)
                .collect();
            Disposition::Quarantined
        };

        log.push((label, disposition, messages));
    }

    check(&ledger, &log);
}

/// Assert the expected dispositions, ledger, and operator-facing contradiction messages;
/// the core admitted nothing itself.
fn check(ledger: &[Deed<&'static str>], log: &[(&'static str, Disposition, Vec<String>)]) {
    let find = |label: &str| {
        log.iter()
            .find(|(l, _, _)| *l == label)
            .map(|(_, d, m)| (d, m))
            .unwrap_or_else(|| panic!("`{label}` should be logged"))
    };

    let (fresh, fresh_messages) = find("fresh charge");
    assert_eq!(
        fresh,
        &Disposition::Recorded,
        "a fresh create must be recorded"
    );
    assert!(
        fresh_messages.is_empty(),
        "a clean create raises no contradiction to display"
    );

    let (idempotent, idempotent_messages) = find("idempotent retry");
    assert_eq!(
        idempotent,
        &Disposition::Deduplicated,
        "a clean repeat must be deduplicated, not recorded again"
    );
    assert!(
        idempotent_messages.is_empty(),
        "a clean re-witness raises no contradiction to display"
    );

    let (drifted, drifted_messages) = find("drifted retry");
    assert_eq!(
        drifted,
        &Disposition::Quarantined,
        "an attach that drifts must be quarantined, not deduplicated"
    );
    assert_eq!(drifted_messages.len(), 1, "exactly one drift was raised");
    assert!(
        drifted_messages[0].contains("drift") && drifted_messages[0].contains('0'),
        "the displayed message should name the drift and its witnessed_index: {drifted_messages:?}"
    );

    let (split, split_messages) = find("split identity");
    assert_eq!(
        split,
        &Disposition::Quarantined,
        "a create that splits an identity must be quarantined"
    );
    assert_eq!(split_messages.len(), 1, "exactly one split was raised");
    assert!(
        split_messages[0].contains("split") && split_messages[0].contains('0'),
        "the displayed message should name the split and its witnessed_index: {split_messages:?}"
    );

    // Only the first clean create was admitted; every repeat and contradiction was withheld.
    assert_eq!(
        ledger.len(),
        1,
        "only the fresh create should be recorded in the ledger"
    );
    assert_eq!(ledger[0].seal, "seal:charge-1");
}
