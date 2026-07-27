# Content-Bound Requirements Fixture Specification

## Purpose

Provide one frozen requirement for Valence content-bound reference tests.

## Requirements

### Requirement: Content-bound reference fixture

r[valence.provenance.content_bound_requirement_ref] Valence MUST preserve every typed requirement identity domain.

#### Scenario: Validate the frozen reference

- GIVEN a reference built from the frozen Cairn registry
- WHEN Valence validates the reference
- THEN every identity MUST match the registry row
