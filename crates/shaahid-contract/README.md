# shaahid-contract

The isolated core contract for Shaahid: sans-I/O idempotency adjudication.

`shaahid-contract` witnesses a `Deed` (a domain-supplied `Seal` and a content
`Fingerprint`) and attests create-or-attach, detecting structural contradictions —
and makes no semantic judgment of its own: the meaning of "the same work" is the
domain's, supplied as a `Seal`. It exposes no `async fn`, reads no ambient clock, and
performs no I/O; a runtime drives it.

0.1.0 ships the witness core: `witness` adjudicates create-or-attach by `Seal`
equality and detects structural contradictions, returning an `Outcome`, alongside the
`Deed`/`Seal`/`Fingerprint`/`Attestation`/`Contradiction` vocabulary and the axioms.
The durable `Ledger`, any contradiction-response policy, and an async edge remain
deferred.

Part of [Shaahid](https://github.com/tacticaldoll/shaahid).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/shaahid/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/shaahid/blob/main/LICENSE-MIT), at your option.
