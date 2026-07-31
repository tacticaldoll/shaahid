# Backlog & Deferred Decisions

This file records deferred decisions, open design questions, and candidate patterns.
It is not a phase roadmap and creates no implementation commitments. Shipped truth
lives in `openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- The **initial project shape** is established: vision and non-goals (`PROJECT.md`),
  operating protocol and axioms (`AGENTS.md`), witness domain language
  (`docs/domain-language.md`), executable governance (`shaahid-governance`), and the
  crate layout (`shaahid-contract` + `shaahid-governance`) — since grown, per the next
  entry, into the shipped witness core.
- The witness core is **shipped**: `witness` adjudicates create-or-attach by `Seal`
  equality and detects structural contradictions, returning an `Outcome`. Its contract
  lives in `openspec/specs/` (`adjudication-contract`). Remaining open questions are
  below.

## Workspace Composition

The workspace stays thin. It owns the adjudication contract (`shaahid-contract`) and
its governance gate (`shaahid-governance`, unpublished). The durable `Ledger` and any
policy on a `Contradiction` **lie outside the pattern's shape** and live outside this
workspace. Adding a workspace crate requires a justified Tianheng boundary or the
coverage gate fails.

## Design Decisions (this shape)

- **Adjudication model.** A `Deed` is witnessed to an `Attestation` of `Create` (the
  `Seal` is new to the `Ledger`) or `Attach` (the `Seal` is already witnessed). The
  decision is `Seal` equality — mechanical, never a comparison of meaning.
- **The semantic bill of purity.** A sans-I/O pure core cannot decide semantic
  identity, so the domain supplies it as a `Seal`. Shaahid adjudicates by `Seal`
  equality and compares `Fingerprint`s mechanically; it never decides whether two
  `Deed`s *mean* the same. The cost — a wrong `Seal` that matches its `Fingerprint`
  fails silently — is accepted, not patched by judging meaning.
- **Fingerprint is mechanical, Seal is semantic.** The `Fingerprint` encodes content
  (a hash) and is compared byte-wise; the `Seal` encodes meaning and is opaque. Their
  disagreement is a `Contradiction`.
- **Contradiction is an alarm, not a judgment.** The core detects same-`Seal`/drifted-
  `Fingerprint` and same-`Fingerprint`/split-`Seal` mechanically and surfaces it. It
  does not decide the domain was wrong, and it owns no response.
- **Core mechanism, edge policy.** The core attests and alarms; the response to a
  `Contradiction` (reject, quarantine, escalate) and the durable `Ledger` are
  downstream concerns.
- **Governance mostly mirrors the reference discipline** (crate-dependency boundaries,
  sans-I/O purity teeth, workspace coverage, active-prose governance) — with one
  honest exception: see the open question on the unenforceable purity invariant.

## Open Design Questions

Recorded so the repo can drive its own development; none is decided here. Discipline:
keep meaning domain-supplied; the core only adjudicates or detects mechanically, never
compares meanings. Do not freeze a user-obligation trait ahead of its first composer.

- **The unenforceable purity invariant.** "The core makes no semantic judgment" is
  not statically expressible — semantic comparison has no syntactic marker, so Tianheng
  cannot bite it the way it bites no-I/O or no-async. It stays review- and
  structure-governed. `Contradiction` detection narrows the silent-failure surface but
  does not enforce the invariant.
- **Async variant.** Deferred until a real driver forces it; the sans-I/O core is
  agnostic to sync/async at the edge.
- **Facade as composition-ergonomics artifact.** Does a pattern-product need the `shaahid`
  facade crate, or is it a composition convenience the pattern itself does not require?
  Deferred; a structural question with a wide blast radius, not decided here.
- **Spec framing residue.** `public-facade`'s requirement prose still projects the
  service-`consumer` frame retired from the governing docs. Reconcile it the next time that
  spec changes for a real reason, rather than through a behavior-inert delta now.

## Recorded Reconsiderations

Inherited discipline first, then this project's own resolved design decisions.


- **Self-positioning reclaimed — resolved.** The governing prose defined the product
  consumer-relative — justifying each non-goal by a downstream *consumer* rather than by
  the pattern's own shape — the consumer-driven frame leaking into governance. Re-anchored
  across `AGENTS.md`, `PROJECT.md`, `README.md`, and `docs/domain-language.md`: each
  non-goal is now self-justified from the pattern's nature (a sans-I/O adjudication owning
  no state cannot persist a `Ledger`; an alarm that makes no judgment cannot own a
  response), and the service-word "consumer" gives way to composition language while
  "downstream" is kept only as architectural direction. Boundaries did not move; only their
  justification did. Done as a governing-docs `docs:` change, **not** an OpenSpec change —
  it alters no requirement or behavior, and provenance lives in git plus this file, not a
  separate record class. Scope held: the historical entries below keep their original
  wording, and `public-facade`'s spec framing was left to a future spec change (see Open
  Design Questions).
- **No architecture-decision-record files.** Decision provenance lives in git commit
  bodies and pull requests; reconsiderations live here; the living docs are the single
  source of truth for current state. The starter's `docs/adr/` was removed on birth.
- **Sync and archive are two gates.** Sync promotes delta specs into `openspec/specs/`
  and leaves the change directory in place — the change stays active for verification. A
  distinct archive step then removes the change directory; git retains the deliberation.
  No `openspec/changes/archive/` folder is created — archive means deletion, and
  `openspec archive` (which recreates that folder) is not used. Reconsidered from the
  earlier one-step stance ("sync removes the directory; there is no archive"), to make a
  change's closure a distinct gate before its pull request.
- **Definition of Done is single-sourced in `AGENTS.md`.** `README.md` and
  `docs/development-flow.md` point to it rather than restating a divergent subset.
- **Attestation shape — resolved.** A closed two-variant enum `Attestation<Seal>`:
  `Create` (the `Seal` is new) or `Attach(Seal)` (already witnessed — carries the
  presented `Seal` by value, nothing else). Deliberately **not** `#[non_exhaustive]`:
  the verdict space is finite by design, so a new outcome should force a deliberate
  breaking change rather than silently widen the surface.
- **Fingerprint ownership — resolved: core-owned bytes, domain-produced.** The domain
  produces the content-hash bytes; the core owns the `Fingerprint` type
  (`Fingerprint(Box<[u8]>)`) and compares them byte-for-byte. This keeps
  `shaahid-contract` dependency-free *without* the core computing the hash — a `Seal`
  is the domain's meaning, a `Fingerprint`'s bytes are the core's mechanism.
- **Contradiction taxonomy and report — resolved.** Two mechanical anomalies:
  `Contradiction::DriftedFingerprint { witnessed_index }` and
  `Contradiction::SplitSeal { witnessed_index }`, each naming the conflicting witnessed
  `Deed` by index. `witness` returns an `Outcome { attestation, contradictions }` where
  `contradictions` is a `Vec` — the two axes are orthogonal; the core owns no response.
- **Ledger state model — resolved: purely functional.** `witness(witnessed:
  &[Deed<Seal>], incoming: Deed<Seal>) -> Outcome<Seal>` is a pure function of one
  call's inputs, holds no cross-witness state, and compares incoming-versus-each-
  witnessed (it does not audit the witnessed ledger against itself). Durable `Ledger`
  persistence stays downstream.
- **Composition demonstrated; no `Outcome` consumption read — resolved.** A composition
  example (`crates/shaahid-contract/examples/adjudicate_ledger.rs`) is the first real
  consumer: it holds its own ledger and turns each `Outcome` into a disposition — record a
  clean `Create`, deduplicate a clean `Attach`, quarantine any `Contradiction` — over the
  public API alone, driving four trajectories (create / attach / drift / split). Building
  it settled whether the core owes a consumer a mechanical read on `Outcome` (e.g.
  `is_clean`/`has_contradictions`): **no.** `Outcome` carries a single `contradictions`
  collection, so `contradictions.is_empty()` has no compound-read trap (there is no second
  collection a consumer could forget), and the `attestation` axis is a separate,
  unavoidable match; a wrapper would add surface without closing a failure. The consumer
  opting into `Clone` to retain a deed past the value-consuming `witness` is a consumer
  choice, not a core imposition — the core still bounds `Seal` by `Eq` alone. Disposition
  (including quarantine winning over an `Attach`-with-drift) is edge policy in the
  consumer's loop body; the core admits and responds to nothing.
- **`Fingerprint` name — kept (0.1.0 freeze).** The name evokes a content hash and so
  invites the core to grow hashing responsibilities (compute the hash, tag an algorithm,
  enforce a length). Renaming to a lower-gravity term before the freeze was considered and
  rejected: the risk is already contained (the contract forbids a hashing dependency and
  mandates domain-produced bytes), `Fingerprint` is canonical witness vocabulary with a
  broad rename cost, and the semver freeze itself turns any future "the core hashes" into a
  breaking change. The residual pull is closed by stating the non-goals explicitly (no
  hash, no algorithm tag, no length) rather than by a rename. See the `freeze-public-surface`
  change and `adjudication-contract`.
- **`Outcome` name and shape — kept, no accessors.** `Outcome` is a generic container name,
  the kind that attracts convenience accessors. Renaming it, and adding
  `is_clean`/`has_contradictions`, were both considered and rejected: the accessor question
  was already settled just above (a single `contradictions` collection has no compound-read
  trap, and the `attestation` match is unavoidable), and a rename disturbs a settled surface
  for little gain. The generic-name accretion risk is held by the facade's re-export-only
  governance plus this record.
- **`Attestation` and `Contradiction` exhaustiveness — kept closed.** Both stay closed
  enums that are not `#[non_exhaustive]`, so a new variant is a deliberate breaking change.
  Previously only `Attestation` documented this; `Contradiction` — the axis likelier to
  grow — now carries the same commitment in its doc and in `adjudication-contract`. Marking
  `Contradiction` `#[non_exhaustive]` to allow additive anomaly variants was rejected: it
  would let "what counts as a contradiction" widen silently, the exact ratchet the design
  resists.

## Dispositions

Not-now comes in three kinds, kept separate because each says something different about what
can reopen it. Filing them under one word ("deferred") hides that most of these are in fact
*settled*.

### Rejected — never core

Refused at the core-identity level. No future composer can overturn these short of
redefining the product; nothing about them is deferred.

- Any semantic comparison inside the core — meaning is the domain's.
- The core computing a `Fingerprint` hash, tagging an algorithm, or enforcing a length.
- Convenience accessors on `Outcome` (`is_clean`/`has_contradictions`).
- Widening `Attestation` or `Contradiction` behind `#[non_exhaustive]`.

### Downstream — decided, not core

The core's answer is settled — *not here, in the driver or composer* — and cannot be
overturned; only a downstream demonstration is still unbuilt.

- The durable `Ledger` and any `Contradiction` response policy (reject / quarantine /
  escalate). The position is fixed downstream; what is pending is only a downstream example,
  not a core decision.

### Deferred — disposition open

May or may not enter the core; even its ownership is undecided. Held open only by refusing
to freeze — the shape is not fixed until a real composer proves it. A real composer
appearing is what reopens it; both the shape and whether it belongs in the core are deferred.

- An async core variant, until a real driver forces it.

## Prioritization

Prefer changes that preserve thinness and strengthen governance:

1. Protect the adjudication core and the witness vocabulary.
2. Keep semantic identity domain-supplied.
3. Add behavior only as a governed pattern on a named surface.
4. Reject downstream concerns (durable `Ledger`, contradiction policy) leaking into
   the core.
