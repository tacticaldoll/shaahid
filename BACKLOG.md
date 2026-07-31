# Backlog & Deferred Decisions

This file records deferred decisions, open design questions, and candidate patterns.
It is not a phase roadmap and creates no implementation commitments. Shipped truth
lives in `openspec/specs/`; active proposed truth lives in `openspec/changes/`.

## Current Baseline

- The **initial project shape** is established: vision and non-goals (`PROJECT.md`),
  operating protocol and axioms (`AGENTS.md`), witness domain language
  (`docs/domain-language.md`), executable governance (`shaahid-governance`), and a
  compiling crate skeleton (`shaahid-contract`).
- The adjudication **core is not yet implemented**. Its intended contract lives in
  `openspec/specs/`; implementation follows in later spec-driven changes,
  deliberately, because the design still has open questions (below).

## Workspace Composition

The workspace stays thin. It owns the adjudication contract (`shaahid-contract`) and
its governance gate (`shaahid-governance`, unpublished). The durable `Ledger` and any
policy on a `Contradiction` are **downstream consumer concerns** and live outside this
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
compares meanings. Do not freeze a user-obligation trait ahead of its first consumer.

- **Attestation shape.** `Create` vs `Attach` is the minimum; what does `Attach`
  return — a handle to the already-witnessed `Deed`, its `Attestation`, nothing? Keep
  it a small closed enum, not a DSL. Not yet designed.
- **Fingerprint ownership.** Does the core *receive* a domain-computed `Fingerprint`
  (keeping `shaahid-contract` dependency-free) or *compute* it with a pure hasher
  (mechanical, but a dependency)? Hashing is pure and sans-I/O, so either is
  axiom-safe; leaning toward domain-supplied to keep the core dep-free. Open.
- **Contradiction taxonomy and report.** The two mechanical anomalies (drifted
  `Fingerprint`, split `Seal`) plus the shape of the report (what it carries, how it
  is surfaced without the core owning a response). Not yet designed.
- **Ledger state model.** Whether the core is purely functional (given the witnessed
  set and a `Deed`, return an `Attestation`) or tracks `Ledger` state across witnesses,
  and if the latter, where that state lives without pulling I/O into the core.
- **The unenforceable purity invariant.** "The core makes no semantic judgment" is
  not statically expressible — semantic comparison has no syntactic marker, so Tianheng
  cannot bite it the way it bites no-I/O or no-async. It stays review- and
  structure-governed. `Contradiction` detection narrows the silent-failure surface but
  does not enforce the invariant.
- **Async variant.** Deferred until a real driver forces it; the sans-I/O core is
  agnostic to sync/async at the edge.

## Recorded Reconsiderations (inherited discipline)

- **No architecture-decision-record files.** Decision provenance lives in git commit
  bodies and pull requests; reconsiderations live here; the living docs are the single
  source of truth for current state. The starter's `docs/adr/` was removed on birth.
- **No OpenSpec change archive.** Sync promotes delta specs into `openspec/specs/` and
  removes the change directory; git retains the deliberation. `openspec archive`
  recreates `openspec/changes/archive/` — remove it after each sync.
- **Definition of Done is single-sourced in `AGENTS.md`.** `README.md` and
  `docs/development-flow.md` point to it rather than restating a divergent subset.

## Explicitly Deferred

- Implementation of the adjudication and contradiction detection (spec-driven, later).
- Any semantic comparison inside the core (never — meaning is the domain's).
- The durable `Ledger` and any `Contradiction` response policy (downstream).
- An async core variant (until a driver forces it).

## Prioritization

Prefer changes that preserve thinness and strengthen governance:

1. Protect the adjudication core and the witness vocabulary.
2. Keep semantic identity domain-supplied.
3. Add behavior only as a governed pattern on a named surface.
4. Reject downstream concerns (durable `Ledger`, contradiction policy) leaking into
   the core.
