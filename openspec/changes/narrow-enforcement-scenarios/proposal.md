## Why

Several sentences about Shaahid's governance claim that the gate enforces more than it
observes. The accepted boundary reasons were already narrowed to what each reaction sees:
the clock tooth sees an inline `std::time` call ending in `now`, the async tooth sees a
public `async fn`, and the I/O teeth see inline `std::io`/`fs`/`net`/`process` calls. The
quality-governance spec still calls the profile's facts "clock-free", says the gate
enforces "the core's sans-I/O purity" and gates "the boundaries prose claims", and lists
only macro-expanded I/O among the unobserved shapes. The governance crate README says "no
ambient-clock reads", and `PROJECT.md` says Tianheng and the specs enforce the boundaries
prose claims. A reader who trusts those sentences will take a green gate as proof of
properties no source scan can see.

## What Changes

- quality-governance: the Purpose and the executable-constitution requirement say the gate
  covers the structural shadow of prose claims, with the rest review-governed.
- quality-governance: "Sans-I/O Purity Is Enforced" is renamed "Sans-I/O Purity Has Static
  Teeth". It names the three observed shapes and adds a clock read through a method on a
  value and a function written to return `impl Future` to the unobserved remainder. Its
  scenarios name the observed shapes.
- The governance crate README and the governance bullet in `PROJECT.md` are narrowed the
  same way.

Intent is unchanged: the core still exposes no `async fn`, reads no ambient clock, and
performs no I/O. No boundary, reason, test, or product code changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `quality-governance`: enforcement claims narrowed to what the gate observes.

## Impact

Prose only: `openspec/specs/quality-governance/spec.md`,
`crates/shaahid-governance/README.md`, and `PROJECT.md`. The constitution, its projection,
and the governance tests are unchanged.
