{
  description = "Environment for my typing game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        speedTyping = rustPlatform.buildRustPackage {
          pname = "speed_typing";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

      in
      {
        defaultPackage = speedTyping;

        devShell = pkgs.mkShell {
          buildInputs = [ (rustVersion.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          }) pkgs.trunk ];
        };

      });
}
