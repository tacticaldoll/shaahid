# Shaahid

Shaahid is a thin, sans-I/O **idempotency-adjudication** core for Rust: given a
`Deed` (a domain-supplied `Seal` and a content `Fingerprint`), it adjudicates
**create-or-attach** and detects structural contradictions — while making no
semantic judgment of its own.

```text
Deed (Seal + Fingerprint) -> Witness -> Attestation (Create | Attach)
                                     \-> Contradiction? (Seal <-> Fingerprint mismatch)
```

It is for the narrow space between "I must not act on the same intent twice" and "I
do not want a store that guesses what 'the same' means." Shaahid witnesses and
attests; the meaning of "the same work" is the domain's, supplied as a `Seal`.

## Scope

Shaahid is a thin sans-I/O idempotency-witness core, not a ledger or a workflow engine. It owns
one mechanism — adjudicating create-or-attach by `Seal` equality and detecting structural
contradictions (a drifted `Fingerprint` under a repeated `Seal`, a split `Seal` under a repeated
`Fingerprint`) — and the `Deed`/`Seal`/`Fingerprint`/`Attestation`/`Contradiction` vocabulary, its
architectural axioms, and executable governance; the meaning of "the same work" (the `Seal`) is the
domain's to supply. It does **not** own a durable `Ledger`, a contradiction-response policy, or an
async edge — those are deferred (see `BACKLOG.md`) or lie outside the pattern's shape.

Depend on the curated **facade** (`shaahid`) — the recommended single entrypoint, which re-exports
the public surface and carries a runnable witness doctest; the isolated core (`shaahid-contract`)
stays available for direct use. See `CHANGELOG.md` for what each release adds.

## What Shaahid owns, and what the domain supplies

Shaahid owns a *decision* and an *alarm*, not *meaning*. It decides create-or-attach
by `Seal` equality and raises a contradiction when a `Seal` and its `Fingerprint`
disagree; it never decides what two `Deed`s *mean*. That is yours.

```text
The domain supplies (meaning)              Shaahid owns (mechanism, no meaning)
  Seal        a stable semantic identity     create-or-attach by Seal equality
  Fingerprint the content it hashes to        Seal <-> Fingerprint contradiction alarm
                                              (never judging whether a Seal is "right")
```

The `Ledger`'s durability and any policy on a detected contradiction are composed around
the pattern, outside its shape — never owned by the core.

## Why sans-I/O and no semantic judgment

A pure core that reads no clock and performs no I/O cannot decide meaning either — so
the domain supplies the `Seal`, and Shaahid only adjudicates and compares
mechanically. This is the **semantic bill of purity**: a wrong `Seal` that still
matches its `Fingerprint` fails silently, accepted deliberately rather than patched by
judging meaning. See `PROJECT.md` and `BACKLOG.md`.

## Domain Language

Shaahid uses witness terms as architecture, not branding — `Deed`, `Seal`,
`Fingerprint`, `Attestation`, `Ledger`, `Witness`, `Contradiction`. See
[`docs/domain-language.md`](docs/domain-language.md).

## Architecture

- `PROJECT.md` — vision, positioning, non-goals.
- `openspec/specs/` — shipped requirements.
- `BACKLOG.md` — deferred decisions and open design questions.
- `AGENTS.md` — operating protocol and the Definition of Done.

## Contributing

This project uses OpenSpec and Tianheng-native governance. Start a change with:

```bash
openspec new change "your-change-name"
```

Run the full Definition of Done (see `AGENTS.md`) before committing, and read
`AGENTS.md` before making repository changes.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
