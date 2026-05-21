{
  description = "Enteec server";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = packages;
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        };
      }
    );
}
