# adjudication-contract Specification

## Purpose
The contract for Shaahid's idempotency-adjudication core: what a `Deed` is, how it is
witnessed to a create-or-attach `Attestation` by `Seal` equality alone, the structural
`Contradiction` the core can raise, and the identity vocabulary the domain supplies —
all sans-I/O and free of semantic judgment.

## Requirements
### Requirement: Witness Vocabulary
Shaahid SHALL name the adjudication roles with a fixed witness register: `Deed` (a
unit of work presented to be witnessed), `Seal` (the domain-supplied stable semantic
identity), `Fingerprint` (the mechanical content identity), `Attestation` (the
create-or-attach verdict), `Ledger` (the record of witnessed `Seal`s), `Witness` (the
adjudicator role), and `Contradiction` (a structural `Seal`↔`Fingerprint` anomaly).
These terms are architecture, not branding.

#### Scenario: The vocabulary is canonical
- **WHEN** documentation or the contract crate refers to an adjudication role
- **THEN** it uses the canonical witness term rather than a generic synonym

### Requirement: Adjudication Is Create-Or-Attach
Shaahid SHALL witness a `Deed` against the witnessed `Deed`s and produce an `Outcome`
whose `Attestation` is `Create` (the incoming `Seal` is new to the witnessed set) or
`Attach` (the incoming `Seal` is already witnessed). The entry point SHALL be
`witness(&[Deed<Seal>], Deed<Seal>) -> Outcome<Seal>`. The attestation decision SHALL
be `Seal` equality — mechanical, never a comparison of meaning. Witnessing SHALL be a
**pure function** of the witnessed `Deed`s (the `Ledger`'s content, supplied to the
core as a value at the edge) and the incoming `Deed`: the core SHALL NOT own, persist,
or mutate `Ledger` state, SHALL NOT require a `Registry`-like obligation trait, and
SHALL NOT read an ambient clock or perform I/O. The core SHALL be generic over the
`Seal` type alone, bounding it by value-equality. The prior seal-only entry point
(`adjudicate` over `&[Seal]`) is retracted: a `Seal`-only projection cannot observe a
`Contradiction`.

#### Scenario: A new Seal creates; a witnessed Seal attaches
- **WHEN** a `Deed` is witnessed whose `Seal` is not among the witnessed `Deed`s, then again with that same `Seal` now present among them
- **THEN** the first `Outcome`'s attestation is `Create` and the second's is `Attach` carrying that `Seal`, decided by `Seal` equality alone

#### Scenario: Witnessing is a pure function of supplied state
- **WHEN** the same witnessed `Deed`s and the same incoming `Deed` are witnessed twice
- **THEN** the two `Outcome`s are identical, and the core reads no state beyond its inputs (no ambient clock, no I/O, no core-persisted `Ledger`)

### Requirement: Attestation Is A Closed Create-Or-Attach Enum
`Attestation` SHALL be a closed enum of exactly two variants — `Create` and `Attach`
— and SHALL NOT grow into a DSL or open extension surface. `Create` SHALL denote that
the `Deed`'s `Seal` was new to the witnessed set. `Attach` SHALL denote that the
`Seal` was already witnessed and SHALL carry **only that `Seal`, by value** — the
`Seal` the `Deed` presented, which is equal by value to the already-witnessed one. The
core SHALL hand back the presented `Seal` rather than clone the witnessed element, so
it requires only value-equality of a `Seal`, never `Clone`. `Attach` SHALL NOT carry
any reference or handle into a `Ledger`, a `Deed` store, or any downstream policy or
response.

#### Scenario: The verdict is one of exactly two closed variants
- **WHEN** adjudication produces an `Attestation`
- **THEN** it is either `Create` or `Attach`, with no third outcome and no embedded policy or response

#### Scenario: Attach carries only the presented Seal, by value
- **WHEN** a `Deed` whose `Seal` is already witnessed is adjudicated
- **THEN** the `Attach` variant carries that presented `Seal` (equal by value to the already-witnessed one) and nothing else — no reference into a `Ledger` or `Deed` store, and nothing that would make the core own a response

### Requirement: Witness Produces One Outcome Of Attestation And Contradictions
`witness` SHALL return a single `Outcome<Seal>` carrying the `Attestation` and every
detected `Contradiction`: conceptually `Outcome { attestation: Attestation<Seal>,
contradictions: Vec<Contradiction> }`. Attestation and contradiction SHALL be
independent axes — a single witness MAY be `Attach` with one or more drifts, `Create`
with one or more splits, or both at once. The contradictions SHALL be a list, never a
single optional, because one incoming `Deed` may expose several structural facts at
once. A `Contradiction` SHALL be a non-generic closed enum referring to the conflicting
witnessed `Deed` by its index in the input slice: conceptually `Contradiction {
DriftedFingerprint { witnessed_index }, SplitSeal { witnessed_index } }`. Each
`Contradiction` SHALL refer to exactly one witnessed `Deed` and be meaningful only
relative to that witness invocation. Contradictions SHALL be emitted in ascending
`witnessed_index`; this is a total order because at most one `Contradiction` arises per
witnessed `Deed` — drift requires the `Seal`s to match, split requires them to differ,
which are mutually exclusive.

#### Scenario: Attach co-occurs with a drift
- **WHEN** the incoming `Seal` is witnessed but under a different `Fingerprint`
- **THEN** the `Outcome` is `Attach` AND carries a `DriftedFingerprint` for that witnessed `Deed`

#### Scenario: Create co-occurs with a split
- **WHEN** the incoming `Seal` is new but its `Fingerprint` matches a different witnessed `Seal`'s `Deed`
- **THEN** the `Outcome` is `Create` AND carries a `SplitSeal` for that witnessed `Deed`

#### Scenario: Multiple contradictions surface in ascending index order
- **WHEN** the incoming `Deed` conflicts with several witnessed `Deed`s
- **THEN** every conflict is surfaced as its own `Contradiction`, ordered by ascending `witnessed_index`

#### Scenario: A clean re-witness yields no contradiction
- **WHEN** the incoming `Deed` equals a witnessed `Deed` (same `Seal` and same `Fingerprint`)
- **THEN** the `Outcome` is `Attach` with an empty contradiction list

### Requirement: The Core Makes No Semantic Judgment
Shaahid's core SHALL make no semantic judgment. Semantic identity SHALL be
domain-supplied as a `Seal`; the core's role SHALL be limited to adjudicating by
`Seal` equality and comparing `Fingerprint`s byte-wise, and it SHALL NOT decide
whether two `Deed`s mean the same thing. This is the semantic bill of purity: its
cost — a wrong `Seal` that still matches its `Fingerprint` fails silently — SHALL be
accepted rather than patched by judging meaning.

#### Scenario: Identity is the domain's Seal
- **WHEN** the core needs to know whether two `Deed`s are the same work
- **THEN** it uses their domain-supplied `Seal`s, never a comparison of meaning it performs itself

#### Scenario: The cost is accepted, not patched
- **WHEN** a domain supplies a wrong `Seal` that still matches its `Fingerprint`
- **THEN** the core does not prevent the resulting mis-adjudication by judging meaning; it stays pure, and the residual silent-failure surface is accepted

### Requirement: Seals Are Domain-Supplied And Opaque
A `Seal` SHALL be domain-supplied, stable across witnesses, and changed only on a
genuine semantic change. The `Seal` type SHALL be a **generic parameter** the domain
supplies; the core SHALL carry it opaquely and SHALL NOT interpret it. For
adjudication the core SHALL bound the `Seal` type by **value-equality alone**; it
SHALL NOT require ordering, inspect the `Seal`'s contents, or otherwise interpret its
meaning.

#### Scenario: The same intent keeps its Seal
- **WHEN** the same intent is witnessed again
- **THEN** it carries the same `Seal`, and a genuine semantic change carries a new one

#### Scenario: The core does not interpret the Seal
- **WHEN** the core handles a `Seal`
- **THEN** it treats it as an opaque identity, compared by value, never by meaning

#### Scenario: Adjudication needs only Seal equality
- **WHEN** the core tests whether a `Deed`'s `Seal` is already witnessed
- **THEN** it uses value-equality against the witnessed `Seal`s alone, requiring no ordering or interpretation of the `Seal`

### Requirement: Fingerprints Are Mechanical Content Identity
A `Fingerprint` SHALL be the mechanical content identity of a `Deed`. Shaahid SHALL
**own** its canonical representation as an immutable byte sequence — `Fingerprint` is a
Shaahid-owned newtype over owned bytes (`Box<[u8]>`), constructed once and never mutated
— and SHALL NOT be a domain-supplied generic type. The domain SHALL produce the
fingerprint bytes; Shaahid SHALL own their canonical representation and compare them
byte-for-byte, encoding content rather than meaning. A `Deed` SHALL therefore be generic
over its `Seal` type alone (`Deed<Seal>`) and carry an owned `Fingerprint`.

#### Scenario: Fingerprints compare byte-for-byte
- **WHEN** the core compares two `Fingerprint`s
- **THEN** it compares their bytes directly, making no judgment about what the content means and delegating the comparison to no domain-supplied equality

#### Scenario: The domain produces bytes; Shaahid owns the canonical representation
- **WHEN** a `Deed` is built with a `Fingerprint`
- **THEN** the domain supplied the bytes, Shaahid holds them in its own immutable canonical type (readable back but not mutable), and the core added no hashing dependency

### Requirement: A Fingerprint Is Mandatory
The `witness` entry point SHALL require a full `Deed` — a `Seal` and a `Fingerprint`.
Shaahid SHALL NOT provide a seal-only witness path. A use case with no `Fingerprint` is
not a degraded use of Shaahid but a plain identity set, and is out of scope for the
core.

#### Scenario: A bare Seal cannot be witnessed
- **WHEN** a caller wishes to witness work
- **THEN** the only entry point takes a `Deed` bearing both a `Seal` and a `Fingerprint`, with no seal-only alternative

### Requirement: Contradiction Is A Structural Alarm
Shaahid SHALL detect a structural `Contradiction` mechanically while witnessing and
surface it in the `Outcome`: a **drifted `Fingerprint`** (a witnessed `Deed` with the
same `Seal` as the incoming but a different `Fingerprint`) or a **split `Seal`** (a
witnessed `Deed` with the same `Fingerprint` as the incoming but a different `Seal`). A
`Contradiction` SHALL NOT be a judgment that the domain was wrong, and the core SHALL
own no response to it.

#### Scenario: A drifted Fingerprint under a repeated Seal is a Contradiction
- **WHEN** the incoming `Deed`'s `Seal` matches a witnessed `Deed`'s `Seal` but their `Fingerprint`s differ
- **THEN** the `Outcome` surfaces a `DriftedFingerprint` `Contradiction` for that witnessed `Deed`, without deciding the domain was wrong

#### Scenario: A split Seal under a repeated Fingerprint is a Contradiction
- **WHEN** the incoming `Deed`'s `Fingerprint` matches a witnessed `Deed`'s `Fingerprint` but their `Seal`s differ
- **THEN** the `Outcome` surfaces a `SplitSeal` `Contradiction` for that witnessed `Deed`

#### Scenario: The core owns no response
- **WHEN** a `Contradiction` is surfaced
- **THEN** what to do about it (reject, quarantine, escalate) is a downstream concern, not the core's

### Requirement: Witnessing Is Per-Witness And Decides No Admission
Witnessing SHALL compare the incoming `Deed` only against each witnessed `Deed`; it
SHALL NOT audit `Contradiction`s internal to the witnessed `Ledger`. The witnessed
`Ledger` SHALL be passed as a read-only `&[Deed<Seal>]`; the core SHALL NOT modify it
and SHALL NOT decide whether the incoming `Deed` is admitted. Witness observes both
attachment and contradiction; it does not decide admission — whether to record the
incoming `Deed` (even a contradictory one) is a downstream `Ledger`/driver concern.

#### Scenario: Scope is incoming-versus-witnessed, not within-ledger
- **WHEN** the incoming `Deed` drifts against two witnessed `Deed`s that are also inconsistent with each other
- **THEN** the two incoming-versus-witnessed drifts are surfaced, and the pre-existing witnessed-versus-witnessed inconsistency is not itself reported

#### Scenario: Read-only, no admission decision
- **WHEN** any `Deed` is witnessed
- **THEN** the witnessed slice is not mutated and the `Outcome` carries no decision about admitting the incoming `Deed` into the `Ledger`

### Requirement: Sans-I/O Purity
The adjudication core SHALL be sans-I/O: it SHALL expose no `async fn`, read no
ambient clock, and perform no I/O. A runtime drives it and supplies the witnessed
state at the edge.

#### Scenario: The core commits to no runtime shape
- **WHEN** the `shaahid-contract` public API is compiled
- **THEN** it exposes no `async fn`, calls no `std::io`/`fs`/`net`/`process`, and reads no ambient clock

### Requirement: Dependency Isolation
`shaahid-contract` SHALL depend on no other workspace crate, so the adjudication core
stays isolated and reusable.

#### Scenario: The core is isolated
- **WHEN** `shaahid-contract`'s manifest is read
- **THEN** it declares no dependency on another workspace crate

