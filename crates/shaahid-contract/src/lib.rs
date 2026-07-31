//! The isolated core contract for Shaahid: sans-I/O idempotency adjudication.
//!
//! Shaahid witnesses a [`Deed`] (a domain-supplied semantic identity — its `Seal` —
//! paired with a mechanical content identity — its [`Fingerprint`]) via [`witness`],
//! producing an [`Outcome`]: a create-or-attach [`Attestation`] plus any structural
//! [`Contradiction`] — and nothing more.
//!
//! # Axioms
//!
//! 1. **No semantic judgment in the core.** Semantic identity is domain-supplied as a
//!    `Seal`. The core decides create-or-attach by `Seal` equality and compares
//!    [`Fingerprint`]s byte-for-byte; it never decides whether two deeds *mean* the same
//!    thing. This is the *semantic bill of purity*: its cost (a wrong `Seal` that still
//!    matches its `Fingerprint` fails silently) is accepted deliberately rather than
//!    patched by judging meaning.
//! 2. **Sans-I/O purity.** The core exposes no `async fn`, reads no ambient clock, and
//!    performs no I/O. A runtime drives it and supplies the witnessed state at the edge.
//! 3. **No dependency on other workspace crates.**
//!
//! # Shape, not types
//!
//! The core is generic over the `Seal` type alone, bounding it by value-equality — it
//! never inspects or interprets a `Seal`. A [`Fingerprint`], by contrast, is
//! Shaahid-**owned** canonical bytes: the domain produces the bytes, and the core owns
//! their representation and compares them byte-for-byte. That asymmetry is deliberate —
//! a `Seal` is the domain's *meaning*, a `Fingerprint`'s bytes are the core's *mechanism*.
//!
//! # What this core does not do
//!
//! It observes; it does not decide admission. Whether to record an incoming `Deed` (even
//! a contradictory one) into a durable `Ledger`, and any response to a `Contradiction`,
//! lie outside the pattern's shape — a pure adjudication that owns no durable state and
//! makes no judgment cannot own them. Persisting the `Ledger` and an `async` variant are
//! likewise out of this core (see `BACKLOG.md`).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// The mechanical content identity of a [`Deed`] — Shaahid-owned canonical bytes.
///
/// The domain produces the bytes (a hash of the content); Shaahid owns their canonical
/// representation and compares them byte-for-byte. Unlike a `Seal`, a `Fingerprint` is
/// not a domain type the core carries opaquely — the core's whole business with a
/// `Fingerprint` is to compare its bytes, so it owns them. Immutable once built.
///
/// **Non-goals.** The name evokes a content hash, but the core never becomes the hasher.
/// It computes no hash, stores no algorithm identifier alongside the bytes, and enforces
/// no length — choosing an algorithm and a length, and producing the bytes, is the
/// domain's. The core owns only the canonical representation and the byte-for-byte
/// comparison; `Fingerprint`s of different lengths compare as unequal, and none is
/// rejected or normalized for its length.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingerprint(Box<[u8]>);

impl Fingerprint {
    /// Build a fingerprint from domain-produced content-hash bytes.
    #[must_use]
    pub fn new(bytes: impl Into<Box<[u8]>>) -> Self {
        Self(bytes.into())
    }

    /// The canonical bytes, compared byte-for-byte by the core.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// A unit of work presented to be witnessed: a domain-supplied semantic identity (the
/// `Seal` type parameter) paired with its mechanical content identity (a [`Fingerprint`]).
///
/// The core is generic over the `Seal` alone; the `Fingerprint` is a Shaahid-owned type.
/// The core carries the `Seal` opaquely and never interprets it — deciding what a deed
/// *means* is a domain judgment, not the core's (see the crate axioms).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deed<Seal> {
    /// The domain-supplied semantic identity, carried opaquely and compared only by value.
    pub seal: Seal,
    /// The domain-produced content identity, owned by the core and compared byte-for-byte.
    pub fingerprint: Fingerprint,
}

impl<Seal> Deed<Seal> {
    /// Present a deed for witnessing from its `Seal` and `Fingerprint`.
    #[must_use]
    pub fn new(seal: Seal, fingerprint: Fingerprint) -> Self {
        Self { seal, fingerprint }
    }
}

/// The create-or-attach verdict for a witnessed [`Deed`].
///
/// A closed, two-variant settlement — deliberately **not** `#[non_exhaustive]`. The
/// verdict space is finite by design, so a genuinely new outcome should force a
/// deliberate breaking change rather than silently widening the surface. The verdict is
/// a mechanism, never a policy: it carries no response to attach — that lies outside the pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attestation<Seal> {
    /// The deed's `Seal` was new to the witnessed set: witness it as fresh work.
    Create,
    /// The deed's `Seal` was already witnessed: this is the same work. Carries the
    /// presented `Seal` by value (equal to the already-witnessed one) and nothing else.
    Attach(Seal),
}

/// A structural anomaly the core detected mechanically while witnessing — an alarm, not
/// a judgment that the domain was wrong. Non-generic: it names the conflicting witnessed
/// [`Deed`] by its index in the witnessed slice, so it imposes no `Clone` or lifetime on
/// the identity types. Each variant refers to exactly one witnessed `Deed` and is
/// meaningful only relative to the [`witness`] call that produced it.
///
/// A closed enum, deliberately **not** `#[non_exhaustive]` — like [`Attestation`], and
/// with more reason: `Contradiction` is the axis likelier to grow as new structural
/// anomalies are found. Keeping it closed forces any new anomaly kind through a
/// deliberate breaking change, never a silent, additive widening of what counts as a
/// contradiction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Contradiction {
    /// A witnessed `Deed` shares the incoming `Seal` but carries a different `Fingerprint`
    /// (the domain reused an identity for changed content).
    DriftedFingerprint {
        /// Index of the conflicting witnessed `Deed` in the witnessed slice.
        witnessed_index: usize,
    },
    /// A witnessed `Deed` shares the incoming `Fingerprint` but carries a different `Seal`
    /// (the domain split an identity across identical content).
    SplitSeal {
        /// Index of the conflicting witnessed `Deed` in the witnessed slice.
        witnessed_index: usize,
    },
}

/// The full result of a [`witness`]: the create-or-attach [`Attestation`] and every
/// structural [`Contradiction`] the incoming `Deed` raised against the witnessed set.
///
/// The two are orthogonal — a witness may be `Attach` with one or more drifts, `Create`
/// with one or more splits, or both. Contradictions are a list, never a single optional:
/// one incoming `Deed` can expose several structural facts at once (and the witnessed
/// ledger may already be inconsistent).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome<Seal> {
    /// The create-or-attach verdict, decided by `Seal` equality.
    pub attestation: Attestation<Seal>,
    /// Every structural contradiction the incoming `Deed` raised, in ascending
    /// `witnessed_index`.
    pub contradictions: Vec<Contradiction>,
}

/// Witness a [`Deed`] against the witnessed `Deed`s, producing an [`Outcome`] that
/// carries the create-or-attach [`Attestation`] and every structural [`Contradiction`].
///
/// Two orthogonal axes from one pass:
/// - **Attestation** — [`Attestation::Create`] if the incoming `Seal` is new to the
///   witnessed set, else [`Attestation::Attach`] carrying that `Seal` by value.
/// - **Contradictions** — a [`Contradiction::DriftedFingerprint`] for each witnessed
///   `Deed` with the same `Seal` but a different `Fingerprint`, and a
///   [`Contradiction::SplitSeal`] for each with the same `Fingerprint` but a different
///   `Seal`. Emitted in ascending `witnessed_index` — a total order, since at most one
///   contradiction arises per witnessed `Deed` (drift needs the seals equal, split needs
///   them unequal).
///
/// A pure function of its inputs: `witnessed` is read-only, the core mutates nothing,
/// reads no clock, performs no I/O, and decides no admission — whether to record the
/// incoming `Deed` is the caller's. It bounds `Seal` by `Eq` alone. The comparison scope
/// is incoming-versus-each-witnessed; it does not audit the witnessed ledger against
/// itself.
///
/// # Examples
///
/// ```
/// use shaahid_contract::{witness, Attestation, Contradiction, Deed, Fingerprint};
///
/// // A ledger holding one witnessed deed.
/// let ledger = [Deed::new("seal:charge-42", Fingerprint::new(*b"content-42"))];
///
/// // Same seal, different content: attaches, but drifts.
/// let outcome = witness(&ledger, Deed::new("seal:charge-42", Fingerprint::new(*b"content-99")));
/// assert_eq!(outcome.attestation, Attestation::Attach("seal:charge-42"));
/// assert_eq!(
///     outcome.contradictions,
///     vec![Contradiction::DriftedFingerprint { witnessed_index: 0 }],
/// );
///
/// // A brand-new seal with brand-new content: a clean create.
/// let fresh = witness(&ledger, Deed::new("seal:charge-99", Fingerprint::new(*b"content-x")));
/// assert_eq!(fresh.attestation, Attestation::Create);
/// assert!(fresh.contradictions.is_empty());
/// ```
#[must_use]
pub fn witness<Seal>(witnessed: &[Deed<Seal>], incoming: Deed<Seal>) -> Outcome<Seal>
where
    Seal: Eq,
{
    let mut seal_seen = false;
    let mut contradictions = Vec::new();

    for (witnessed_index, past) in witnessed.iter().enumerate() {
        let same_seal = past.seal == incoming.seal;
        let same_fingerprint = past.fingerprint == incoming.fingerprint;

        if same_seal {
            seal_seen = true;
            if !same_fingerprint {
                contradictions.push(Contradiction::DriftedFingerprint { witnessed_index });
            }
        } else if same_fingerprint {
            contradictions.push(Contradiction::SplitSeal { witnessed_index });
        }
    }

    let attestation = if seal_seen {
        Attestation::Attach(incoming.seal)
    } else {
        Attestation::Create
    };

    Outcome {
        attestation,
        contradictions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fp(bytes: &[u8]) -> Fingerprint {
        Fingerprint::new(bytes.to_vec())
    }

    // A domain seal that is `Eq` but deliberately NOT `Ord` and NOT `Hash`: that
    // `witness` decides over it at all proves the core demands only value-equality.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct UnorderedSeal(u32);

    #[test]
    fn a_new_seal_creates_and_a_witnessed_seal_attaches() {
        let ledger = [Deed::new("seal:a", fp(b"content-a"))];

        let fresh = witness(&ledger, Deed::new("seal:b", fp(b"content-b")));
        assert_eq!(fresh.attestation, Attestation::Create);
        assert!(fresh.contradictions.is_empty());

        let repeat = witness(&ledger, Deed::new("seal:a", fp(b"content-a")));
        assert_eq!(repeat.attestation, Attestation::Attach("seal:a"));
        assert!(repeat.contradictions.is_empty());
    }

    #[test]
    fn witnessing_is_a_pure_function_of_supplied_state() {
        let ledger = [Deed::new("seal:a", fp(b"content-a"))];
        let incoming = || Deed::new("seal:a", fp(b"content-a"));

        assert_eq!(witness(&ledger, incoming()), witness(&ledger, incoming()));
    }

    #[test]
    fn fingerprint_is_domain_produced_and_core_owned_bytes() {
        let fingerprint = Fingerprint::new(vec![0xde, 0xad, 0xbe, 0xef]);
        // The domain's bytes, held verbatim in the core's canonical type.
        assert_eq!(fingerprint.as_bytes(), &[0xde, 0xad, 0xbe, 0xef]);
        // Comparison is byte-for-byte, by the core's own type.
        assert_eq!(fingerprint, Fingerprint::new(vec![0xde, 0xad, 0xbe, 0xef]));
        assert_ne!(fingerprint, Fingerprint::new(vec![0xde, 0xad, 0xbe, 0xff]));
    }

    #[test]
    fn attach_co_occurs_with_a_drift() {
        let ledger = [Deed::new("seal:a", fp(b"v1"))];

        let outcome = witness(&ledger, Deed::new("seal:a", fp(b"v2")));
        assert_eq!(outcome.attestation, Attestation::Attach("seal:a"));
        assert_eq!(
            outcome.contradictions,
            vec![Contradiction::DriftedFingerprint { witnessed_index: 0 }]
        );
    }

    #[test]
    fn create_co_occurs_with_a_split() {
        let ledger = [Deed::new("seal:a", fp(b"shared"))];

        let outcome = witness(&ledger, Deed::new("seal:b", fp(b"shared")));
        assert_eq!(outcome.attestation, Attestation::Create);
        assert_eq!(
            outcome.contradictions,
            vec![Contradiction::SplitSeal { witnessed_index: 0 }]
        );
    }

    #[test]
    fn multiple_contradictions_surface_in_ascending_index() {
        // index 0: split (same fp, different seal); index 2: drift (same seal, diff fp).
        let ledger = [
            Deed::new("seal:x", fp(b"shared")),
            Deed::new("seal:y", fp(b"unrelated")),
            Deed::new("seal:a", fp(b"v1")),
        ];

        let outcome = witness(&ledger, Deed::new("seal:a", fp(b"shared")));
        assert_eq!(outcome.attestation, Attestation::Attach("seal:a"));
        assert_eq!(
            outcome.contradictions,
            vec![
                Contradiction::SplitSeal { witnessed_index: 0 },
                Contradiction::DriftedFingerprint { witnessed_index: 2 },
            ]
        );
    }

    #[test]
    fn a_clean_rewitness_raises_no_contradiction() {
        let ledger = [Deed::new("seal:a", fp(b"content-a"))];

        let outcome = witness(&ledger, Deed::new("seal:a", fp(b"content-a")));
        assert_eq!(outcome.attestation, Attestation::Attach("seal:a"));
        assert!(outcome.contradictions.is_empty());
    }

    #[test]
    fn scope_is_incoming_versus_witnessed_not_within_ledger() {
        // The ledger is already internally inconsistent: seal:a under two fingerprints.
        let ledger = [
            Deed::new("seal:a", fp(b"v1")),
            Deed::new("seal:a", fp(b"v2")),
        ];

        // Incoming drifts against BOTH — two contradictions, one per witnessed deed.
        let outcome = witness(&ledger, Deed::new("seal:a", fp(b"v3")));
        assert_eq!(outcome.attestation, Attestation::Attach("seal:a"));
        assert_eq!(
            outcome.contradictions,
            vec![
                Contradiction::DriftedFingerprint { witnessed_index: 0 },
                Contradiction::DriftedFingerprint { witnessed_index: 1 },
            ],
            "the pre-existing v1<->v2 inconsistency is not itself reported"
        );
    }

    #[test]
    fn witness_needs_only_seal_value_equality() {
        // `UnorderedSeal` is `Eq` but not `Ord`/`Hash`; witnessing over it proves the
        // core requires nothing of a `Seal` but value-equality.
        let ledger = [
            Deed::new(UnorderedSeal(1), fp(b"a")),
            Deed::new(UnorderedSeal(2), fp(b"b")),
        ];

        assert_eq!(
            witness(&ledger, Deed::new(UnorderedSeal(3), fp(b"c"))).attestation,
            Attestation::Create
        );
        assert_eq!(
            witness(&ledger, Deed::new(UnorderedSeal(2), fp(b"b"))).attestation,
            Attestation::Attach(UnorderedSeal(2))
        );
    }

    #[test]
    fn fingerprints_of_differing_lengths_compare_unequal() {
        // The core enforces no length policy: differing-length fingerprints simply
        // compare unequal, and neither is rejected or normalized for its length.
        let short = Fingerprint::new(vec![0x01, 0x02]);
        let long = Fingerprint::new(vec![0x01, 0x02, 0x03]);
        assert_ne!(short, long);
        assert_eq!(short.as_bytes().len(), 2);
        assert_eq!(long.as_bytes().len(), 3);
    }

    #[test]
    fn attestation_and_contradiction_match_exhaustively_without_wildcard() {
        // Pins the not-`#[non_exhaustive]` contract: these exhaustive matches carry no
        // wildcard arm, so a future added variant would break them at compile time.
        let attestation: Attestation<&str> = Attestation::Create;
        let _ = match attestation {
            Attestation::Create => "create",
            Attestation::Attach(_) => "attach",
        };

        let contradiction = Contradiction::SplitSeal { witnessed_index: 0 };
        let _ = match contradiction {
            Contradiction::DriftedFingerprint { .. } => "drift",
            Contradiction::SplitSeal { .. } => "split",
        };
    }
}
