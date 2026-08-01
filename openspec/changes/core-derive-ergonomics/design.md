## Context

`shaahid-contract`'s public types form a small field graph:

```
Fingerprint (owns Box<[u8]>)
Contradiction (owns usize, non-generic)
Deed<Seal>        { seal: Seal, fingerprint: Fingerprint }
Attestation<Seal> { Create | Attach(Seal) }
Outcome<Seal>     { attestation: Attestation<Seal>, contradictions: Vec<Contradiction> }
```

`#[derive(Hash)]` on a struct/enum requires every field type to already implement
`Hash` (plus, for a generic type, the derive macro adds a `Seal: Hash` bound on the
type parameter itself, whether or not every variant actually uses it — the same
blanket-bound behavior the existing `Eq`/`PartialEq`/`Clone` derives already accept on
these types). So the leaf types (`Fingerprint`, `Contradiction`) must gain `Hash` before
the types that contain them (`Deed`, `Attestation`, `Outcome`) can.

## Goals / Non-Goals

**Goals:**
- Let a composing system use `Fingerprint`, `Contradiction`, `Attestation<Seal>`,
  `Deed<Seal>`, and `Outcome<Seal>` as `HashMap`/`HashSet` keys wherever `Seal: Hash`
  already holds on their own type.
- Give `Contradiction` a human-readable `Display` for logging, independent of the `Hash`
  work.
- Touch nothing about `witness`'s behavior, the sans-I/O/no-`std` boundary, or any
  existing trait already derived.

**Non-Goals:**
- No `Hash` (or any new trait) on `Seal` itself — `Seal` stays domain-supplied and
  bounded only by what each derive naturally requires (`Eq` for adjudication, `Hash`
  only when a caller opts into hashing a type that carries a `Seal`).
- No `Display` for `Attestation<Seal>` or `Outcome<Seal>` in this change — both carry or
  aggregate a `Seal`/`Contradiction` whose own `Display` is not guaranteed (`Seal` is
  domain-supplied and unbounded by default), so a conditional `Seal: Display` impl is a
  separate, larger surface than the non-generic `Contradiction` case the proposal
  motivated. Deferred rather than scope-crept into this change.
- No change to `Eq`/`Ord` semantics or to hashing consistency guarantees beyond what
  `#[derive(Hash)]` gives for free (it is defined to agree with the derived `PartialEq`/
  `Eq` field-for-field, which these types already have).

## Decisions

- **Derive, not hand-written impls.** Every new `Hash` impl is `#[derive(Hash)]`. A
  hand-written impl would risk drifting from the derived `Eq` (the `Hash`/`Eq`
  consistency invariant: equal values must hash equal) with no benefit here — all
  fields are already the natural hash inputs. Consistent with how `Debug`/`Clone`/
  `PartialEq`/`Eq` are already derived, not hand-written, on these same types.
- **`Contradiction` gains both `Hash` and `Display` in the same change.** They are
  independent traits touching the same small enum; splitting them into two changes
  would double the propose/apply/sync/archive overhead for no isolation benefit (the
  two impls do not interact and cannot conflict).
- **`Display` message shape for `Contradiction`.** One line per variant, naming the
  variant and its `witnessed_index`, e.g. `drifted fingerprint against witnessed deed at
  index 3`. No punctuation-sensitive machine format is implied by the requirement — this
  is a log/debug aid, not a serialization contract, so the exact wording is an
  implementation-level test fixture, not a spec-pinned string (the delta spec asserts
  that a message exists and names the index, not its literal text).
- **Where the new tests live.** Alongside the existing `#[cfg(test)] mod tests` in
  `crates/shaahid-contract/src/lib.rs`, following the file's existing convention (no new
  test file, no new crate dependency).

## Risks / Trade-offs

- **Derive macro's blanket `Seal: Hash` bound is coarser than strictly necessary** (e.g.
  `Attestation::Create` never touches `Seal` at all, yet the derive still requires
  `Seal: Hash` to hash a `Create` value) → Mitigation: this is the same shape of
  over-bounding the existing derives on these types already accept (e.g. `Clone` on
  `Attestation<Seal>` requires `Seal: Clone` even for the `Create` arm); it is Rust's
  standard derive behavior, not a new inconsistency this change introduces, and callers
  that never need to hash simply never add the bound.
- **Scope creep risk** (adding `Hash` to `Attestation`/`Contradiction` beyond the
  proposal's original `Fingerprint`/`Outcome` motivation) → Mitigation: both are
  mechanically required for `Outcome<Seal>: Hash` to compile at all, not speculative
  additions; the proposal has been updated to state this plainly.

## Migration Plan

Not applicable — purely additive trait implementations, no existing API removed or
changed, no data migration, no feature flag. Ships in the next release's Added section.

## Open Questions

None outstanding for this change's scope.
