## Context

`idempotency_gate_resolves_four_trajectories` in `crates/shaahid/tests/idempotency_gate.rs`
is the facade's composition proof for the `public-facade` spec's "Composition Is
Demonstrated In Depth" requirement. Its quarantine branch currently does this per
contradiction:

```rust
let (kind, index) = match contradiction {
    Contradiction::DriftedFingerprint { witnessed_index } => ("drift", *witnessed_index),
    Contradiction::SplitSeal { witnessed_index } => ("split", *witnessed_index),
};
let _ = (kind, index);
```

This predates `Contradiction`'s `Display` (added in `core-derive-ergonomics`) and is now
redundant with it.

## Goals / Non-Goals

**Goals:**
- Replace the manual match-and-discard with `Contradiction`'s `Display`, so the test
  does something with the value instead of computing and dropping it.
- Keep every existing assertion (dispositions, final ledger state) unchanged.

**Non-Goals:**
- No change to `Disposition`, the stream of trajectories, or the `check` assertions.
- No new capability, no production code change — this is test-only.

## Decisions

- **Collect the `Display` message into the log entry alongside the label**, rather than
  merely calling `format!` and discarding the result again. Just swapping the match for
  a `format!("{contradiction}")` that's still assigned to `_` would repeat the same
  smell one level down. Extending `log`'s tuple to also capture the formatted message
  (or asserting on it directly) gives the change a real assertion, not just a
  find-and-replace of one no-op for another.
- **No change to the `Disposition` enum or `check`'s existing assertions** — the four
  trajectories' outcomes are already correctly asserted; this only touches the
  quarantine branch's internal handling of contradictions.

## Risks / Trade-offs

- **Over-asserting on exact `Display` wording** would couple this test to the message
  literal text, which `adjudication-contract`'s "Contradiction Supports Display"
  requirement deliberately leaves unpinned (log aid, not a serialization contract) →
  Mitigation: assert only that a non-empty message was produced per contradiction (or
  that it contains the `witnessed_index`), not the exact string.
