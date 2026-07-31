# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_0.1.0 is being prepared; it has not yet been published to crates.io._

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
- **Composition example** (`examples/adjudicate_ledger.rs`): an idempotency-gate consumer
  that holds its own ledger and disposes of each `Outcome` — record, deduplicate, or
  quarantine — over the public API, demonstrating the create, attach, drift, and split
  trajectories.
- **Executable governance** (`shaahid-governance`): dependency-isolation, sans-I/O purity
  (no I/O, no ambient clock, no exposed `async fn`), workspace coverage, and active-prose
  boundaries, each with a firing test.

### Design

- **The semantic bill of purity**: the one judgment left to the domain is semantic
  identity (the `Seal`); the core adjudicates by `Seal` equality and compares
  `Fingerprint`s mechanically, never deciding what a deed *means*. A `Contradiction` is an
  observable alarm, not a judgment; the durable `Ledger` and any contradiction response are
  downstream concerns. See `BACKLOG.md`.
