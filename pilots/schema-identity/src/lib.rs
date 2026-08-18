//! Independent Valence pilot for the published schema identity cohort.
//!
//! This pilot checks deterministic fixture agreement only. It does not add
//! Evidence IR authority, prove schema meaning, or authorize migrations.

/// Runs the exact checked cohort carried by the pinned dependency.
///
/// # Errors
///
/// Returns an error when the published vector bytes or pure decisions drift.
pub fn verify_published_cohort() -> Result<(), schema_identity_conformance::FixtureError> {
    schema_identity_conformance::verify_fixture_json(
        schema_identity_conformance::CHECKED_VECTORS_JSON,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn published_cohort_agrees() -> Result<(), schema_identity_conformance::FixtureError> {
        super::verify_published_cohort()
    }

    #[test]
    fn substituted_fixture_fails_closed() {
        let substituted = r#"{"schema":"substituted","cases":[]}"#;
        assert!(schema_identity_conformance::verify_fixture_json(substituted).is_err());
    }
}
