let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base & { external_statuses | force = [(std.array.at 0 base.external_statuses) & { role | force = "verified" }] } | Contracts.Bundle
