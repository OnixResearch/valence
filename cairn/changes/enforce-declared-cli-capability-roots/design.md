## Context

`split_operator_path` currently turns an operator path into `(parent, file_name)`, then `open_capability_root` opens that parent with ambient authority. For an absolute locator, the absolute parent becomes a newly authorized root. The existing negative unit test exercises `validate_capability_relative_path` directly, but normal command paths pass only the extracted file name to that validator, so the public shell bypasses the intended absolute-path rejection.

## Decisions

### Declare authority independently from the locator

**Choice:** Every filesystem role is represented by a typed root declaration and a separate relative locator. The CLI may use role-specific root options or an equivalent typed authority object, but it must not infer a root from the locator being opened.

A command that needs more than one role opens each declared root once and reuses its `cap_std::fs::Dir` handle. An operator may explicitly choose the same directory for multiple roles, but the handles and operation permissions remain role-typed.

### Validate a pure access plan before I/O

**Choice:** Add a pure deterministic access-plan core containing the role, operation, root declaration identity, and relative locator. It rejects empty locators, absolute locators, platform prefixes, parent traversal, and operation/role mismatches before the shell opens a directory.

The root path itself is an explicit shell input and may be absolute. Only the file locator is capability-relative. Nested relative locators remain supported so a single declared root can safely contain an evidence tree.

### Perform capability-relative operations without ambient re-resolution

**Choice:** After opening the declared root, reads, measurements, and writes use the relative locator through that handle. The shell must not canonicalize an ambient path and then reopen it by ambient name. `cap-std` errors remain I/O diagnostics; pure plan failures remain deterministic validation diagnostics.

### Fail closed rather than preserve parent-derived compatibility

**Choice:** Omitted role roots and absolute file locators fail. Silent fallback to the current working directory or to `path.parent()` is not allowed. Documentation provides command migration examples, but no compatibility mode may recreate parent-derived ambient authority on evidence-producing paths.

### Keep publication atomicity separate

This change establishes authority selection and confinement. Atomic multi-artifact publication is handled by `fail-closed-function-address-publication`; this package does not claim that a confined write is transactional.

## Risks / Trade-offs

- Existing scripts using absolute file arguments must be updated, but retaining those arguments as implicit authority would preserve the flaw.
- More root options increase CLI verbosity; typed root declarations make that authority visible and reviewable.
- Capability confinement cannot prove that the explicitly supplied root itself is trustworthy or that host filesystem semantics are race-free in every environment.
