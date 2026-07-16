# Radicle COB evidence profile

`profile.ncl` is the reviewed source of truth for the Valence evidence-sidecar type, schemas, relationship vocabulary, actor roles, embed bounds, confidentiality classes, and required non-claims. `generated/profile.json` is a deterministic runtime export and must not be edited directly.

Run the focused gate with:

```bash
nix build .#checks.x86_64-linux.radicle-cob-evidence-profile --no-link
```

Nickel validates static profile shape and bounded claim policy only. Rust remains responsible for Evidence IR validation, byte measurement, Git/Radicle OID checks, signatures, authorization bindings, deterministic event reduction, and receipt construction. Passing the contract does not prove evidence truth, target correctness, authorization beyond the supplied policy, lifecycle readiness, or release eligibility.
