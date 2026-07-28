# Radicle COB evidence profile

`profile.ncl` is the reviewed source of truth for the Valence evidence-sidecar profile.

It defines schemas, relationships, actor roles, embed bounds, confidentiality classes, and required non-claims.

`generated/profile.json` is a deterministic runtime export. Do not edit it directly.

Run the focused gate with:

```bash
nix build .#checks.x86_64-linux.radicle-cob-evidence-profile --no-link
```

Nickel validates static profile shape and bounded claim policy only.

Rust retains responsibility for Evidence IR validation, byte measurement, OID checks, signatures, authorization, event reduction, and receipts.

A passing contract does not prove evidence truth, target correctness, lifecycle readiness, release eligibility, or authority beyond the supplied policy.
