# Change: Kani contract evidence roles

## Why

Kani contracts can turn bounded harnesses into scoped functional evidence when function contracts, loop invariants, and decreases clauses are machine-checked. Valence needs to preserve those semantics instead of flattening every Kani result into a generic proof label.

## What Changes

- Add Kani contract facets for preconditions, postconditions, loop invariants, loop decreases, quantifiers, and contract harnesses.
- Bind each facet to exact source identity, monomorphization or target scope, solver/toolchain identity, and receipt status.
- Treat passing `proof_for_contract` results as scoped property evidence only for the declared contract.
- Preserve bounded-only status when a harness lacks passing contracts or complete unwind coverage.

## Impact

- **Files**: Evidence IR payload models, verifier-role policy checks, fixtures, CLI validation reports, and docs.
- **Consumers**: Octet exports and Cairn gates can distinguish contract-backed Kani evidence from bounded diagnostics.
- **Non-claims**: Contract facets do not prove verifier soundness, all monomorphizations, whole-program correctness, or unmodeled effects.
