## Why

`shaahid-contract`'s identity types (`Fingerprint`, `Deed<Seal>`, `Outcome<Seal>`) derive
only `Debug, Clone, PartialEq, Eq`. A composing system that wants to key a `HashMap` or
`HashSet` on a witnessed `Fingerprint` or `Outcome` — for example a dedup cache keyed by
content identity — cannot, because none of them derive `Hash`. Separately,
`Contradiction` has no `core::fmt::Display`, so a composing system can only log it via
`{:?}`, which is harder to read in an operator-facing trace than a purpose-written
message would be. Both gaps are additive: filling them changes no existing behavior and
opens no new axiom risk.

## What Changes

- Derive `Hash` on the identity types, bottom-up through the existing field structure:
  `Fingerprint` and `Contradiction` unconditionally (both own plain leaf data), and
  `Attestation<Seal>`, `Deed<Seal>`, and `Outcome<Seal>` conditionally on `Seal: Hash`
  (mirroring the existing conditional-on-`Seal` pattern already used for
  `Debug`/`Clone`/`PartialEq`/`Eq`). `Outcome<Seal>` and `Deed<Seal>` need their field
  types (`Attestation<Seal>`, `Vec<Contradiction>`, `Fingerprint`) to already be `Hash`,
  which is why `Attestation` and `Contradiction` are included even though the original
  motivation was keying on `Fingerprint`/`Outcome`.
- Implement `core::fmt::Display` for `Contradiction` (non-generic, so no `Seal` bound is
  needed) with a one-line, human-readable message per variant naming the
  `witnessed_index`.
- No change to `witness`'s behavior, to any existing derive, or to the public API shape
  otherwise — this only adds trait implementations.
- No change to the `shaahid` facade source (`pub use shaahid_contract::*` picks up the
  new impls automatically; the facade re-exports-only governance reaction is unaffected
  because no new item is declared in the facade itself).

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `adjudication-contract`: adds two requirements — that `Fingerprint`, `Contradiction`,
  `Attestation<Seal>` (`Seal: Hash`), `Deed<Seal>` (`Seal: Hash`), and `Outcome<Seal>`
  (`Seal: Hash`) implement `Hash`; and that `Contradiction` implements `Display`. These
  are additive guarantees about the existing types, not a change to adjudication
  behavior.

## Impact

- Code: `crates/shaahid-contract/src/lib.rs` only (derive attributes plus one `Display`
  impl and its tests).
- API: purely additive — new trait implementations on existing public types. No
  existing signature, derive, or behavior changes; not a breaking change.
- Dependencies: none added (`Hash` and `core::fmt::Display` are both in `core`/`alloc`,
  already available to a `no_std + alloc` crate).
- Governance: no crate-boundary, sans-I/O, or facade-reexport reaction is affected —
  confirmed by reading `shaahid-governance`'s constitution before scoping this change.
