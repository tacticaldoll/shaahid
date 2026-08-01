## Why

`core-derive-ergonomics` added `Hash` to `shaahid-contract`'s identity types so a
composing system could key a `HashMap`/`HashSet` on them. But `shaahid-contract` is an
unconditional `no_std + alloc` crate, verified against a real embedded target
(`thumbv7em-none-eabi`) with no `std` at all. `std::collections::HashMap`/`HashSet` are
not available in that environment — there is no `std`, and no default hasher without
it. `alloc::collections::BTreeMap`/`BTreeSet`, by contrast, need only `Ord` and are
available in `alloc` alone. For the crate's actual bare-metal audience, `Ord` — not
`Hash` — is what makes these types usable as a sorted-collection key at all.

## What Changes

- Derive `PartialOrd, Ord` on the same types and in the same bottom-up shape as the
  prior `Hash` change: `Fingerprint` and `Contradiction` unconditionally, then
  `Attestation<Seal>`, `Deed<Seal>`, and `Outcome<Seal>` conditionally on
  `Seal: PartialOrd`/`Seal: Ord` respectively.
- The derived order is mechanical (field/variant declaration order), not a claim about
  domain meaning — same stance already taken for the derived `Hash`: these types don't
  need their ordering to *mean* anything to satisfy `BTreeMap`/`BTreeSet`'s contract.
- No change to `witness`'s behavior or the `Seal` bound it requires (`Eq` alone, per
  `adjudication-contract`'s "Seals Are Domain-Supplied And Opaque" requirement) — that
  requirement governs the adjudication function itself, not an optional derive on the
  container types, exactly as already established for `Hash`.
- No change to any existing derive/signature. Purely additive; not a breaking change.

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `adjudication-contract`: adds one requirement — that `Fingerprint` and
  `Contradiction` implement `PartialOrd`/`Ord` unconditionally, and `Attestation<Seal>`,
  `Deed<Seal>`, and `Outcome<Seal>` implement them whenever `Seal` does, so a composing
  system can key an `alloc`-only sorted collection (`BTreeMap`/`BTreeSet`) on these
  types without requiring `std`.

## Impact

- Code: `crates/shaahid-contract/src/lib.rs` only.
- API: purely additive — new trait implementations on existing public types. Not a
  breaking change.
- Dependencies: none (`Ord`/`PartialOrd` are `core` traits).
- Governance: no crate-boundary, sans-I/O, or facade-reexport reaction is affected.
