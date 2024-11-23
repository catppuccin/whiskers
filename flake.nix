{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      forEachSystem = nixpkgs.lib.genAttrs systems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    {
      devShells = forEachSystem (system: {
        default = pkgsFor.${system}.mkShell {
          inputsFrom = [ self.packages.${system}.default ];

          packages = with pkgsFor.${system}; [
            cargo
            clippy
            rust-analyzer
            rustfmt
          ];
        };
      });

      packages = forEachSystem (system: {
        default = self.packages.${system}.whiskers;
        whiskers = pkgsFor.${system}.callPackage ./default.nix { };
      });

      overlays.default = final: _: { catppuccin-whiskers = final.callPackage ./default.nix { }; };
    };

  nixConfig = {
    extra-substituters = [ "https://catppuccin.cachix.org" ];
    extra-trusted-public-keys = [
      "catppuccin.cachix.org-1:noG/4HkbhJb+lUAdKrph6LaozJvAeEEZj4N732IysmU="
    ];
  };
}
