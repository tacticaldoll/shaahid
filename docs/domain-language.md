# Domain Language

Shaahid uses a witness register as architecture, not branding. Each term names a role
in the adjudication; prefer the canonical term over synonyms.

## The adjudication

```text
Deed (Seal + Fingerprint) -> Witness -> Attestation (Create | Attach)
                                     \-> Contradiction? (Seal <-> Fingerprint mismatch)
```

- **Deed** — a unit of work presented to be witnessed. It carries a `Seal` and a
  `Fingerprint`.
- **Seal** — the domain-supplied, stable semantic identity of a `Deed`: what the
  domain means by "the same work". The same intent carries the same `Seal`; a genuine
  semantic change carries a new one. The core carries a `Seal` opaquely and never
  interprets it.
- **Fingerprint** — the mechanical content identity of a `Deed` (a hash of its
  bytes). Unlike a `Seal`, a `Fingerprint` is compared byte-wise by the core; it
  encodes content, not meaning.
- **Attestation** — the verdict for a witnessed `Deed`: `Create` (the `Seal` is new)
  or `Attach` (the `Seal` was already witnessed, so this is the same work).
- **Ledger** — the record of witnessed `Seal`s. Its durable persistence is a
  downstream concern, not the core.
- **Witness** — the adjudicator role that produces an `Attestation`.
- **Contradiction** — a structural anomaly the core can detect mechanically: the same
  `Seal` presented with a drifted `Fingerprint` (the domain reused an identity for
  changed content), or the same `Fingerprint` presented under split `Seal`s (the
  domain split an identity across identical content). A `Contradiction` is an
  observable alarm, not a judgment that the domain was "wrong".

## Domain-supplied meaning (the semantic bill of purity)

The core makes no semantic judgment. The one judgment that is the domain's:

- **Semantic identity** — the `Seal`. The domain decides what "the same work" means;
  Shaahid only adjudicates by `Seal` equality and compares `Fingerprint`s
  mechanically.

The cost — a wrong `Seal` that still matches its `Fingerprint` fails silently — is
accepted deliberately. Shaahid's structural `Contradiction` detection narrows, but
cannot eliminate, that silent-failure surface. See `PROJECT.md` and `BACKLOG.md`.

## Out of scope for the core

The durable persistence of the `Ledger`, and any policy on what to do when a
`Contradiction` is raised (reject, quarantine, escalate), are **downstream consumer
concerns**, not the adjudication core. Shaahid attests and alarms; the response is
composed outside `shaahid-contract`.
