{
    description = "devshell for my personal website";

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
        formatter = forAllSystems (
            system: let
                pkgs = nixpkgs.legacyPackages.${system};
            in
                pkgs.alejandra
        );
        devShells = forAllSystems (
            system: let
                pkgs = nixpkgs.legacyPackages.${system};
            in {
                default = pkgs.mkShell {
                    packages = builtins.attrValues {
                        inherit
                            (pkgs)
                            nodejs
                            pnpm
                            prettierd
                            typescript-language-server
                            tailwindcss-language-server
                            astro-language-server
                            ;
                    };
                };
            }
        );
    };
}
