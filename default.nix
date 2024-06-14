{ lib, rustPlatform }:
let
  toml = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = "catppuccin-whiskers";
  inherit (toml) version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
      lib.fileset.unions [
        ./Cargo.toml
        ./Cargo.lock
        ./src
        ./tests
        ./LICENSE
      ]
    );
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    inherit (toml) homepage description;
    license = lib.licenses.mit;
    mainProgram = "whiskers";
  };
}
