let Contracts = import "../contracts.ncl" in
let base = import "../default.ncl" in
let spec_driven = std.array.at 0 base.schemas in
let proposal = std.array.at 0 spec_driven.artifacts in
base
& {
  schemas = base.schemas
    |> std.array.map (fun schema =>
      if schema.id == "spec-driven" then
        schema & { artifacts = [std.record.remove "output_path" proposal] }
      else
        schema
    ),
}
| Contracts.Policy
