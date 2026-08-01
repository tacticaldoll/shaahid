## Why

The `shaahid` facade's composition test (`crates/shaahid/tests/idempotency_gate.rs`)
proves the `public-facade` spec's "Composition Is Demonstrated In Depth" requirement by
driving four trajectories through the facade's public API alone. Since
`core-derive-ergonomics` shipped `Contradiction`'s `Display`, that test's quarantine
branch still manually re-derives a `(kind, index)` pair from each `Contradiction` and
immediately discards it (`let _ = (kind, index);`) — dead code that duplicates what
`Display` now does, and a missed chance to prove the new capability actually composes
through the facade rather than only in `shaahid-contract`'s own unit tests.

## What Changes

- Replace the manual `(kind, index)` match-and-discard in
  `idempotency_gate_resolves_four_trajectories` with `Contradiction`'s `Display` (e.g.
  collecting `format!("{contradiction}")` into the disposition log), so the quarantine
  branch does something with each contradiction instead of computing and dropping it.
- No change to the test's asserted dispositions or ledger outcome — the four
  trajectories (create, attach, drift, split) still resolve exactly as before; only the
  quarantine branch's internals change.
- No production code changes; no new capability.

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
(none — this exercises the existing "Composition Is Demonstrated In Depth" requirement
more faithfully; it does not change what that requirement demands)

## Impact

- Code: `crates/shaahid/tests/idempotency_gate.rs` only.
- API: none — test-only change, no public surface touched.
- Dependencies: none.
