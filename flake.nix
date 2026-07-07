{
  description = "Valence Evidence IR";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
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
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [
            "-p"
            "valence-cli"
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
          cargoLock.lockFile = ./Cargo.lock;
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
      });
    };
}
