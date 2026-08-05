{
  description = "Valence Evidence IR";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.artifact = {
    url = "git+ssh://git@github.com/OnixResearch/onix-artifact.git?rev=e41340bec587b6d049b5cc518ec7db925dde84be";
    flake = false;
  };

  outputs =
    {
      self,
      nixpkgs,
      artifact,
    }:
    let
      artifactSourcePin = import ./nix/artifact-source-pin.nix {
        lib = nixpkgs.lib;
      };
      artifactSourceHashes = {
        core = "839ea3e35429e2a1252020dd779b46ebd185b6911674036820da118d487d1d21";
        ed25519 = "7fd3987fe3ddc6d30efca37878952135f19905a00b52f349db237054be6b3633";
      };
      artifactSourcePinInput = {
        cargoWorkspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        coreManifest = builtins.fromTOML (builtins.readFile ./crates/valence-core/Cargo.toml);
        radicleManifest = builtins.fromTOML (builtins.readFile ./crates/valence-radicle/Cargo.toml);
        cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
        flakeLock = builtins.fromJSON (builtins.readFile ./flake.lock);
        releaseProfile = builtins.fromJSON (builtins.readFile "${artifact}/release/profile.json");
        sourceWorkspaceManifest = builtins.fromTOML (builtins.readFile "${artifact}/Cargo.toml");
        sourceCoreManifest = builtins.fromTOML (
          builtins.readFile "${artifact}/crates/artifact-auth-core/Cargo.toml"
        );
        sourceEd25519Manifest = builtins.fromTOML (
          builtins.readFile "${artifact}/crates/artifact-auth-ed25519/Cargo.toml"
        );
        resolvedNixRevision = artifact.rev or null;
        resolvedNixNarHash = artifact.narHash or null;
      };
      artifactSourceValidation = artifactSourcePin.validate artifactSourcePinInput;
      artifactSourceAdmitted =
        assert nixpkgs.lib.assertMsg artifactSourceValidation.valid (
          "Valence Artifact source pin drifted: "
          + nixpkgs.lib.concatStringsSep ", " artifactSourceValidation.issues
        );
        true;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = eachSystem (pkgs: {
        valence = pkgs.rustPlatform.buildRustPackage {
          pname = "valence";
          version = "0.1.0";
          src = self;
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = artifactSourceAdmitted;
          };
          cargoBuildFlags = [
            "-p"
            "valence-cli"
          ];
          doCheck = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ ];
        };
        valence-radicle = pkgs.rustPlatform.buildRustPackage {
          pname = "valence-radicle";
          version = "0.1.0";
          src = self;
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = artifactSourceAdmitted;
          };
          cargoBuildFlags = [
            "-p"
            "valence-radicle"
          ];
          cargoTestFlags = [
            "-p"
            "valence-radicle"
          ];
          doCheck = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ ];
        };
        valence-radicle-native = pkgs.rustPlatform.buildRustPackage {
          pname = "valence-radicle-native";
          version = "0.1.0";
          src = self;
          cargoRoot = "crates/valence-radicle-native";
          buildAndTestSubdir = "crates/valence-radicle-native";
          cargoLock.lockFile = ./crates/valence-radicle-native/Cargo.lock;
          doCheck = true;
          nativeBuildInputs = [
            pkgs.git
            pkgs.pkg-config
          ];
          buildInputs = [ ];
        };
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.valence;
      });

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustup
            pkg-config
            clippy
            rustfmt
            nickel
          ];
        };
      });

      checks = eachSystem (pkgs: {
        valence-test = pkgs.rustPlatform.buildRustPackage {
          pname = "valence-test";
          version = "0.1.0";
          src = self;
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = artifactSourceAdmitted;
          };
          doCheck = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ ];
        };

        artifact-auth-radicle-cutover-history =
          pkgs.runCommand "valence-artifact-auth-radicle-cutover-history"
            {
              nativeBuildInputs = [
                pkgs.b3sum
                pkgs.jq
                pkgs.nickel
              ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              nickel typecheck evidence/radicle/artifact-auth-cutover-v1.ncl
              nickel typecheck lib/artifact-auth-cutover-receipt.ncl
              nickel export --format json tests/artifact-auth-cutover.ncl > "$TMPDIR/tests.json"
              grep -Fq '"tests": true' "$TMPDIR/tests.json"

              nickel export --format json evidence/radicle/artifact-auth-cutover-v1.ncl > "$TMPDIR/cutover.json"
              jq -S . "$TMPDIR/cutover.json" > "$TMPDIR/cutover.normalized.json"
              jq -S . evidence/radicle/artifact-auth-cutover-v1.json > "$TMPDIR/evidence.normalized.json"
              cmp "$TMPDIR/cutover.normalized.json" "$TMPDIR/evidence.normalized.json"

              receipt_hash="$(b3sum evidence/radicle/artifact-auth-cutover-v1.json | cut -d ' ' -f 1)"
              expected_receipt_hash="$(tr -d '\n' < evidence/radicle/artifact-auth-cutover-v1.blake3)"
              test "$receipt_hash" = "$expected_receipt_hash"

              touch "$out"
            '';

        artifact-auth-source-pin =
          let
            validation = artifactSourceValidation;
            selfTestIssues = artifactSourcePin.selfTest artifactSourcePinInput;
            issues = validation.issues ++ selfTestIssues;
          in
          if issues != [ ] then
            throw "artifact source pin invalid:\n${nixpkgs.lib.concatStringsSep "\n" issues}"
          else
            pkgs.runCommand "valence-artifact-auth-source-pin"
              {
                nativeBuildInputs = [ pkgs.b3sum ];
              }
              ''
                # r[verify valence.artifact_auth_adoption.radicle_transport]
                core_hash="$(b3sum ${artifact}/crates/artifact-auth-core/src/lib.rs | cut -d ' ' -f 1)"
                ed25519_hash="$(b3sum ${artifact}/crates/artifact-auth-ed25519/src/lib.rs | cut -d ' ' -f 1)"
                test "$core_hash" = '${artifactSourceHashes.core}'
                test "$ed25519_hash" = '${artifactSourceHashes.ed25519}'

                cp ${artifact}/crates/artifact-auth-core/src/lib.rs "$TMPDIR/core.rs"
                chmod u+w "$TMPDIR/core.rs"
                printf '\n' >> "$TMPDIR/core.rs"
                mutated_core_hash="$(b3sum "$TMPDIR/core.rs" | cut -d ' ' -f 1)"
                test "$mutated_core_hash" != '${artifactSourceHashes.core}'

                printf '%s\n' '${validation.revision}' > "$out"
              '';

        artifact-source-migration =
          assert artifactSourceAdmitted;
          pkgs.runCommand "valence-artifact-source-migration"
            {
              nativeBuildInputs = [
                pkgs.b3sum
                pkgs.jq
                pkgs.nickel
                pkgs.ripgrep
              ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              # r[verify valence.artifact_auth_adoption.radicle_evidence]
              nickel typecheck evidence/source/artifact-workspace-migration-v1.ncl
              nickel typecheck lib/artifact-source-migration-receipt.ncl
              nickel export --format json tests/artifact-source-migration.ncl > "$TMPDIR/tests.json"
              grep -Fq '"tests": true' "$TMPDIR/tests.json"

              nickel export --format json evidence/source/artifact-workspace-migration-v1.ncl > "$TMPDIR/migration.json"
              jq -S . "$TMPDIR/migration.json" > "$TMPDIR/migration.normalized.json"
              jq -S . evidence/source/artifact-workspace-migration-v1.json > "$TMPDIR/evidence.normalized.json"
              cmp "$TMPDIR/migration.normalized.json" "$TMPDIR/evidence.normalized.json"

              receipt_hash="$(b3sum evidence/source/artifact-workspace-migration-v1.json | cut -d ' ' -f 1)"
              expected_receipt_hash="$(tr -d '\n' < evidence/source/artifact-workspace-migration-v1.blake3)"
              test "$receipt_hash" = "$expected_receipt_hash"

              jq -r '.evidence[]' evidence/source/artifact-workspace-migration-v1.json \
                | while IFS= read -r evidence_path; do
                  if ! test -f "$evidence_path"; then
                    echo "migration evidence path is missing: $evidence_path" >&2
                    exit 1
                  fi
                done

              predecessor_owner='OnixResearch'
              predecessor_repository='artifact-auth'
              forbidden_github_source="$predecessor_owner/$predecessor_repository"
              if rg -F "$forbidden_github_source" Cargo.toml Cargo.lock flake.nix flake.lock; then
                echo 'executable predecessor artifact-auth source remains' >&2
                exit 1
              fi

              radicle_host='git.onix.computer'
              predecessor_rid='z4JGYYW7WsesXUq7MXVdx16Fawu2f'
              forbidden_radicle_source="$radicle_host/$predecessor_rid"
              if rg -F "$forbidden_radicle_source" Cargo.toml Cargo.lock flake.nix flake.lock; then
                echo 'executable predecessor Radicle source remains' >&2
                exit 1
              fi

              if rg -F 'artifact-transfer-core' Cargo.lock; then
                echo 'artifact-transfer-core entered the Valence consumer graph' >&2
                exit 1
              fi

              touch "$out"
            '';

        build-service-nickel-contracts =
          pkgs.runCommand "build-service-nickel-contracts"
            {
              nativeBuildInputs = [ pkgs.nickel ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              for fixture in contracts/build-service-evidence/fixtures/pass/*.ncl; do
                nickel export "$fixture" >/dev/null
              done

              for fixture in contracts/build-service-evidence/fixtures/fail/*.ncl; do
                if nickel export "$fixture" >/dev/null 2>"$TMPDIR/nickel-error.log"; then
                  echo "expected Nickel fixture to fail: $fixture" >&2
                  exit 1
                fi
              done

              touch "$out"
            '';

        stack-provenance-nickel-contracts =
          pkgs.runCommand "stack-provenance-nickel-contracts"
            {
              nativeBuildInputs = [ pkgs.nickel ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              for fixture in contracts/stack-provenance/fixtures/pass/*.ncl; do
                nickel export "$fixture" >/dev/null
              done

              for fixture in contracts/stack-provenance/fixtures/fail/*.ncl; do
                if nickel export "$fixture" >/dev/null 2>"$TMPDIR/nickel-error.log"; then
                  echo "expected Nickel fixture to fail: $fixture" >&2
                  exit 1
                fi
              done

              touch "$out"
            '';

        aeneas-translation-chain-fixtures =
          pkgs.runCommand "valence-aeneas-translation-chain-fixtures"
            {
              nativeBuildInputs = [ pkgs.nickel ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              nickel typecheck fixtures/aeneas-translation-chain/chain.ncl
              nickel typecheck fixtures/aeneas-translation-chain/negative-cases.ncl
              nickel export --format json fixtures/aeneas-translation-chain/chain.ncl > "$TMPDIR/chain.json"
              nickel export --format json fixtures/aeneas-translation-chain/negative-cases.ncl > "$TMPDIR/negative-cases.json"
              cmp "$TMPDIR/chain.json" fixtures/aeneas-translation-chain/chain.json
              cmp "$TMPDIR/negative-cases.json" fixtures/aeneas-translation-chain/negative-cases.json

              touch "$out"
            '';

        radicle-cob-evidence-profile =
          pkgs.runCommand "valence-radicle-cob-evidence-profile"
            {
              nativeBuildInputs = [ pkgs.nickel ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              nickel typecheck contracts/radicle-cob-evidence/profile.ncl
              nickel export --format json contracts/radicle-cob-evidence/profile.ncl > "$TMPDIR/profile.json"
              cmp "$TMPDIR/profile.json" contracts/radicle-cob-evidence/generated/profile.json

              for fixture in contracts/radicle-cob-evidence/fixtures/pass/*.ncl; do
                nickel export "$fixture" >/dev/null
              done

              for fixture in contracts/radicle-cob-evidence/fixtures/fail/*.ncl; do
                if nickel export "$fixture" >/dev/null 2>"$TMPDIR/nickel-error.log"; then
                  echo "expected Radicle COB evidence fixture to fail: $fixture" >&2
                  exit 1
                fi
              done

              touch "$out"
            '';

        radicle-forge-operations =
          pkgs.runCommand "valence-radicle-forge-operations"
            {
              nativeBuildInputs = [
                pkgs.jq
                pkgs.nickel
                self.packages.${pkgs.stdenv.hostPlatform.system}.valence-radicle
                self.packages.${pkgs.stdenv.hostPlatform.system}.valence-radicle-native
              ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              nickel typecheck contracts/radicle-forge-operations/profile.ncl
              nickel export --format json contracts/radicle-forge-operations/profile.ncl > "$TMPDIR/profile.json"
              cmp "$TMPDIR/profile.json" contracts/radicle-forge-operations/generated/profile.json

              for fixture in contracts/radicle-forge-operations/fixtures/pass/*.ncl; do
                nickel export "$fixture" >/dev/null
              done
              for fixture in contracts/radicle-forge-operations/fixtures/fail/*.ncl; do
                if nickel export "$fixture" >/dev/null 2>"$TMPDIR/nickel-error.log"; then
                  echo "expected Radicle forge operations fixture to fail: $fixture" >&2
                  exit 1
                fi
              done

              mkdir "$TMPDIR/output"
              valence-radicle forge-plan \
                --profile contracts/radicle-forge-operations/generated/profile.json \
                --source fixtures/radicle-forge-operations/source-batch.json \
                --output-root "$TMPDIR/output" \
                --output migration-plan.json >/dev/null
              cmp "$TMPDIR/output/migration-plan.json" \
                fixtures/radicle-forge-operations/migration-plan.json
              valence-radicle-native import \
                --repository /nonexistent-preview-repository \
                --plan "$TMPDIR/output/migration-plan.json" \
                --output-root "$TMPDIR/output" \
                --output native-observations.json \
                > "$TMPDIR/native-preview.json"
              test "$(jq -r '.effects' "$TMPDIR/native-preview.json")" = no-write-preview
              test ! -e "$TMPDIR/output/native-observations.json"

              valence-radicle ci-admission \
                --input fixtures/radicle-forge-operations/ci-admission-input.json \
                --output-root "$TMPDIR/output" \
                --output ci-admission-report.json >/dev/null
              cmp "$TMPDIR/output/ci-admission-report.json" \
                fixtures/radicle-forge-operations/ci-admission-report.json
              jq '.receipt' "$TMPDIR/output/ci-admission-report.json" > "$TMPDIR/admission-receipt.json"
              valence-radicle verify-ci-admission --receipt "$TMPDIR/admission-receipt.json" >/dev/null

              jq '.result.disposition = "timed_out" | .result.exit_code = null' \
                fixtures/radicle-forge-operations/ci-admission-input.json \
                > "$TMPDIR/failed-admission.json"
              if valence-radicle ci-admission \
                --input "$TMPDIR/failed-admission.json" \
                --output-root "$TMPDIR/output" \
                --output rejected-admission.json >/dev/null; then
                echo "expected failed CI result to be denied" >&2
                exit 1
              fi
              test "$(jq -r '.admitted' "$TMPDIR/output/rejected-admission.json")" = false
              test "$(jq -r '.receipt' "$TMPDIR/output/rejected-admission.json")" = null

              touch "$out"
            '';

        cairn-policy-compatibility-inputs =
          pkgs.runCommand "valence-cairn-policy-compatibility-inputs"
            {
              nativeBuildInputs = [
                pkgs.b3sum
                pkgs.nickel
              ];
              src = self;
            }
            ''
              set -eu
              cd "$src"

              nickel export --format json cairn-policy/default.ncl > "$TMPDIR/cairn-policy.json"
              nickel export --format json cairn-policy/stack-provenance-authentication-profile.ncl > "$TMPDIR/stack-profile.json"
              nickel export --format json cairn-policy/function-address-authentication-profile.ncl > "$TMPDIR/function-profile.json"

              cmp "$TMPDIR/cairn-policy.json" cairn-policy/generated/cairn-policy.json
              cmp "$TMPDIR/stack-profile.json" cairn-policy/generated/stack-provenance-authentication-profile.json
              cmp "$TMPDIR/function-profile.json" cairn-policy/generated/function-address-authentication-profile.json

              cat > "$TMPDIR/authenticated-policy-hashes" <<'EOF'
              a1c63619b0f72140c6e350b61ed18d3d552310067406ba0b819ace462d1f1e48  cairn-policy/evidence/valence-preserves-integrity-archive-receipt.json
              93993e99e87206d232b64efcc8631611aad3b5b0c4b81855e7e1a0cdcc391cff  cairn-policy/generated/stack-provenance-authentication-profile.json
              62c88a0263720dea4c37a48ef46921c46e5dd3e4924fff096b84a65d5738a412  cairn-policy/generated/function-address-authentication-profile.json
              EOF
              sed -i 's/^              //' "$TMPDIR/authenticated-policy-hashes"
              b3sum --check "$TMPDIR/authenticated-policy-hashes"

              touch "$out"
            '';

        effect-closure-policy =
          pkgs.runCommand "valence-effect-closure-policy"
            {
              nativeBuildInputs = [ pkgs.nickel ];
              src = self;
            }
            ''
              set -eu
              cd "$src"
              generated_policy=$(mktemp)
              nickel typecheck policy/octet-effect-closure.ncl
              nickel export --format json policy/octet-effect-closure.ncl > "$generated_policy"
              cmp "$generated_policy" octet-effect-closure.json
              touch "$out"
            '';
      });
    };
}
