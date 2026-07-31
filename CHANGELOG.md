# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-07-18

An identity-and-governance release. No change to the public API surface: the same items
are exported and every behaviour is unchanged. The work sharpens Shaahid's stated
positioning and makes the facade's completeness invariant structural.

### Changed

- **The facade re-exports the core surface by glob** (`pub use shaahid_contract::*`), so
  "the facade withholds nothing" is enforced by the compiler rather than by a
  hand-maintained name list — a new `shaahid-contract` public item now appears
  automatically and none can be silently withheld. The exported surface is identical to
  before (`Fingerprint`, `Deed`, `Attestation`, `Contradiction`, `Outcome`, `witness`).

### Documentation

- **Self-positioning reclaimed.** The governing docs (`AGENTS.md`, `PROJECT.md`,
  `README.md`, `docs/domain-language.md`) and the crates' rustdoc now justify each non-goal
  from the pattern's own nature — a sans-I/O adjudication that owns no durable state cannot
  persist a `Ledger`; an alarm that makes no judgment cannot own a response — rather than by
  deferring to a downstream "consumer". The service-word "consumer" gives way to composition
  language; "downstream" is kept only as architectural direction.
- **Contradiction taxonomy proven exhaustive.** `BACKLOG.md` records that
  `DriftedFingerprint` and `SplitSeal` exhaust the mechanically-detectable, currently-silent
  structural facts a witness can raise: the per-witness `Seal`/`Fingerprint` grid has no
  third contradiction cell, and the residual silent-failure surface is irreducible without
  judging meaning — the contradiction layer is the complete theorem of the vision.

## [0.1.0] - 2026-07-14

### Added

- **The witness core** (`shaahid-contract`): `witness` adjudicates an incoming `Deed`
  against a witnessed set, returning an `Outcome` — a create-or-attach `Attestation`
  decided by `Seal` equality, plus every structural `Contradiction` (a drifted
  `Fingerprint` under a repeated `Seal`, or a split `Seal` under a repeated `Fingerprint`)
  in ascending witnessed index. It is a pure function: it holds no state, reads no clock,
  performs no I/O, and decides no admission.
- **Adjudication vocabulary**: `Deed`, `Fingerprint`, `Attestation`, `Contradiction`, and
  `Outcome`. `Seal` is a domain type parameter bounded by value-equality alone; a
  `Fingerprint` is core-owned canonical bytes, domain-produced and compared byte-for-byte.
- **Curated facade** (`shaahid`): the recommended single entrypoint — a pure re-export of
  the public surface, carrying no logic of its own. Its crate-root doctest witnesses a
  create-then-attach through the public API; `crates/shaahid/tests/idempotency_gate.rs`
  drives the four-trajectory (create / attach / drifted-fingerprint / split-seal)
  idempotency-gate demonstration. This retires the former `shaahid-contract`
  `examples/adjudicate_ledger.rs`: the composition proof now lives on the facade, off the
  core crate.
- **Executable governance** (`shaahid-governance`): dependency-isolation, sans-I/O purity
  (no I/O, no ambient clock, no exposed `async fn`), a facade dependency boundary and a
  re-exports-only source tooth, workspace coverage, and active-prose boundaries, each with
  a firing test.

### Design

- **The semantic bill of purity**: the one judgment left to the domain is semantic
  identity (the `Seal`); the core adjudicates by `Seal` equality and compares
  `Fingerprint`s mechanically, never deciding what a deed *means*. A `Contradiction` is an
  observable alarm, not a judgment; the durable `Ledger` and any contradiction response are
  downstream concerns. See `BACKLOG.md`.

[0.1.1]: https://github.com/tacticaldoll/shaahid/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/shaahid/releases/tag/v0.1.0
