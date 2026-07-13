# CLI capability roots

Every Valence filesystem operation now receives authority separately from its
file locator. Operators declare a role-specific root (`--artifact-root`,
`--manifest-root`, `--sidecar-root`, `--binary-root`, or `--output-root`) and
supply a relative locator beneath that root. Nested relative locators are
supported. Absolute locators, parent traversal, omitted roots, and read/write
role mismatches fail before a capability handle is opened.

For example, migrate:

```text
valence check /srv/evidence/artifact.json
```

to:

```text
valence check --artifact-root /srv/evidence artifact.json
```

There is no ambient fallback to the locator's parent or the current directory.
The CLI opens each declared root with `cap-std` and performs the operation
through that handle. A symlink that resolves outside the handle fails closed.
The pure access-plan validator in `valence-core` performs no filesystem I/O;
root opening, reads, measurements, and writes remain in `valence-cli`.

Passing this boundary proves only that the selected local operation remained
beneath the explicitly supplied capability handle. It does not prove that the
root is trustworthy, that host filesystem semantics are race-free, evidence
truth, sandbox completeness, behavioral correctness, or release eligibility.
