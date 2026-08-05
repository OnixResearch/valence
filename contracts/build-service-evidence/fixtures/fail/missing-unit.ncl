let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
base
& {
  resource_observations | force = [
    (std.array.at 0 base.resource_observations) & { measurements | force = [{ name = "wall-clock", value = 42, unit = "" }] }
  ],
}
| Contracts.Bundle
