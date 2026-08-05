let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base
& {
  reuse_edges | force = [
    (std.array.at 0 base.reuse_edges) & { authorization_decision_identity | force = null }
  ],
}
| Contracts.Bundle
| Contracts.ReuseUsabilityRule
