## Design

The artifact ref core validates in-memory artifact reference fields and metadata. It does not read artifact bytes, walk directories, fetch remote artifacts, mutate stores, or decide whether an artifact is meaningful evidence.

### Decisions

1. **Reference roles are typed.** Binary artifacts, sidecars, receipts, source artifacts, release bundle members, and compatibility refs have distinct wrapper types even when the underlying digest bytes match.

2. **Digest spelling is profile-scoped.** Raw BLAKE3 hex, `b3:`, and `blake3:` spellings are accepted only when the selected compatibility profile allows them.

3. **Path confinement is metadata validation.** The core may validate already-supplied relative paths or artifact labels for escape patterns. Shells own filesystem resolution and byte measurement.

4. **Hash and size metadata are paired.** When a caller supplies both digest and byte count, validation reports mismatches deterministically and preserves role-specific diagnostics.

5. **Artifact refs are not evidence truth.** Passing validation proves syntax, role separation, path metadata confinement, and declared hash/size consistency only.

### Validation shape

Positive tests cover accepted digest spellings by profile, role-specific refs, confined relative paths, and matching hash/size metadata. Negative tests cover malformed refs, unsupported prefixes, role mixups, path escapes, stale size metadata, missing non-claims, and text that treats artifact identity as correctness or release approval.
