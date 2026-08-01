## 1. Leaf types gain Ord

- [ ] 1.1 Add `PartialOrd, Ord` to `Fingerprint`'s derive list in
      `crates/shaahid-contract/src/lib.rs`
- [ ] 1.2 Add `PartialOrd, Ord` to `Contradiction`'s derive list, with a doc-comment
      note that the derived order is mechanical and carries no domain meaning

## 2. Seal-conditional types gain Ord

- [ ] 2.1 Add `PartialOrd, Ord` to `Attestation<Seal>`'s derive list
- [ ] 2.2 Add `PartialOrd, Ord` to `Deed<Seal>`'s derive list (needs `Fingerprint: Ord`
      from 1.1)
- [ ] 2.3 Add `PartialOrd, Ord` to `Outcome<Seal>`'s derive list (needs
      `Attestation<Seal>: Ord` from 2.1 and `Contradiction: Ord` from 1.2)

## 3. Tests

- [ ] 3.1 Add a test proving `Fingerprint` and `Contradiction` can key a `BTreeMap` (or
      equivalent ordered-collection assertion) without any `Seal` bound
- [ ] 3.2 Add a test proving `Deed<Seal>`/`Attestation<Seal>`/`Outcome<Seal>` can key a
      sorted collection when `Seal: Ord`
- [ ] 3.3 Add a test asserting the derived order is consistent with the existing `Eq`
      (equal values compare as `Ordering::Equal`)

## 4. Verification

- [ ] 4.1 Run the full Definition of Done from `AGENTS.md` and confirm all gates pass
- [ ] 4.2 Confirm `shaahid-governance`'s facade re-exports-only and crate-boundary
      reactions still report clean
