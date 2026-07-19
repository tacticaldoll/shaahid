# public-facade Specification

## Purpose
Define the curated `shaahid` facade as the workspace's single public entrypoint: a pure re-export crate over `shaahid-contract` that withholds nothing (there is no advanced kernel), proves end-to-end witness composition with a crate-root doctest and a facade integration test, and is enforced by executable governance.

## Requirements
### Requirement: Curated Public Entrypoint
Shaahid SHALL provide a single facade crate `shaahid` that is the curated public
entrypoint to the adjudication API. The facade SHALL re-export the public items a
downstream consumer needs to witness a stream of deeds — build a `Fingerprint` and a
`Deed`, call `witness`, and read the resulting `Outcome` (its `Attestation` and any
`Contradiction`) — drawing them from `shaahid-contract`. Because all of
`shaahid-contract`'s public API is compose-level (there is no advanced kernel to
withhold), the facade SHALL re-export it in full. The facade SHALL depend only on
`shaahid-contract`.

#### Scenario: Facade re-exports the public surface
- **WHEN** a downstream consumer depends only on `shaahid`
- **THEN** it can name `Fingerprint`, `Deed`, `Attestation`, `Contradiction`, and `Outcome`, and call `witness`, without depending on `shaahid-contract` directly

#### Scenario: Facade depends only on the core
- **WHEN** `cargo run -p shaahid-governance -- check --manifest-path Cargo.toml` runs
- **THEN** the Tianheng constitution reports no violation, because `shaahid` depends only on `shaahid-contract`

### Requirement: Facade Carries No Logic
The facade SHALL be a pure re-export crate: its library SHALL contain only re-exports,
crate attributes, and documentation, and SHALL NOT define functions, types, traits, or
other behavior. This keeps the published entrypoint from accreting convenience logic
over time. This constraint SHALL be enforced by an executable reaction, not by omission
alone.

#### Scenario: A logic item in the facade fails governance
- **WHEN** the facade library defines an item other than a re-export (for example a function, struct, enum, or trait)
- **THEN** the governance reaction fails, naming the offending line

#### Scenario: The facade library composes only through re-exports
- **WHEN** the facade library is reviewed
- **THEN** every public item it offers is a re-export of an item from `shaahid-contract`, and it holds no logic of its own

### Requirement: Facade Composition Doctest
The `shaahid` facade SHALL carry a runnable documentation test in its crate root that
witnesses a deed end to end through the facade — build a `Deed`, call `witness` against
a consumer-held ledger, and read the `Attestation` — using only the facade's public
API. This composition proof SHALL be a doctest rather than a separate `examples/` build
target, so it runs and asserts under `cargo test` and is rendered on the published
documentation.

#### Scenario: Facade composition doctest witnesses create then attach
- **WHEN** the facade doctest witnesses a fresh `Seal` and then re-witnesses it
- **THEN** the first witness yields `Attestation::Create` and the second yields `Attestation::Attach` with no contradiction

#### Scenario: Facade composition doctest imports only from the facade
- **WHEN** the facade doctest is compiled
- **THEN** it references only items re-exported by `shaahid`, and does not import from `shaahid-contract` directly

### Requirement: Composition Is Demonstrated In Depth
The `shaahid` facade SHALL carry an integration test that composes an idempotency gate
over the facade's public API across four trajectories — a fresh `Create`, an idempotent
`Attach`, a `DriftedFingerprint` contradiction, and a `SplitSeal` contradiction — so its
demonstration is earned rather than a happy-path stub. The witnessed ledger and every
disposition (record / deduplicate / quarantine) SHALL be held in the consumer test, not
the core. It SHALL run clean under the Definition of Done, so composability is an
enforced, non-regressing property rather than a claim.

#### Scenario: The four trajectories are exercised through the facade
- **WHEN** the integration test runs its stub domain
- **THEN** it drives a fresh create, an idempotent attach, a drifted-fingerprint contradiction, and a split-seal contradiction within one run, using only items re-exported by `shaahid`

#### Scenario: The consumer owns admission and disposition
- **WHEN** an outcome carries a contradiction
- **THEN** the consumer quarantines the deed in its own loop body over its own ledger, and the core neither admits, records, nor responds to the deed

#### Scenario: A broken composition fails the gate
- **WHEN** the facade fails to compose or the gate does not reach its expected ledger state
- **THEN** running the test under the Definition of Done fails rather than passing silently
