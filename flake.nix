{
  description = "Official Beyond Identity command-line interface";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          rustc = toolchain;
          cargo = toolchain;
          clippy = toolchain;
        };

        darwinBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
          pkgs.darwin.apple_sdk.frameworks.Security
        ];

        buildInputs = [ pkgs.zlib ] ++ darwinBuildInputs;
      in
      rec {
        packages = {
          default = naersk'.buildPackage {
            inherit buildInputs;
            name = "bi";
            src = ./.;
            mode = "build";
          };

          clippy = naersk'.buildPackage {
            inherit buildInputs;
            src = ./.;
            mode = "clippy";
          };
        };

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = [ toolchain ] ++ darwinBuildInputs;
          };

          bi = pkgs.mkShell {
            nativeBuildInputs = [ toolchain ] ++ darwinBuildInputs;
            buildInputs = [ packages.default ];
          };
        };
      }
    );
}
