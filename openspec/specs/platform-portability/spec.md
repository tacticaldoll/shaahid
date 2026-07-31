# platform-portability Specification

## Purpose
Define the supported Rust platform perimeter for Shaahid's published crates:
unconditional `no_std + alloc` libraries with one adjudication API, proven at the
declared MSRV on a representative no-OS target without claiming allocation-free or
target-specific integration.

## Requirements
### Requirement: Published Crates Support No-Std With Allocation
The `shaahid-contract` and `shaahid` library crates SHALL compile unconditionally
without the Rust standard library and SHALL use only `core` plus `alloc` for their
language-runtime surface. The `shaahid` facade SHALL preserve this support while
remaining a pure re-export of `shaahid-contract`.

#### Scenario: Both published crates compile for a no-OS target
- **WHEN** the published libraries are checked for `thumbv7em-none-eabi`
- **THEN** both compile without linking or requiring `std`

#### Scenario: The recommended facade preserves portability
- **WHEN** a no-std composing system depends on `shaahid`
- **THEN** it receives the same re-exported adjudication API without depending directly on `shaahid-contract`

### Requirement: Portability Has One API And Behavior
Shaahid SHALL NOT introduce a Cargo `std` feature, a `no_std` feature, or separate
standard-library and no-standard-library implementations. The existing public API,
adjudication outcomes, ordering, dependency boundaries, and facade completeness SHALL
remain the same in the newly supported environment.

#### Scenario: No feature selects a different implementation
- **WHEN** the published crate manifests and library roots are inspected
- **THEN** no Cargo feature switches between std and no-std behavior, and the libraries declare no-std unconditionally

#### Scenario: Existing behavior remains authoritative
- **WHEN** the host workspace tests and doctests run after portability is added
- **THEN** the existing create, attach, drift, split, ordering, purity, and facade-composition behaviors remain unchanged

### Requirement: Allocation Perimeter Is Explicit
Shaahid SHALL continue to use allocation for its existing owned representations:
`Fingerprint` owns boxed bytes and `Outcome` owns a vector of contradictions. The
composing system SHALL provide an allocator. Shaahid SHALL NOT claim allocation-free
operation, provide an allocator, or add fixed-capacity, borrowed-storage, runtime,
serialization, persistence, or platform-adapter alternatives as part of this
capability.

#### Scenario: No-std does not imply no-alloc
- **WHEN** the portability capability is documented
- **THEN** it names `alloc` as required and leaves the existing owned public representations unchanged

#### Scenario: Platform policy stays outside Shaahid
- **WHEN** Shaahid is compiled for a no-OS target
- **THEN** selecting and supplying an allocator, runtime, driver, or persistence mechanism remains the composing system's responsibility

### Requirement: Portability Is Proven At The MSRV
The representative no-OS reaction SHALL compile both published crates using Rust
1.88 and target `thumbv7em-none-eabi`. A clean host build or Tianheng reaction alone
SHALL NOT count as proof of this portability capability.

#### Scenario: The representative MSRV reaction is clean
- **WHEN** `cargo +1.88 check -p shaahid-contract -p shaahid --target thumbv7em-none-eabi` runs with the target installed
- **THEN** it exits successfully

#### Scenario: Standard-library coupling breaks the reaction
- **WHEN** either published library requires `std`
- **THEN** the representative no-OS compile reaction fails rather than silently certifying portability
