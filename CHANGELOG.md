# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2026-08-01

A derive-and-composition release. No breaking change to the public API surface: every
change here adds a new trait implementation to an already-published type, or improves a
test's fidelity. No existing signature, derive, or behavior changed.

### Added

- **`Hash` on the identity types**: `Fingerprint` and `Contradiction` implement `Hash`
  unconditionally; `Deed<Seal>`, `Attestation<Seal>`, and `Outcome<Seal>` implement it
  whenever `Seal: Hash`. Lets a composing system key a `HashMap`/`HashSet` on these types.
- **`Display` on `Contradiction`**: a human-readable, one-line message per variant naming
  the variant kind and the conflicting `witnessed_index`, independent of the existing
  `Debug`. Intended for operator-facing logs, not a machine-parseable format.
- **`PartialOrd`/`Ord` on the identity types**: the same shape as `Hash` above —
  `Fingerprint` and `Contradiction` unconditionally, `Deed<Seal>`/`Attestation<Seal>`/
  `Outcome<Seal>` whenever `Seal` implements it. Unlike `Hash`, this also lets a composing
  system key an `alloc`-only sorted collection (`BTreeMap`/`BTreeSet`) with no `std` —
  the only sorted-collection option shaahid-contract's own no_std + alloc + MSRV
  1.88 thumbv7em-none-eabi target can use. The derived order is mechanical (field/variant
  declaration order) and carries no claim about severity, precedence, or correctness.

### Changed

- **The facade's composition test now demonstrates `Contradiction`'s `Display`**: the
  quarantine branch of `idempotency_gate_resolves_four_trajectories` used to compute and
  discard a `(kind, index)` pair per contradiction; it now captures and asserts on the
  `Display` message instead, so the new capability is proven to compose through the
  `shaahid` facade, not only in `shaahid-contract`'s own unit tests.

## [0.1.2] - 2026-07-30

A portability-and-governance release. No change to the public API surface: `Box` and
`Vec` in the exported types now resolve through `alloc` rather than an implicit `std`
import, but these are the identical items `std` re-exports — no consumer-visible type,
signature, or behavior changes, and no `Cargo.toml` feature was added or removed.

### Added

- **Unconditional `no_std + alloc` portability**: `shaahid-contract` and `shaahid` compile
  without `std` on any target with an allocator, verified by an MSRV-pinned
  `thumbv7em-none-eabi` compile check in CI and the Definition of Done. Not a
  no-allocation claim — `Fingerprint` keeps its owned boxed bytes, `Outcome` keeps its
  contradiction vector, and the composing system supplies the allocator. Allocators,
  runtimes, target adapters, async, storage, serialization, and facade topology remain
  outside this capability.

### Changed

- **Governance upgraded to Tianheng 0.3.0**, adopting its composed capabilities: one
  Constitution feeds the runner, workspace-coverage assertion, structured negative
  proofs, and a freshness-gated `AGENTS.shaahid-law.md` projection. The clock-free and
  synchronous-API laws are now expressed through `SansIoPure`, with the explicit
  `std::io`/`fs`/`net`/`process` boundaries retained. No semantic, runtime, or API policy
  was added; the published crates and their surface are unchanged.

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

[0.1.3]: https://github.com/tacticaldoll/shaahid/releases/tag/v0.1.3
[0.1.2]: https://github.com/tacticaldoll/shaahid/releases/tag/v0.1.2
[0.1.1]: https://github.com/tacticaldoll/shaahid/releases/tag/v0.1.1
[0.1.0]: https://github.com/tacticaldoll/shaahid/releases/tag/v0.1.0
