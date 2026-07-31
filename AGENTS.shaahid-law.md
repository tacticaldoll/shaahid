# Shaahid Tianheng Law Projection

Generated from `constitution()` in `crates/shaahid-governance/src/main.rs`.
**Do not edit by hand.** Regenerate it with:
`BLESS=1 cargo test -p shaahid-governance law_projection_is_fresh`.
If the law itself is wrong, amend the Constitution through the governed OpenSpec workflow.

This projection covers Tianheng-observable structure only. The custom active-prose and
facade-reexports reactions remain executable in `shaahid-governance`, but are outside
Tianheng's generated projection.
# Constitution: shaahid

## Static boundaries

### `shaahid-contract`

> shaahid-contract is the isolated adjudication core. At this shape it depends on nothing, and must never depend on another workspace crate or a runtime framework: its adjudication is pure.

- **rule**: restrict dependencies to (only: )
- **kind**: crate · **severity**: enforce

### `shaahid-contract`

> shaahid-contract is the isolated adjudication core. At this shape it depends on nothing, and must never depend on another workspace crate or a runtime framework: its adjudication is pure.

- **rule**: restrict dependencies to (dependency_kind: dev; only: )
- **kind**: crate · **severity**: enforce

### `shaahid-contract`

> shaahid-contract is the isolated adjudication core. At this shape it depends on nothing, and must never depend on another workspace crate or a runtime framework: its adjudication is pure.

- **rule**: restrict dependencies to (dependency_kind: build; only: )
- **kind**: crate · **severity**: enforce

### `shaahid-governance`

> the governance gate must stay independent of the workspace graph it judges: it may depend only on the tianheng governance harness, never on a workspace crate under judgment.

- **rule**: restrict dependencies to (only: tianheng)
- **kind**: crate · **severity**: enforce

### `shaahid-governance`

> the governance gate must stay independent of the workspace graph it judges: it may depend only on the tianheng governance harness, never on a workspace crate under judgment.

- **rule**: restrict dependencies to (dependency_kind: dev; only: )
- **kind**: crate · **severity**: enforce

### `shaahid-governance`

> the governance gate must stay independent of the workspace graph it judges: it may depend only on the tianheng governance harness, never on a workspace crate under judgment.

- **rule**: restrict dependencies to (dependency_kind: build; only: )
- **kind**: crate · **severity**: enforce

### `shaahid`

> shaahid is the curated published entrypoint. It may depend only on shaahid-contract, never on a backend, runtime, or external framework.

- **rule**: restrict dependencies to (only: shaahid-contract)
- **kind**: crate · **severity**: enforce

### `shaahid`

> shaahid is the curated published entrypoint. It may depend only on shaahid-contract, never on a backend, runtime, or external framework.

- **rule**: restrict dependencies to (dependency_kind: dev; only: )
- **kind**: crate · **severity**: enforce

### `shaahid`

> shaahid is the curated published entrypoint. It may depend only on shaahid-contract, never on a backend, runtime, or external framework.

- **rule**: restrict dependencies to (dependency_kind: build; only: )
- **kind**: crate · **severity**: enforce

### `crate`

> the sans-I/O adjudication core reads no ambient clock and stays runtime-agnostic: witnessed state is supplied at the runtime edge, and its public API exposes no async fn.

- **rule**: inline symbol path confined to module (confined_prefix: std::time; ending_with: now)
- **kind**: module · **severity**: enforce · **crate**: shaahid-contract

### `crate`

> the sans-I/O adjudication core performs no I/O: no code in shaahid-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::io)
- **kind**: module · **severity**: enforce · **crate**: shaahid-contract

### `crate`

> the sans-I/O adjudication core performs no I/O: no code in shaahid-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::fs)
- **kind**: module · **severity**: enforce · **crate**: shaahid-contract

### `crate`

> the sans-I/O adjudication core performs no I/O: no code in shaahid-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::net)
- **kind**: module · **severity**: enforce · **crate**: shaahid-contract

### `crate`

> the sans-I/O adjudication core performs no I/O: no code in shaahid-contract may call into std::io/fs/net/process; I/O lives in a runtime outside the core. Coverage is partial by nature (I/O entry points cannot be enumerated, and macro-expanded I/O such as println! is invisible to a source scan), so this tooth complements review rather than replacing it.

- **rule**: inline symbol path confined to module (confined_prefix: std::process)
- **kind**: module · **severity**: enforce · **crate**: shaahid-contract

## Async-exposure boundaries

### `crate`

> the sans-I/O adjudication core reads no ambient clock and stays runtime-agnostic: witnessed state is supplied at the runtime edge, and its public API exposes no async fn.

- **rule**: must not expose async fn (including_submodules: true; scan_depth: subtree)
- **kind**: semantic · **severity**: enforce · **crate**: shaahid-contract
