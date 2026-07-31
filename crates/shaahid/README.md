# shaahid

The curated entrypoint to Shaahid: a thin, sans-I/O idempotency-adjudication core you
compose.

`shaahid` is a pure re-export facade — it carries no logic of its own. It re-exports the
public surface you need to witness a stream of deeds: the identity types (`Fingerprint`,
`Deed`), the witness verdict (`Attestation`, `Contradiction`, `Outcome`), and the
adjudication itself (`witness`). This is the recommended crate to depend on.

Shaahid owns one mechanism — the witness — and outsources semantic identity to the
domain: given a `Deed` (a domain-supplied `Seal` paired with a content `Fingerprint`)
and the witnessed set, `witness` returns a create-or-attach `Attestation` plus any
structural `Contradiction`, and decides no admission of its own.

Shaahid's whole public surface is compose-level, so this facade withholds nothing;
there is no advanced kernel to reach for through
[`shaahid-contract`](https://crates.io/crates/shaahid-contract) directly.

Part of [Shaahid](https://github.com/tacticaldoll/shaahid).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/shaahid/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/shaahid/blob/main/LICENSE-MIT), at your option.
