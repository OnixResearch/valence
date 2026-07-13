# Evidence span integrity

`EvidenceSpan` byte offsets are zero-based and use an exclusive end. Line and column coordinates are one-based. A span is valid only when required text is non-empty, unpadded, and free of control characters; byte and source positions are ordered; and every line and column is nonzero. A zero-width span is valid when both byte endpoints and both line/column endpoints are equal.

Valence validates every row before manifest lookup and requires each `source_id` to occur exactly once. A valid manifest without the requested ID yields `NotFound`. Malformed JSON, an invalid row, or any duplicate ID yields `InvalidManifest` with sorted field-scoped diagnostics; no row is selected from invalid input.

Artifact construction and checking apply the same validator to attached spans even though span display metadata is excluded from canonical artifact identity.

These checks establish structural validity and unambiguous lookup only. Valence does not open `source_path`, compare offsets with measured source bytes, interpret Unicode columns according to a compiler convention, prove compiler provenance, or establish that linked code is correct.
