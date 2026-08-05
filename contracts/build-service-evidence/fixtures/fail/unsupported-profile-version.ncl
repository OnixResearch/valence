let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base & { profile.profile_version | force = 2 } | Contracts.Bundle
