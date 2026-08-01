## ADDED Requirements

### Requirement: Identity Types Support Hash
`Fingerprint` and `Contradiction` SHALL implement `Hash` unconditionally. `Deed<Seal>`,
`Attestation<Seal>`, and `Outcome<Seal>` SHALL implement `Hash` whenever `Seal: Hash`.
Every `Hash` implementation SHALL be consistent with the type's existing `Eq`: values
that compare equal SHALL hash equal. This SHALL NOT change `witness`'s behavior, the
`Seal` bound required for adjudication (`Eq` alone), or any other already-derived trait.

#### Scenario: Fingerprint and Contradiction key a hash-based collection
- **WHEN** a composing system inserts `Fingerprint` or `Contradiction` values into a `HashMap` or `HashSet`
- **THEN** it compiles and behaves like any other `Hash + Eq` key, with no `Seal` bound required

#### Scenario: Deed, Attestation, and Outcome key a hash-based collection when Seal is Hash
- **WHEN** a composing system uses a `Seal` type that implements `Hash` and inserts `Deed<Seal>`, `Attestation<Seal>`, or `Outcome<Seal>` values into a `HashMap` or `HashSet`
- **THEN** it compiles and behaves like any other `Hash + Eq` key

#### Scenario: Equal values hash equal
- **WHEN** two values of the same identity type compare equal under the type's derived `Eq`
- **THEN** they also hash equal

### Requirement: Contradiction Supports Display
`Contradiction` SHALL implement `core::fmt::Display`, producing a human-readable,
one-line message per variant that names the variant kind and the conflicting
`witnessed_index`. This SHALL be independent of `Debug` (which remains derived) and
SHALL carry no machine-parseable format guarantee beyond naming the `witnessed_index`.

#### Scenario: Displaying a DriftedFingerprint contradiction
- **WHEN** a `Contradiction::DriftedFingerprint { witnessed_index }` is formatted with `Display`
- **THEN** the resulting message names the drift and includes that `witnessed_index`

#### Scenario: Displaying a SplitSeal contradiction
- **WHEN** a `Contradiction::SplitSeal { witnessed_index }` is formatted with `Display`
- **THEN** the resulting message names the split and includes that `witnessed_index`
