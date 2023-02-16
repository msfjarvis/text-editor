{
  description = "text-editor";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};
    flake-utils = {url = "github:numtide/flake-utils";};
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        flake-compat.follows = "flake-compat";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustStable =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      src = ./.;
      cargoArtifacts = craneLib.buildDepsOnly {inherit src buildInputs;};
      buildInputs = [];

      text-editor = craneLib.buildPackage {
        inherit src;
        doCheck = false;
      };
      text-editor-clippy = craneLib.cargoClippy {
        inherit cargoArtifacts src buildInputs;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };
      text-editor-fmt = craneLib.cargoFmt {inherit src;};
      text-editor-audit = craneLib.cargoAudit {inherit src advisory-db;};
      text-editor-nextest = craneLib.cargoNextest {
        inherit cargoArtifacts src buildInputs;
        partitions = 1;
        partitionType = "count";
      };
    in {
      checks = {
        inherit
          text-editor
          text-editor-audit
          text-editor-clippy
          text-editor-fmt
          text-editor-nextest
          ;
      };

      packages.default = text-editor;

      apps.default = flake-utils.lib.mkApp {drv = text-editor;};

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        nativeBuildInputs = with pkgs; [
          cargo-nextest
          cargo-release
          rustStable
        ];
      };
    });
}
