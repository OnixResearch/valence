let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base & { service_requests | force = [(std.array.at 0 base.service_requests) & { concrete_request_hash | force = "not-hex" }] } | Contracts.Bundle
