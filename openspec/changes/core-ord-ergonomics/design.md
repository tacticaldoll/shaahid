## Context

Same field graph as `core-derive-ergonomics`:

```
Fingerprint (owns Box<[u8]>)
Contradiction (owns usize, non-generic)
Deed<Seal>        { seal: Seal, fingerprint: Fingerprint }
Attestation<Seal> { Create | Attach(Seal) }
Outcome<Seal>     { attestation: Attestation<Seal>, contradictions: Vec<Contradiction> }
```

`#[derive(PartialOrd, Ord)]` requires every field type to already implement the trait,
plus (for `Ord`) `Eq` as a supertrait — already present on every type here. As with the
prior `Hash` change, the leaf types must gain the derive before the types that contain
them.

## Goals / Non-Goals

**Goals:**
- Let a composing system use `Fingerprint`, `Contradiction`, `Attestation<Seal>`,
  `Deed<Seal>`, and `Outcome<Seal>` as `BTreeMap`/`BTreeSet` keys wherever `Seal: Ord`
  already holds on their own type — including in a `std`-less `no_std + alloc`
  environment, where `BTreeMap`/`BTreeSet` are the only sorted-collection option.
- Keep the derived order purely mechanical, with no claim that it is domain-meaningful.

**Non-Goals:**
- No `Ord` (or any new trait) required of `Seal` itself for adjudication — `witness`
  keeps bounding `Seal` by `Eq` alone, per `adjudication-contract`'s existing "Seals Are
  Domain-Supplied And Opaque" requirement, which governs the adjudication function, not
  an opt-in derive on the surrounding container types.
- No claim that `Contradiction`'s derived variant order (`DriftedFingerprint` before
  `SplitSeal`, from declaration order) means anything about severity or precedence — it
  exists only to satisfy `Ord`'s total-order contract for collection use.
- No change to `witness`'s emission order (already ascending `witnessed_index`, decided
  by the adjudication algorithm, not by `Contradiction: Ord`).

## Decisions

- **Derive, not hand-written impls** — same rationale as `Hash`: consistency with the
  already-derived `PartialEq`/`Eq` (Rust requires `Ord`'s comparison to agree with `Eq`;
  a hand-written impl risks drifting from that, a derived one cannot when derived from
  the same field order).
- **`PartialOrd` and `Ord` together, not `PartialOrd` alone** — every field type here
  already has a total order (`Box<[u8]>`, `usize`, and whatever `Ord` the caller's
  `Seal` supplies), so there is no partial-only case to preserve; shipping only
  `PartialOrd` would withhold `Ord` for no reason and block `BTreeMap`/`BTreeSet`
  specifically (they require `Ord`, not just `PartialOrd`).
- **Same bottom-up scope as `Hash`** (`Fingerprint`/`Contradiction` unconditional,
  `Attestation`/`Deed`/`Outcome` conditional on `Seal`) — reusing an already-reviewed
  shape rather than inventing a new one.

## Risks / Trade-offs

- **Derive macro's blanket `Seal: Ord` bound is coarser than strictly necessary** (e.g.
  `Attestation::Create` never touches `Seal`) → Mitigation: identical, already-accepted
  shape as the `Hash` change; not a new inconsistency.
- **A reader could mistake the derived order for a domain-meaningful ranking** (e.g.
  assume `SplitSeal > DriftedFingerprint` means something) → Mitigation: state the
  non-goal explicitly in the delta spec and in a doc comment at the derive site, mirroring
  how `Fingerprint`'s docs already disclaim non-goals explicitly rather than relying on
  silence.
