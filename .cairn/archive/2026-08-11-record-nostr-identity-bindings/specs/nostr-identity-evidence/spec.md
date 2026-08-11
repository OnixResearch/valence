# Nostr Identity Evidence Specification

## ADDED Requirements

### Requirement: The evidence profile is closed and versioned [r[valence.nostr_identity_evidence.profile]]
r[valence.nostr_identity_evidence.profile]

Valence MUST define a versioned Nostr identity evidence envelope with closed control fields and required non-claims.

#### Scenario: Valid producer artifacts decode

- GIVEN reviewed UCAN and Basalt producer artifacts use supported schemas
- WHEN Valence decodes the already-loaded values
- THEN it MUST preserve each typed credential, binding, request, policy, role, and receipt field

#### Scenario: Unknown or weakened shape fails

- GIVEN an artifact has unknown control fields, duplicate fields, unsupported schemas, missing identities, or weakened non-claims
- WHEN profile validation runs
- THEN validation MUST fail with deterministic field-scoped diagnostics

### Requirement: Producer linkage is exact [r[valence.nostr_identity_evidence.producer_linkage]]
r[valence.nostr_identity_evidence.producer_linkage]

Valence MUST link the UCAN credential and Basalt binding through exact event, key, request, audience, purpose, subject, binding, policy, and schema identities.

#### Scenario: Exact authentication and binding link

- GIVEN the credential and binding cite the same Nostr key, event, request, audience, and purpose
- AND the binding cites the reviewed Basalt policy and subject
- WHEN linkage validation runs
- THEN Valence MUST produce one deterministic linked record

#### Scenario: Cross-record mismatch fails

- GIVEN any event, key, request, audience, purpose, subject, binding, policy, schema, or optional authorization identity differs
- WHEN linkage validation runs
- THEN validation MUST fail
- AND no fallback or nearest match MUST be selected

### Requirement: Protocol and evidence digest roles remain distinct [r[valence.nostr_identity_evidence.digest_domains]]
r[valence.nostr_identity_evidence.digest_domains]

Valence MUST preserve Nostr SHA-256 event identities separately from Valence-owned BLAKE3 artifact and receipt identities.

#### Scenario: Valid digest roles coexist

- GIVEN a linked record contains a Nostr event SHA-256 ID and Valence BLAKE3 IDs
- WHEN digest validation runs
- THEN each value MUST retain its declared role
- AND Valence MUST NOT replace the protocol ID with a rehash

#### Scenario: Cross-domain substitution fails

- GIVEN a SHA-256 value appears in a BLAKE3 role or a BLAKE3 value appears as the Nostr event ID
- WHEN digest validation runs
- THEN validation MUST fail with a digest-role diagnostic

### Requirement: Identity evidence remains recorded-only [r[valence.nostr_identity_evidence.role_policy]]
r[valence.nostr_identity_evidence.role_policy]

Valence MUST classify Nostr authentication, subject binding, and authorization composition evidence as `RecordedOnly`.

#### Scenario: Valid record preserves bounded claims

- GIVEN reviewed producers report a valid authentication and binding decision
- WHEN Valence creates Evidence IR
- THEN the artifact MUST remain `RecordedOnly`
- AND it MAY retain the producer decision as linked metadata

#### Scenario: Signature or binding requests promotion

- GIVEN an artifact claims that a signature, key, relay, binding, or authorization decision proves identity, correctness, authority, or release readiness
- WHEN role validation runs
- THEN validation MUST fail
- AND no `Property` count MUST increase

### Requirement: The shell remains capability-rooted and offline [r[valence.nostr_identity_evidence.shell_boundary]]
r[valence.nostr_identity_evidence.shell_boundary]

The Valence shell MUST read declared producer artifacts through capability roots. It MUST NOT contact relays, DNS, or signers.

#### Scenario: Local validation succeeds

- GIVEN supported producer artifacts exist beneath declared roots
- WHEN the CLI validates and writes the Evidence IR artifact
- THEN output MUST be deterministic for equivalent inputs
- AND no network or signing effect MUST occur

#### Scenario: Input escapes its root

- GIVEN a locator is absolute, traverses a parent, or uses an undeclared root
- WHEN the CLI plans the read
- THEN it MUST fail before opening the input or writing output

### Requirement: Sensitive material is rejected [r[valence.nostr_identity_evidence.privacy]]
r[valence.nostr_identity_evidence.privacy]

The profile MUST reject private keys, NIP-46 secrets, credentials, proof bytes, challenges, encrypted payloads, and secret metadata.

#### Scenario: Public bounded metadata passes

- GIVEN an artifact contains public keys, stable identifiers, decisions, reasons, and required non-claims only
- WHEN privacy validation runs
- THEN validation MAY pass

#### Scenario: Secret-bearing field fails

- GIVEN an artifact contains excluded private, credential, proof, challenge, signer, or encrypted material
- WHEN privacy validation runs
- THEN validation MUST fail before canonical identity or publication

### Requirement: Positive and negative evidence is complete [r[valence.nostr_identity_evidence.final_validation]]
r[valence.nostr_identity_evidence.final_validation]

The change MUST cover every profile, producer, digest, linkage, role, shell, privacy, and non-claim boundary before archive.

#### Scenario: Complete focused matrix passes

- GIVEN core, CLI, fixtures, documentation, and producer schema pins are complete
- WHEN format, workspace, Clippy, purity, Cairn, and Nix verification runs
- THEN valid fixtures MUST pass
- AND every malformed, stale, tampered, mismatched, secret-bearing, or overclaiming fixture MUST fail as declared
