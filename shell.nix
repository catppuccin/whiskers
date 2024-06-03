{ callPackage, toolchain }:
let
  mainPkg = callPackage ./default.nix { };
  toolchainWithComponents = (
    toolchain.stable.latest.default.override {
      extensions = [
        "rustfmt"
        "rust-analyzer"
        "clippy"
      ];
    }
  );
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [ toolchainWithComponents ] ++ (oa.nativeBuildInputs or [ ]);
})
