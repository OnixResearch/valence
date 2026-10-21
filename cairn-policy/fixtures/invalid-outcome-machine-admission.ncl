let Contracts = import "../contracts.ncl" in
let base = import "../default.ncl" in
base
& {
  schemas = base.schemas
    |> std.array.map (fun schema =>
      if schema.id == "outcome-machine" then
        schema & {
          artifacts = schema.artifacts
            |> std.array.map (fun artifact =>
              if artifact.id == "outcome-machine" then
                artifact & { structural_admission = "arbitrary_execution" }
              else
                artifact
            ),
        }
      else
        schema
    ),
}
| Contracts.Policy
