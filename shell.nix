{
  clippy,
  rustfmt,
  callPackage,
  rust-analyzer,
}:
let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
    clippy
    rustfmt
    rust-analyzer
  ] ++ (oa.nativeBuildInputs or [ ]);
})
