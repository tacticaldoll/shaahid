# shaahid-governance

Executable architectural governance for the Shaahid workspace — the Tianheng
constitution.

This crate is an internal gate, not a published library (`publish = false`). It
depends only on the composed [Tianheng](https://github.com/tacticaldoll/tianheng)
0.3 harness and uses one Constitution for the command-line reaction and architecture
tests. That Constitution governs all three Cargo dependency tables, complete workspace
coverage, and the adjudication core's observable sans-I/O shape: no ambient-clock
reads, no exposed `async fn`, and explicit source-level guards against
`std::io/fs/net/process`.

`AGENTS.shaahid-law.md` is generated from that Constitution and byte-checked in tests.
Regenerate it with:

```sh
BLESS=1 cargo test -p shaahid-governance law_projection_is_fresh
```

The gate also runs custom active-prose and facade-reexports-only reactions. Those
checks remain executable but are honestly outside Tianheng's generated projection.

It deliberately does **not** enforce "the core makes no semantic judgment": that
axiom has no syntactic marker, so it is not statically expressible and stays
review-governed, not a tooth here.

Run it from the workspace root:

```sh
cargo run -p shaahid-governance -- check --manifest-path Cargo.toml
```

Part of [Shaahid](https://github.com/tacticaldoll/shaahid).

## License

Licensed under either of [Apache-2.0](https://github.com/tacticaldoll/shaahid/blob/main/LICENSE-APACHE) or [MIT](https://github.com/tacticaldoll/shaahid/blob/main/LICENSE-MIT), at your option.
