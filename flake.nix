{
  description = "Valence Evidence IR";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.artifact-auth-src = {
    url = "git+ssh://git@github.com/OnixResearch/artifact-auth.git?rev=799459346d5416fbd7b9f55840a7371441b55afa";
    flake = false;
  };

  outputs =
    {
      self,
      nixpkgs,
      artifact-auth-src,
    }:
    let
      artifactAuthRevision = "799459346d5416fbd7b9f55840a7371441b55afa";
      artifactAuthCargoDependency =
        (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.dependencies.artifact-auth-core;
      artifactAuthLockPackages = builtins.filter (
        package: package.name == "artifact-auth-core"
      ) (builtins.fromTOML (builtins.readFile ./Cargo.lock)).package;
      artifactAuthExpectedLockSource =
        "git+ssh://git@github.com/OnixResearch/artifact-auth.git?rev=${artifactAuthRevision}#${artifactAuthRevision}";
      artifactAuthWorkspace = builtins.fromTOML (builtins.readFile (artifact-auth-src + "/Cargo.toml"));
      artifactAuthSourceAdmitted =
        assert nixpkgs.lib.assertMsg (
          artifactAuthCargoDependency.git == "ssh://git@github.com/OnixResearch/artifact-auth.git"
          && artifactAuthCargoDependency.rev == artifactAuthRevision
          && artifact-auth-src.rev == artifactAuthRevision
          && builtins.length artifactAuthLockPackages == 1
          && (builtins.head artifactAuthLockPackages).source == artifactAuthExpectedLockSource
          && artifactAuthWorkspace.workspace.package.license == "MIT OR Apache-2.0"
        ) "Valence artifact-auth Cargo/Nix source identity, uniqueness, or license drifted";
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
            allowBuiltinFetchGit = artifactAuthSourceAdmitted;
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
            allowBuiltinFetchGit = artifactAuthSourceAdmitted;
          };
          cargoBuildFlags = [
            "-p"
            "valence-radicle"
          ];
          doCheck = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
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
            allowBuiltinFetchGit = artifactAuthSourceAdmitted;
          };
          doCheck = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ ];
        };

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
