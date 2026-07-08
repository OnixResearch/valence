## Design

Add an adapter contract that maps selected ChaosControl receipt families into Valence `EvidenceArtifact` fields: schema, artifact kind, stable ID, canonical hash, source span or source ID, verification role, payload digest, and non-claims.

The pure core validates parsed adapter rows and non-claims. The shell or external producer measures ChaosControl artifact bytes. Validation rejects missing schema, unsupported artifact kind, malformed hash, missing non-claims, or role promotion from recorded evidence to property evidence.
