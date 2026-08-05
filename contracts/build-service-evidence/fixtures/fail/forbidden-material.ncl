let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base & { authority_decisions | force = [(std.array.at 0 base.authority_decisions) & { subject_projection | force = "Bearer abc.def.ghi" }] } | Contracts.Bundle
