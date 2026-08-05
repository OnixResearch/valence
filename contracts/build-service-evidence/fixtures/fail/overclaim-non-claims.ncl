let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base & { profile.non_claims | force = ["build-service linkage proves canonical identity and bounded observation linkage only; it does not prove build correctness, hermeticity, sandboxing, host isolation, identity beyond checked claims, authorization freshness, future resource sufficiency, accounting fairness, external status correctness, or release eligibility", "this profile proves release eligibility"] } | Contracts.Bundle
