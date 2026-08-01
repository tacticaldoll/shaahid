## ADDED Requirements

### Requirement: Identity Types Support Ord
`Fingerprint` and `Contradiction` SHALL implement `PartialOrd` and `Ord`
unconditionally. `Deed<Seal>`, `Attestation<Seal>`, and `Outcome<Seal>` SHALL implement
`PartialOrd` and `Ord` whenever `Seal` does. The derived order SHALL be mechanical
(field and variant declaration order) and SHALL NOT be interpreted as a domain-meaningful
ranking of severity, precedence, or correctness. This SHALL NOT change `witness`'s
behavior, the `Seal` bound required for adjudication (`Eq` alone), the emission order
of `Contradiction`s in an `Outcome` (already ascending `witnessed_index`, decided by the
adjudication algorithm), or any other already-derived trait.

#### Scenario: Fingerprint and Contradiction key an alloc-only sorted collection
- **WHEN** a composing system inserts `Fingerprint` or `Contradiction` values into a `BTreeMap` or `BTreeSet`
- **THEN** it compiles and behaves like any other `Ord` key, with no `std` and no `Seal` bound required

#### Scenario: Deed, Attestation, and Outcome key a sorted collection when Seal is Ord
- **WHEN** a composing system uses a `Seal` type that implements `Ord` and inserts `Deed<Seal>`, `Attestation<Seal>`, or `Outcome<Seal>` values into a `BTreeMap` or `BTreeSet`
- **THEN** it compiles and behaves like any other `Ord` key

#### Scenario: The derived order carries no domain meaning
- **WHEN** a composing system compares two `Contradiction` values of different variants with `Ord`
- **THEN** the result reflects only declaration order, and Shaahid makes no claim that it reflects severity, precedence, or correctness
