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

- **The unenforceable purity invariant.** "The core makes no semantic judgment" is
  not statically expressible — semantic comparison has no syntactic marker, so Tianheng
  cannot bite it the way it bites no-I/O or no-async. It stays review- and
  structure-governed. `Contradiction` detection narrows the silent-failure surface but
  does not enforce the invariant.
- **Async variant.** Deferred until a real driver forces it; the sans-I/O core is
  agnostic to sync/async at the edge.

## Recorded Reconsiderations

Inherited discipline first, then this project's own resolved design decisions.


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

## Explicitly Deferred

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
