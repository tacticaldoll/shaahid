# quality-governance Specification

## Purpose
The executable-governance contract for Shaahid: the Tianheng constitution and prose
gates that enforce the architecture — crate dependency boundaries, the core's sans-I/O
purity, the facade's re-export purity, workspace coverage, and active-prose presence — so
the boundaries the prose claims are gated, not merely asserted. The one honest exception
(the no-semantic-judgment invariant is not statically expressible) is recorded rather
than papered over.

## Requirements
### Requirement: Executable Constitution
Shaahid SHALL enforce its architecture with one executable Tianheng constitution
(`shaahid-governance`), so the boundaries prose claims are gated, not merely asserted.
The runner, architecture tests, workspace-coverage check, and generated law projection
SHALL all consume that same Constitution. The gate SHALL depend directly only on the
composed `tianheng` shell, never on an individual Tianheng instrument or on a workspace
crate under judgment.

#### Scenario: The unified constitution runs clean on the workspace
- **WHEN** `cargo run -p shaahid-governance -- check --manifest-path Cargo.toml` runs
- **THEN** its unified Tianheng reaction reports no static, semantic, or runtime-coverage violation for the current workspace

#### Scenario: The gate is independent of instruments and the judged graph
- **WHEN** `shaahid-governance`'s normal dependencies are read
- **THEN** they contain only `tianheng`, never an individual instrument crate or a workspace crate under judgment

### Requirement: Dependency Boundaries Are Enforced
The Constitution SHALL restrict every workspace crate's normal, development, and build
dependency tables independently. `shaahid-contract` SHALL declare no dependency in any
table. The `shaahid` facade SHALL declare only `shaahid-contract` as a normal
dependency and no development or build dependency. `shaahid-governance` SHALL declare
only `tianheng` as a normal dependency and no development or build dependency. These
authored-table boundaries SHALL complement, not replace, `cargo-deny`'s resolved
whole-graph supply-chain policy.

#### Scenario: An unapproved normal core dependency fails the gate
- **WHEN** `shaahid-contract` gains a normal dependency
- **THEN** the Constitution reports an enforced dependency-boundary violation

#### Scenario: A development dependency cannot bypass core isolation
- **WHEN** `shaahid-contract` gains a development dependency
- **THEN** the Constitution reports an enforced dependency-boundary violation

#### Scenario: A build dependency cannot bypass core isolation
- **WHEN** `shaahid-contract` gains a build dependency
- **THEN** the Constitution reports an enforced dependency-boundary violation

#### Scenario: An unapproved facade dependency fails the gate
- **WHEN** the `shaahid` facade gains a dependency other than its allowed normal dependency on `shaahid-contract`
- **THEN** the Constitution reports an enforced dependency-boundary violation for the affected dependency table

#### Scenario: The governance gate knows only the composed shell
- **WHEN** `shaahid-governance` directly depends on an individual Tianheng instrument such as `guibiao`
- **THEN** the Constitution reports an enforced dependency-boundary violation

### Requirement: Sans-I/O Purity Is Enforced
The Constitution SHALL express the core's clock-free and synchronous public-API facts
as one composed sans-I/O profile over the full `shaahid-contract` module subtree. It
SHALL separately enforce that the subtree calls no `std::io`, `std::fs`, `std::net`, or
`std::process` inline symbol path. These static reactions are partial by nature:
macro-expanded I/O and general effect reachability are not observed, so the executable
teeth SHALL complement review rather than claim complete effect analysis.

#### Scenario: An exposed async function in the core fails the profile
- **WHEN** `shaahid-contract` exposes an `async fn` at the crate root or in a reachable submodule
- **THEN** the composed sans-I/O profile reports an enforced async-exposure violation

#### Scenario: An ambient clock read in the core fails the profile
- **WHEN** `shaahid-contract` calls a path under `std::time` ending in `now`
- **THEN** the composed sans-I/O profile reports an enforced inline-call violation

#### Scenario: An explicit I/O call in the core fails its boundary
- **WHEN** `shaahid-contract` calls into `std::fs`
- **THEN** the explicit no-I/O boundary reports an enforced violation

### Requirement: The Facade Is A Pure Re-Export Surface
The constitution SHALL enforce that the `shaahid` facade library holds only re-exports,
crate attributes, and documentation, so the curated entrypoint cannot accrete logic. The
check SHALL scan the facade source tree and SHALL NOT pass vacuously: a facade source
tree that is missing or unreadable SHALL fail the gate rather than scan zero files and
pass. Because `shaahid-governance` may depend only on governance-family tooling, the scan
SHALL be a brace-depth line heuristic rather than a full parser; the Definition of Done's
`cargo fmt --all --check` backstops the one gap where a logic item is co-located on a
re-export line.

#### Scenario: A logic item in the facade fails the gate
- **WHEN** the facade library defines an item other than a re-export at brace-depth zero
- **THEN** the re-exports-only scan reports a violation naming the file and line

#### Scenario: A missing facade source tree fails loudly
- **WHEN** the re-exports-only scan finds no facade source files
- **THEN** it fails the gate rather than passing on an empty scan

#### Scenario: A clean facade passes
- **WHEN** the facade library contains only re-exports, attributes, and comments
- **THEN** the re-exports-only scan reports no violation

### Requirement: Workspace Coverage
Every workspace crate SHALL be covered by a dependency boundary, so no crate is
silently ungoverned.

#### Scenario: Coverage is complete and non-vacuous
- **WHEN** coverage is computed from `cargo metadata`
- **THEN** the crate count is greater than zero and no crate is uncovered

### Requirement: Active Prose Is Present
The governed active-prose files SHALL be present and readable, and a governed doc that
vanishes SHALL fail the gate rather than pass vacuously. The governed set is
`AGENTS.md`, `PROJECT.md`, `README.md`, `BACKLOG.md`, `docs/development-flow.md`, and
`docs/domain-language.md`.

#### Scenario: A missing governed doc fails loudly
- **WHEN** the prose check runs against a root missing a governed file
- **THEN** it fails the gate, naming the unreadable file

### Requirement: The No-Semantic-Judgment Invariant Is Not Statically Enforced
The constitution SHALL NOT claim to statically enforce "the core makes no semantic
judgment": semantic comparison has no syntactic marker, so it is not expressible as a
static boundary. It SHALL remain review- and structure-governed, and this honest limit
SHALL be recorded rather than papered over.

#### Scenario: The limit is acknowledged
- **WHEN** the governance surface describes what it enforces
- **THEN** it states that the no-semantic-judgment axiom is review-governed, not a Tianheng tooth

### Requirement: Governance Reactions Have Focused Structured Proofs
The governance test suite SHALL evaluate the unified Constitution and SHALL prove both
teeth and precision for each changed law. A violating proof SHALL identify the intended
reaction by its structured governed target, semantic rule key, and fact identity rather
than by presentation wording alone. A nearby allowed case SHALL remain clean. Exit class
2, a warning, a baseline, successful compilation, or projection generation alone SHALL
NOT count as proof that a boundary reacts.

#### Scenario: A focused violating fixture proves the intended boundary
- **WHEN** a fixture introduces one governed violation
- **THEN** the test observes an enforced violation whose structured target, rule key, and fact identify the intended boundary

#### Scenario: A precision fixture stays clean
- **WHEN** a nearby fixture retains only facts allowed by the same boundary
- **THEN** the unified Constitution returns a clean outcome

#### Scenario: Custom reactions remain independently proved
- **WHEN** active prose disappears or facade logic is introduced
- **THEN** the corresponding Shaahid custom reaction fails even though it is not a Tianheng Constitution boundary

### Requirement: Executable Law Is Projected Into Agent Context
Shaahid SHALL commit an agent-readable Markdown projection generated from the same
Constitution used by the governance runner. A test SHALL byte-compare the committed
artifact with a fresh projection and fail when it is missing, unreadable, or stale;
explicit `BLESS=1` or `BLESS=true` regeneration SHALL be the only test-supported write
path. The projection preamble SHALL state that Shaahid's custom active-prose and facade
source reactions are outside the Tianheng projection, so the artifact does not claim to
be the entire governance surface.

#### Scenario: A current projection passes
- **WHEN** the committed agent-law document equals the Constitution's generated Markdown plus its declared preamble
- **THEN** the projection-freshness test passes without writing the file

#### Scenario: A stale projection fails
- **WHEN** the Constitution changes without regenerating the committed agent-law document
- **THEN** the projection-freshness test fails and names the explicit regeneration path

#### Scenario: An agent sees the projection's honest scope
- **WHEN** an agent follows the repository's documented context order
- **THEN** it reads the generated Tianheng law after `AGENTS.md` and is told that custom prose and facade-source reactions remain outside that projection

### Requirement: Definition Of Done Is Single-Sourced
`AGENTS.md` SHALL state the complete Definition of Done, and other active prose
(`README.md`, `docs/development-flow.md`) SHALL point to it rather than restate a
divergent subset.

#### Scenario: The Definition of Done is stated once
- **WHEN** the Definition of Done is documented
- **THEN** `AGENTS.md` holds the complete gate list and other docs point to it

### Requirement: No-Std Portability Is A Required Gate
The authoritative Definition of Done and CI SHALL include the MSRV-pinned
`thumbv7em-none-eabi` compile reaction for both published crates. CI SHALL install the
target through its Rust 1.88 toolchain setup. Contributor documentation SHALL state
the exact one-time local target-install prerequisite without duplicating or weakening
the authoritative gate list.

#### Scenario: Definition of Done includes the portability reaction
- **WHEN** the complete Definition of Done is read from `AGENTS.md`
- **THEN** it includes `cargo +1.88 check -p shaahid-contract -p shaahid --target thumbv7em-none-eabi`

#### Scenario: CI provisions and runs the representative target
- **WHEN** CI evaluates a push or pull request
- **THEN** its Rust 1.88 job installs `thumbv7em-none-eabi` and runs the same portability reaction

#### Scenario: Other documentation remains single-sourced
- **WHEN** development documentation explains how to verify the workspace
- **THEN** it points to `AGENTS.md` for the complete gate list and may state only the target-install prerequisite outside that list
