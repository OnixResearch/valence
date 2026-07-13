# Preserves integrity profile migration

Valence rejects the legacy `preserves-text-v1` identity profile for new Preserves evidence. That profile filtered rendered lines containing self-hash field names, so attacker-controlled payload text could be omitted from the digest.

The corrected profile is `preserves-packed-structural-v2`. It parses the envelope, removes only self-hash fields declared by the selected top-level exclusion profile, encodes the remaining typed value deterministically, and hashes those bytes with BLAKE3. Payload keys and strings remain hash-bearing even when they contain `receipt_hash` or `preserves_content_hash`. Duplicate, nested, misplaced, or malformed self-hash structures fail closed.

## Migration ordering

1. Retain the exact original legacy envelope bytes.
2. Validate the declared legacy identity against those bytes.
3. Select a corrected structural exclusion profile explicitly.
4. Emit a `preserves-integrity-migration-v1` receipt binding the original-byte BLAKE3 hash, legacy identity, corrected identity, and non-claim boundary.
5. Configure each downstream consumer to accept the corrected profile or explicit migration receipt before publishing corrected-profile evidence.

Valence never rewrites a legacy receipt in place and cannot migrate an identity when the original bytes are unavailable. Octet, Aspen, Mantle, and Cairn consumers must record and validate their own profile acceptance decisions.

## Typed claim boundary

Claim authority is carried by typed claim class, verification role, required boundary identifiers, and prohibited capability values. Human-readable text remains display metadata: it cannot grant authority, promote a role, or replace the required `valence.identity-linkage-only.v1` boundary. Contradictory metadata that asserts correctness, equivalence, authority, verifier soundness, or release eligibility fails validation.

Passing these checks proves only canonical content identity, explicit migration linkage, and typed claim-policy conformance. It does not prove receipt truth, behavioral correctness, semantic equivalence, verifier soundness, authority, or release eligibility.
