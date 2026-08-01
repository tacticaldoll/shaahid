## 1. Use Contradiction's Display in the composition test

- [ ] 1.1 In `idempotency_gate_resolves_four_trajectories`, replace the manual
      `(kind, index)` match-and-discard with `Contradiction`'s `Display`, capturing the
      formatted message per contradiction into the disposition log (extend `log`'s tuple
      or an adjacent collection) instead of computing and dropping it
- [ ] 1.2 Assert that each quarantined trajectory produced a non-empty `Display` message
      containing its `witnessed_index`, without pinning the exact wording (the
      `adjudication-contract` spec leaves the literal text unpinned)

## 2. Verification

- [ ] 2.1 Run the full Definition of Done from `AGENTS.md` and confirm all gates pass,
      including that the four existing disposition/ledger assertions still hold
      unchanged
