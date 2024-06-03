{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      inherit (nixpkgs) lib;
      forEachSystem =
        f:
        (lib.listToAttrs (
          map (system: {
            name = system;
            value = f {
              inherit system;
              pkgs = import nixpkgs {
                inherit system;
                overlays = [ rust-overlay.overlays.default ];
              };
            };
          }) systems
        ));
    in
    {
      devShells = forEachSystem (
        { pkgs, system }:
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];

            packages = [
              (pkgs.rust-bin.stable.latest.default.override {
                extensions = [
                  "rustfmt"
                  "rust-analyzer"
                  "clippy"
                ];
              })
            ];
          };
        }
      );

      packages = forEachSystem (
        { pkgs, system }:
        {
          default = self.packages.${system}.whiskers;
          whiskers = pkgs.callPackage ./default.nix {
            rustPlatform =
              let
                toolchain = pkgs.rust-bin.stable.latest.default;
              in
              pkgs.makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
              };
          };
        }
      );

      overlays.default = final: _: { catppuccin-whiskers = final.callPackage ./default.nix { }; };
    };

  nixConfig = {
    extra-substituters = [ "https://catppuccin.cachix.org" ];
    extra-trusted-public-keys = [
      "catppuccin.cachix.org-1:noG/4HkbhJb+lUAdKrph6LaozJvAeEEZj4N732IysmU="
    ];
  };
}
