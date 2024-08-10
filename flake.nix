{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        stable = pkgs.callPackage ./package.nix { };
        latest = pkgs.rustPlatform.buildRustPackage {
          pname = "mekuteriya";
          version = "latest";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      in
      {
        packages = {
          inherit stable;
          default = latest;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-bin.stable.latest.default
          ];
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
