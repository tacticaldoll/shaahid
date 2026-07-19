## Context

Shaahid inherits a discipline from its Tianheng lineage — sans-I/O purity, OpenSpec,
vocabulary-as-governance, least-commitment — and shares no code with it. It is
stamped from the `pacta` reference implementation's skeleton and then made its own:
copy the skeleton, replace the domain, rewrite the worldview, and stand alone. After
this change the repository owns its architecture independently.

## Goals / Non-Goals

**Goals:**
- A self-sufficient initial shape: vision, vocabulary, axioms, governance, and a
  compiling skeleton, so the adjudication core can be built from the repo alone.
- Preserve the design knowledge (decisions + open questions) in `BACKLOG.md` and the
  specs, without freezing open design into code.

**Non-Goals:**
- The adjudication implementation (deferred, spec-driven).
- Naming any sibling product — this repo is sibling-blind. The durable `Ledger` and
  any `Contradiction` response are generic "consumer concerns", never named siblings.
- A published API; version bump; async variant.

## Decisions

- **Scaffold-first, implementation later.** The specs describe the *established*
  contract (vocabulary, axioms, the semantic bill, sans-I/O purity, dependency
  isolation) — all true now — and the adjudication *definition* (create-or-attach by
  `Seal` equality, `Contradiction` as a `Seal`↔`Fingerprint` mismatch), but not the
  *algorithm*, which is deferred (`BACKLOG.md`). This keeps `openspec/specs/` an honest
  record of shipped truth rather than a promise of unbuilt behavior.
- **Two crates.** `shaahid-contract` (the pure core) and `shaahid-governance` (the
  gate, `publish = false`). The gate depends only on `tianheng`/`guibiao`, never on
  the graph it judges.
- **The semantic bill of purity is the central design commitment.** A sans-I/O pure
  core cannot decide semantic identity, so the domain supplies the `Seal`. The core
  adjudicates by `Seal` equality and compares `Fingerprint`s byte-wise; it never
  decides what a `Deed` means. `Contradiction` detection makes a `Seal`↔`Fingerprint`
  mismatch observable — it narrows, but does not eliminate, the silent-failure surface.
- **The unenforceable invariant is acknowledged, not faked.** "The core makes no
  semantic judgment" has no syntactic marker, so Tianheng cannot bite it the way it
  bites no-I/O or no-async. `quality-governance` states this honestly and keeps it
  review- and structure-governed.
- **Governance mirrors the reference discipline** minus what does not apply: crate
  dependency boundaries, sans-I/O teeth (no I/O, no ambient clock, no async exposure),
  coverage, and active-prose presence. No facade check (no facade) and no
  forbidden-marker check (no serde yet).
- **Inherited hygiene from birth.** No ADRs (git-as-provenance), no OpenSpec archive
  (sync removes the change dir), single-sourced Definition of Done in `AGENTS.md`,
  crate-local READMEs with absolute LICENSE URLs.

## Risks / Trade-offs

- **Specs describe some not-yet-built contract (the adjudication definition).** →
  Mitigated by specing only definitional/constraint requirements that hold now (what
  create-or-attach and a `Contradiction` *are*, the purity constraints), not
  algorithmic behavior; the algorithm's requirements are added by the change that
  builds it.
- **The purity invariant that matters most is unenforceable.** → Acknowledged in the
  spec rather than hidden; `Contradiction` detection is the structural mitigation, not
  an enforcement.
