{
  description = "Valence Evidence IR";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in {
      packages = eachSystem (pkgs: {
        valence = pkgs.rustPlatform.buildRustPackage {
          pname = "valence";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "-p" "valence-cli" ];
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
      });
    };
}
