## RENAMED Requirements

- FROM: `### Requirement: Sans-I/O Purity Is Enforced`
- TO: `### Requirement: Sans-I/O Purity Has Static Teeth`

## MODIFIED Requirements

### Requirement: Executable Constitution
Shaahid SHALL enforce its architecture with one executable Tianheng constitution
(`shaahid-governance`), so the structural shadow of the boundaries prose claims is
gated, not merely asserted; what a static scan cannot observe stays review-governed.
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

### Requirement: Sans-I/O Purity Has Static Teeth
The Constitution SHALL express, as one composed sans-I/O profile over the full
`shaahid-contract` module subtree, that the subtree makes no inline `std::time` call
ending in `now` and exposes no public `async fn`. It SHALL separately enforce that the
subtree calls no `std::io`, `std::fs`, `std::net`, or `std::process` inline symbol
path. These static reactions are partial by nature: macro-expanded I/O, a clock read
through a method on a value (such as `Instant::elapsed`), a public function written to
return `impl Future`, and general effect reachability are not observed, so the
executable teeth SHALL complement review rather than claim complete effect analysis.

#### Scenario: An exposed async function in the core fails the profile
- **WHEN** `shaahid-contract` exposes a public `async fn` at the crate root or in a reachable submodule
- **THEN** the composed sans-I/O profile reports an enforced async-exposure violation

#### Scenario: An ambient clock read in the core fails the profile
- **WHEN** `shaahid-contract` makes an inline call to a path under `std::time` ending in `now`
- **THEN** the composed sans-I/O profile reports an enforced inline-call violation

#### Scenario: An explicit I/O call in the core fails its boundary
- **WHEN** `shaahid-contract` makes an inline call into `std::fs`
- **THEN** the explicit no-I/O boundary reports an enforced violation
