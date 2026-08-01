## 1. Leaf types gain Hash

- [x] 1.1 Add `Hash` to `Fingerprint`'s derive list in `crates/shaahid-contract/src/lib.rs`
- [x] 1.2 Add `Hash` to `Contradiction`'s derive list

## 2. Seal-conditional types gain Hash

- [x] 2.1 Add `Hash` to `Attestation<Seal>`'s derive list (compiles once `Contradiction`
      is not involved here — only needs `Seal: Hash`, added automatically by the derive
      macro's generated bound)
- [x] 2.2 Add `Hash` to `Deed<Seal>`'s derive list (needs `Fingerprint: Hash` from 1.1)
- [x] 2.3 Add `Hash` to `Outcome<Seal>`'s derive list (needs `Attestation<Seal>: Hash`
      from 2.1 and `Contradiction: Hash` from 1.2)

## 3. Contradiction gains Display

- [x] 3.1 Implement `core::fmt::Display for Contradiction` with a one-line message per
      variant naming the variant kind and `witnessed_index`

## 4. Tests

- [x] 4.1 Add a test proving `Fingerprint` and `Contradiction` can key a `HashMap` (or
      equivalent hash-based assertion) without any `Seal` bound
- [x] 4.2 Add a test proving `Deed<Seal>`/`Attestation<Seal>`/`Outcome<Seal>` can key a
      hash-based collection when `Seal: Hash` (reuse or extend the existing test seal
      types)
- [x] 4.3 Add a test asserting equal values hash equal (e.g. via a `HashSet` dedup check
      or a manual hasher comparison) for at least one identity type
- [x] 4.4 Add tests asserting the `Display` output for `DriftedFingerprint` and
      `SplitSeal` each names the variant and the `witnessed_index`

## 5. Verification

- [x] 5.1 Run the full Definition of Done from `AGENTS.md` (build, test, clippy, fmt,
      doc, `cargo deny check`, `shaahid-governance` check, MSRV `no_std` target check)
      and confirm all gates pass before checking off any task above as verified
- [x] 5.2 Confirm `shaahid-governance`'s facade re-exports-only and crate-boundary
      reactions still report clean (no edit to `crates/shaahid/src/lib.rs` is expected,
      but the gate must be run, not assumed)
