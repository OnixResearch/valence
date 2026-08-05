let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base & { profile.non_claims | force = [] } | Contracts.Bundle
