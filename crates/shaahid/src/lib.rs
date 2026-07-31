//! Shaahid: a thin, sans-I/O idempotency-adjudication core you compose.
//!
//! This crate is the curated public entrypoint. It re-exports the public API of the
//! Shaahid workspace so a composing system can depend on one crate:
//!
//! - the identity types — [`Fingerprint`] and [`Deed`];
//! - the witness verdict — [`Attestation`], [`Contradiction`], [`Outcome`];
//! - the adjudication itself — [`witness`].
//!
//! It carries no logic of its own: every item here is a re-export. Shaahid's whole
//! public surface is compose-level, so the facade withholds nothing — there is no
//! advanced kernel to reach for through [`shaahid_contract`] directly.
//!
//! # The contract
//!
//! Shaahid owns a *mechanism* and no *meaning*. [`witness`] decides create-or-attach by
//! `Seal` equality and compares [`Fingerprint`]s byte-for-byte; it never judges whether
//! two deeds *mean* the same thing, and it never decides admission — whether to record a
//! deed into a ledger is the composing system's. See the crate-level docs of [`shaahid_contract`]
//! for the full axioms and the semantic bill of purity.
//!
//! # Composing a witness
//!
//! A create-then-attach idempotency check wired entirely through this entrypoint: a
//! fresh `Seal` witnesses as [`Attestation::Create`], and re-witnessing the same deed
//! attaches with no contradiction — the idempotency win. Admission (pushing to the
//! ledger) is the composing system's, never the core's (run `cargo test` to see it execute):
//!
//! ```
//! use shaahid::{Attestation, Deed, Fingerprint, witness};
//!
//! // The composing system's own witnessed ledger. The core never owns, mutates, or persists it.
//! let mut ledger: Vec<Deed<&str>> = Vec::new();
//!
//! // A fresh seal with fresh content: a clean Create.
//! let first = Deed::new("seal:charge-1", Fingerprint::new(*b"amount=100"));
//! let created = witness(&ledger, first.clone());
//! assert_eq!(created.attestation, Attestation::Create);
//! assert!(created.contradictions.is_empty());
//! ledger.push(first); // admission is the composing system's choice, not the core's
//!
//! // The same deed again: now it Attaches — deduplicated — with no contradiction.
//! let repeat = witness(&ledger, Deed::new("seal:charge-1", Fingerprint::new(*b"amount=100")));
//! assert_eq!(repeat.attestation, Attestation::Attach("seal:charge-1"));
//! assert!(repeat.contradictions.is_empty());
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use shaahid_contract::*;
