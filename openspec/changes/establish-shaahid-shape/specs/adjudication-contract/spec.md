## ADDED Requirements

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
Shaahid SHALL adjudicate a witnessed `Deed` to an `Attestation` of `Create` (its
`Seal` is new to the `Ledger`) or `Attach` (its `Seal` was already witnessed). The
decision SHALL be `Seal` equality — mechanical, never a comparison of meaning. This
requirement defines the adjudication contract; the algorithm that produces an
`Attestation` is realized in a later spec-driven change (see `BACKLOG.md`), so at this
shape the constraint binds the design rather than describing a shipped algorithm.

#### Scenario: A new Seal creates; a witnessed Seal attaches
- **WHEN** a `Deed` is witnessed whose `Seal` is not yet in the `Ledger`, then again with the same `Seal`
- **THEN** the contract requires the first be attested `Create` and the second `Attach`, decided by `Seal` equality alone (once the algorithm is realized — see `BACKLOG.md`)

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
genuine semantic change. The core SHALL carry a `Seal` opaquely and SHALL NOT
interpret it.

#### Scenario: The same intent keeps its Seal
- **WHEN** the same intent is witnessed again
- **THEN** it carries the same `Seal`, and a genuine semantic change carries a new one

#### Scenario: The core does not interpret the Seal
- **WHEN** the core handles a `Seal`
- **THEN** it treats it as an opaque identity, compared by value, never by meaning

### Requirement: Fingerprints Are Mechanical Content Identity
A `Fingerprint` SHALL be the mechanical content identity of a `Deed` and SHALL be
compared byte-wise by the core, encoding content rather than meaning.

#### Scenario: Fingerprints compare byte-wise
- **WHEN** the core compares two `Fingerprint`s
- **THEN** it compares their bytes, making no judgment about what the content means

### Requirement: Contradiction Is A Structural Alarm
Shaahid SHALL be able to detect a structural `Contradiction` mechanically — the same
`Seal` presented with a drifted `Fingerprint`, or the same `Fingerprint` presented
under split `Seal`s — and surface it as an observable alarm. A `Contradiction` SHALL
NOT be a judgment that the domain was wrong, and the core SHALL own no response to it.
The detection is defined here; its implementation and report shape are deferred (see
`BACKLOG.md`).

#### Scenario: A drifted Fingerprint under a repeated Seal is a Contradiction
- **WHEN** a `Seal` already witnessed is presented again with a different `Fingerprint`
- **THEN** the core can surface a `Contradiction`, without deciding the domain was wrong

#### Scenario: The core owns no response
- **WHEN** a `Contradiction` is surfaced
- **THEN** what to do about it (reject, quarantine, escalate) is a downstream concern, not the core's

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
