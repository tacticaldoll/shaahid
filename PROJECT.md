# Project Contract

## Vision

Shaahid is a thin, sans-I/O idempotency-adjudication core for Rust: given a `Deed`
(a domain-supplied `Seal` and a content `Fingerprint`) and the witnessed state, it
adjudicates **create-or-attach** and detects structural contradictions — while making
no semantic judgment of its own.

It fills a narrow gap: the thinnest useful primitive that answers "have we already
witnessed this work?" without becoming a durable store, a deduplicator that guesses
identity, or a workflow engine. Shaahid witnesses and attests; the meaning of "the
same work" is the domain's, supplied as a `Seal`.

## Product Positioning

Shaahid is for systems that must not act on the same intent twice and want that
guarded honestly:

```text
Deed (Seal + Fingerprint) -> Witness -> Attestation (Create | Attach)
                                     \-> Contradiction? (Seal <-> Fingerprint mismatch)
```

The promise is not "a dedup cache with batteries". It is a clean, pure place to
attach:

- domain-supplied semantic identity (`Seal`)
- mechanical content identity (`Fingerprint`) the core compares byte-wise
- downstream durability of the `Ledger`, and any policy on a detected contradiction
  — consumer concerns, not Shaahid

## Core Contract

The behavior that must be protected first:

- **Thin adjudication core**: `shaahid-contract` owns the create-or-attach decision
  (by `Seal` equality) and structural contradiction detection (a `Seal` whose
  `Fingerprint` drifted, or a `Fingerprint` whose `Seal` split). It does not own the
  durable `Ledger`, retries, or any policy on a contradiction.
- **No semantic judgment in the core — the semantic bill of purity**: a sans-I/O pure
  core cannot decide whether two `Deed`s *mean* the same thing. Semantic identity is
  domain-supplied as a `Seal`; Shaahid only adjudicates by `Seal` equality and
  compares `Fingerprint`s mechanically. Its defense is to make a `Seal`↔`Fingerprint`
  contradiction **observable**, never to judge whether the domain *should* have used
  that `Seal`. A wrong `Seal` that still matches its `Fingerprint` is a silent failure
  — the deliberate cost of purity (see `BACKLOG.md`).
- **Sans-I/O purity**: the core exposes no `async fn`, reads no ambient clock, and
  performs no I/O. A runtime drives it and supplies the witnessed state at the edge.
- **Governance with teeth**: Tianheng and project specs enforce the boundaries prose
  claims — with the honest exception that "no semantic judgment" is not statically
  expressible (see BACKLOG).

## Elegance

Elegance in Shaahid is technical restraint:

- one owned decision (create-or-attach) plus one mechanical alarm (contradiction);
  every semantic judgment outsourced
- precise witness vocabulary (`Deed`, `Seal`, `Fingerprint`, `Attestation`, `Ledger`,
  `Witness`, `Contradiction`)
- domain-owned meaning
- small composable interfaces
- executable governance against architectural drift

## Non-Goals

Shaahid core is not:

- a durable store (the `Ledger`'s persistence is a consumer concern)
- a deduplicator that guesses identity (identity is domain-supplied, never inferred)
- a semantic-comparison engine (it never decides what two `Deed`s mean)
- a workflow or orchestration engine
- a policy owner for contradictions (it raises the alarm; the response is downstream)

## References

- Canonical shipped requirements: `openspec/specs/`
- Active proposed requirements: `openspec/changes/`
- Domain language: `docs/domain-language.md`
- Deferred decisions and open design questions: `BACKLOG.md`
