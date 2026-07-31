//! The isolated core contract for Shaahid: sans-I/O idempotency adjudication.
//!
//! Shaahid witnesses a [`Deed`] (a domain-supplied semantic identity — its `Seal` —
//! paired with a mechanical content identity — its `Fingerprint`) and attests
//! create-or-attach via [`adjudicate`], and nothing more.
//!
//! # Axioms
//!
//! 1. **No semantic judgment in the core.** Semantic identity is domain-supplied as a
//!    `Seal`. The core adjudicates by `Seal` equality and (in a later change) compares
//!    `Fingerprint`s byte-wise; it never decides whether two deeds *mean* the same
//!    thing. This is the *semantic bill of purity*: its cost (a wrong `Seal` that still
//!    matches its `Fingerprint` fails silently) is accepted deliberately rather than
//!    patched by judging meaning.
//! 2. **Sans-I/O purity.** The core exposes no `async fn`, reads no ambient clock, and
//!    performs no I/O. A runtime drives it and supplies the witnessed state at the edge.
//! 3. **No dependency on other workspace crates.**
//!
//! # Shape, not types
//!
//! The core is **generic** over the `Seal` and `Fingerprint` types: it fixes the
//! *shape* of a deed and the create-or-attach verdict, but never the domain's concrete
//! identity types. The domain brings its own; the core constrains each only by the
//! minimal capability it uses — a `Seal` by value-equality, a `Fingerprint` (when the
//! contradiction check lands) by its bytes.
//!
//! # Status
//!
//! This change realizes the create-or-attach decision ([`adjudicate`]) as a pure
//! function of the witnessed `Seal`s and a [`Deed`]. Structural contradiction detection
//! (comparing `Fingerprint`s), the durable ledger, and any response to a contradiction
//! remain out of this core — the first is a later spec-driven change, the rest are
//! downstream consumer concerns (see `BACKLOG.md`).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// A unit of work presented to be witnessed: a domain-supplied semantic identity (the
/// `Seal` type parameter) paired with its mechanical content identity (the
/// `Fingerprint` type parameter).
///
/// The core is generic over both — it fixes that a deed *is* a seal and a fingerprint,
/// but never what the domain's concrete identity types are. It carries the seal
/// opaquely and never interprets it; deciding what a deed *means* is a domain judgment,
/// not the core's (see the crate axioms).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deed<Seal, Fingerprint> {
    seal: Seal,
    fingerprint: Fingerprint,
}

impl<Seal, Fingerprint> Deed<Seal, Fingerprint> {
    /// Present a deed for witnessing from its domain-supplied `Seal` and `Fingerprint`.
    #[must_use]
    pub fn new(seal: Seal, fingerprint: Fingerprint) -> Self {
        Self { seal, fingerprint }
    }

    /// The domain-supplied semantic identity. The core carries it opaquely and never
    /// interprets it.
    #[must_use]
    pub fn seal(&self) -> &Seal {
        &self.seal
    }

    /// The domain-supplied mechanical content identity. The core receives it — it never
    /// computes one — and (in the contradiction check, a later change) compares it
    /// byte-wise, never by meaning.
    #[must_use]
    pub fn fingerprint(&self) -> &Fingerprint {
        &self.fingerprint
    }
}

/// The create-or-attach verdict for a witnessed [`Deed`].
///
/// A closed, two-variant settlement — deliberately **not** `#[non_exhaustive]`. The
/// verdict space is finite by design, so a genuinely new outcome should force a
/// deliberate breaking change rather than silently widening the surface. The verdict is
/// a mechanism, never a policy: it carries no response to attach — that is downstream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attestation<Seal> {
    /// The deed's `Seal` was new to the witnessed set: witness it as fresh work.
    Create,
    /// The deed's `Seal` was already witnessed: this is the same work. Carries the
    /// matched `Seal` by value and nothing else — no reference or handle into a ledger,
    /// a deed store, or any downstream policy.
    Attach(Seal),
}

/// Adjudicate a [`Deed`] against the witnessed `Seal`s to a create-or-attach
/// [`Attestation`], decided by `Seal` value-equality alone — mechanical, never a
/// comparison of meaning.
///
/// A pure function of its inputs: it owns, persists, and mutates no ledger state,
/// requires no registry trait, reads no ambient clock, and performs no I/O. The
/// witnessed set — the `Seal`s already witnessed — is supplied by value at the edge; a
/// `Seal` absent from it attests [`Attestation::Create`], one present attests
/// [`Attestation::Attach`] carrying that `Seal`.
///
/// `witnessed` is a slice, not a set type: the core tests membership by value-equality
/// and depends on neither ordering nor uniqueness, so keeping the witnessed set unique
/// is the caller's concern, not a bound the core imposes. On `Attach`, the returned
/// `Seal` is the one `deed` presented (equal by value to the already-witnessed one); the
/// core hands it back rather than cloning the witnessed element, so it requires only
/// `Eq` of a `Seal` — never `Clone`, ordering, or interpretation of its contents.
///
/// # Examples
///
/// ```
/// use shaahid_contract::{adjudicate, Attestation, Deed};
///
/// let witnessed = ["seal:charge-42"];
///
/// // A seal new to the witnessed set is fresh work.
/// let fresh = adjudicate(&witnessed, Deed::new("seal:charge-99", b"bytes-99"));
/// assert_eq!(fresh, Attestation::Create);
///
/// // A seal already witnessed attaches, carrying that seal back.
/// let seen = adjudicate(&witnessed, Deed::new("seal:charge-42", b"bytes-42"));
/// assert_eq!(seen, Attestation::Attach("seal:charge-42"));
/// ```
#[must_use]
pub fn adjudicate<Seal, Fingerprint>(
    witnessed: &[Seal],
    deed: Deed<Seal, Fingerprint>,
) -> Attestation<Seal>
where
    Seal: Eq,
{
    if witnessed.contains(&deed.seal) {
        Attestation::Attach(deed.seal)
    } else {
        Attestation::Create
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A domain seal that is `Eq` but deliberately NOT `Ord` and NOT `Hash`, proving the
    // core demands only value-equality of a `Seal` — that `adjudicate` accepts it at all
    // is the proof.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct UnorderedSeal(u32);

    #[test]
    fn a_new_seal_creates_and_a_witnessed_seal_attaches() {
        let witnessed = ["deed:charge-42"];

        let fresh = adjudicate(&witnessed, Deed::new("deed:charge-99", b"content-99"));
        assert_eq!(fresh, Attestation::Create);

        let repeat = adjudicate(&witnessed, Deed::new("deed:charge-42", b"content-42"));
        assert_eq!(repeat, Attestation::Attach("deed:charge-42"));
    }

    #[test]
    fn adjudication_is_a_pure_function_of_supplied_state() {
        let witnessed = ["deed:charge-42"];
        let deed = || Deed::new("deed:charge-42", b"content-42");

        // Same witnessed set and same deed -> identical attestation, every time.
        assert_eq!(
            adjudicate(&witnessed, deed()),
            adjudicate(&witnessed, deed())
        );
    }

    #[test]
    fn the_core_carries_a_received_fingerprint_and_computes_none() {
        let deed = Deed::new("deed:charge-42", vec![0xde, 0xad, 0xbe, 0xef]);
        // The fingerprint is the domain's bytes, carried verbatim; the core minted none.
        assert_eq!(deed.fingerprint(), &vec![0xde, 0xad, 0xbe, 0xef]);
        assert_eq!(deed.seal(), &"deed:charge-42");
    }

    #[test]
    fn attach_carries_only_the_matched_seal_by_value() {
        let witnessed = [String::from("seal-a"), String::from("seal-b")];

        match adjudicate(&witnessed, Deed::new(String::from("seal-b"), b"bytes")) {
            Attestation::Attach(seal) => assert_eq!(seal, "seal-b"),
            Attestation::Create => panic!("a witnessed seal must attach"),
        }
    }

    #[test]
    fn adjudication_needs_only_seal_value_equality() {
        // `UnorderedSeal` is `Eq` but not `Ord`/`Hash`; adjudicate deciding over it
        // proves the core requires nothing but value-equality.
        let witnessed = [UnorderedSeal(1), UnorderedSeal(2)];

        assert_eq!(
            adjudicate(&witnessed, Deed::new(UnorderedSeal(3), ())),
            Attestation::Create
        );
        assert_eq!(
            adjudicate(&witnessed, Deed::new(UnorderedSeal(2), ())),
            Attestation::Attach(UnorderedSeal(2))
        );
    }
}
