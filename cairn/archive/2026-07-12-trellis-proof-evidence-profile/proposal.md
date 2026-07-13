## Why

Valence needs to distinguish Trellis proof evidence linkage from proof acceptance. A canonical Trellis proof evidence envelope carries source, proof IR, verifier receipt, assumptions, and role metadata; Valence should validate those links, enforce non-claims, and decide recorded-only versus formal-proof-counting boundaries for stack consumers.

## What Changes

- Add a Trellis proof evidence profile validator under the generic evidence-chain profile.
- Validate source/IR/verifier/assumption/property links, digest domains, spans, verification roles, policy hashes, and non-claims.
- Represent reference-only, formal-proof candidate, and accepted formal-proof roles distinctly.
- Emit graph rows linking Trellis proof evidence to requirements, source spans, proof obligations, and downstream artifacts.

## Impact

Valence can expose Trellis verified-logic evidence to Mantle and Cairn without allowing proof evidence to overclaim downstream system correctness or release eligibility.
