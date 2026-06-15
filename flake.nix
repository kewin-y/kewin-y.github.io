{
  description = "devshell for pages";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    forAllSystems = nixpkgs.lib.genAttrs [
      "aarch64-linux"
      "x86_64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
  in {
    packages = forAllSystems (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        generator = pkgs.rustPlatform.buildRustPackage {
          pname = "generator";
          version = "0.1.0";
          cargoLock.lockFile = ./Cargo.lock;

          src = pkgs.lib.cleanSource ./.;
        };
        site = pkgs.stdenv.mkDerivation {
          name = "site";
          buildInputs = [generator];
          buildPhase = "generator";
          src = pkgs.lib.cleanSource ./.;
          installPhase = ''
            mkdir -p $out/public
            cp -r ./output/* $out/public
          '';
        };
        default = site;
      }
    );
    devShells = forAllSystems (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        default = pkgs.mkShell {
          buildInputs = builtins.attrValues {
            inherit
              (pkgs)
              cargo
              rustc
              rustfmt
              clippy
              rust-analyzer
              miniserve
              ;
          };

          env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
  };
}
