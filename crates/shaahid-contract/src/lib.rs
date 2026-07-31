//! The isolated core contract for Shaahid: sans-I/O idempotency adjudication.
//!
//! Shaahid witnesses a `Deed` (a domain-supplied [`Seal`] and a content
//! [`Fingerprint`]) and attests create-or-attach, detecting structural
//! contradictions — and nothing more.
//!
//! # Axioms
//!
//! 1. **No semantic judgment in the core.** Semantic identity is domain-supplied as a
//!    [`Seal`]. The core adjudicates by `Seal` equality and compares [`Fingerprint`]s
//!    byte-wise; it never decides whether two deeds *mean* the same thing. This is the
//!    *semantic bill of purity*: its cost (a wrong `Seal` that still matches its
//!    `Fingerprint` fails silently) is accepted deliberately rather than patched by
//!    judging meaning.
//! 2. **Sans-I/O purity.** The core exposes no `async fn`, reads no ambient clock, and
//!    performs no I/O. A runtime drives it and supplies the witnessed state at the edge.
//! 3. **No dependency on other workspace crates.**
//!
//! # Status
//!
//! This crate is the initial shape: the vocabulary anchors below plus the axioms. The
//! adjudication and contradiction-detection logic are defined in `openspec/specs/` and
//! built in later spec-driven changes (see `BACKLOG.md`). The durable ledger and any
//! response to a contradiction are downstream consumer concerns, not this core.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// A domain-supplied, stable semantic identity for a deed: what the domain means by
/// "the same work".
///
/// The same intent carries the same `Seal`; a genuine semantic change carries a new
/// one. The core carries a `Seal` opaquely and never interprets it — deciding semantic
/// identity is a domain judgment, not the core's (see the crate axioms). Construct via
/// [`Seal::new`] and read via [`Seal::as_str`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seal(String);

impl Seal {
    /// Mint a seal from a domain-supplied stable identity.
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The opaque identity string. The core does not interpret it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The mechanical content identity of a deed — a hash of its bytes.
///
/// Unlike a [`Seal`], a `Fingerprint` encodes content, not meaning, and the core
/// compares it byte-wise. A `Seal` presented with a drifted `Fingerprint` (or a
/// `Fingerprint` under split `Seal`s) is a structural contradiction the core can
/// detect mechanically.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fingerprint(Vec<u8>);

impl Fingerprint {
    /// Build a fingerprint from content-hash bytes.
    #[must_use]
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    /// The content-hash bytes, compared byte-wise by the core.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// The verdict for a witnessed deed.
///
/// `#[non_exhaustive]` because the attestation may gain a variant as the design
/// settles; a downstream match must carry a wildcard arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Attestation {
    /// The deed's `Seal` is new to the ledger: witness it as fresh work.
    Create,
    /// The deed's `Seal` was already witnessed: this is the same work, attach to it.
    Attach,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seal_carries_domain_identity_opaquely() {
        let seal = Seal::new("deed:charge-invoice-42");
        assert_eq!(seal.as_str(), "deed:charge-invoice-42");
        // Identity is value equality; the core compares bytes, not meaning.
        assert_eq!(seal, Seal::new("deed:charge-invoice-42"));
        assert_ne!(seal, Seal::new("deed:charge-invoice-43"));
    }

    #[test]
    fn fingerprint_compares_content_byte_wise() {
        let fingerprint = Fingerprint::new(vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(fingerprint.as_bytes(), &[0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(fingerprint, Fingerprint::new(vec![0xde, 0xad, 0xbe, 0xef]));
        assert_ne!(fingerprint, Fingerprint::new(vec![0xde, 0xad, 0xbe, 0xff]));
    }

    #[test]
    fn attestation_distinguishes_create_from_attach() {
        assert_ne!(Attestation::Create, Attestation::Attach);
    }
}
