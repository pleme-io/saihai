{
  description = "saihai — the typed desktop action catalog. Every action a pleme-io desktop can perform is one (defaction …) row carrying its authority rung and whether its effect is observable, from which SDKs and a convergence loop are generated.";
  inputs = {
    nixpkgs = {
      follows = "substrate/nixpkgs";
    };
    crate2nix = {
      url = "github:nix-community/crate2nix";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    substrate = {
      url = "github:pleme-io/substrate";
    };
  };
  outputs = inputs @ { self, nixpkgs, crate2nix, flake-utils, substrate, ... }:
    (import "${substrate}/lib/rust-workspace-release-flake.nix" {
      inherit nixpkgs crate2nix flake-utils;
    }) {
      toolName = "bancadad";
      packageName = "bancadad";
      src = self;
      repo = "pleme-io/saihai";
    };
}
