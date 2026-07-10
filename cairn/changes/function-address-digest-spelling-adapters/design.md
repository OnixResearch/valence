## Design

The adapter layer accepts only configured spellings at function-address evidence boundaries, normalizes them to a canonical Valence digest string for reports, and preserves the typed `FunctionAddressDigestDomain`. Internal validation continues to parse against the existing BLAKE3 profile.

### Decisions

1. **Spelling is not identity domain.** `b3:` prefix handling is separate from source/function/Kamacite/Mantle digest domain checks.
2. **Normalization is explicit.** Only named adapter inputs accept bare hex; internal report fields use a documented canonical spelling.
3. **Malformed input fails closed.** Short hex, uppercase when disallowed, missing prefix where required, and wrong algorithm labels are rejected.
4. **No cross-domain substitution.** A valid BLAKE3 hex cannot move between source, function, receipt, sidecar, or binary domains without a diagnostic.

### Validation shape

Positive fixtures cover Octet `b3:<hex>`, Kamacite bare hex with explicit domain, and Mantle sidecar bare hex. Negative fixtures cover short hex, non-hex, wrong prefix, and digest domain substitution.
