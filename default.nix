{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "catppuccin-whiskers";
  inherit ((lib.importTOML ./Cargo.toml).package) version;

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
    homepage = "https://github.com/catppuccin/whiskers";
    description = "😾 Soothing port creation tool for the high-spirited!";
    license = lib.licenses.mit;
    mainProgram = "whiskers";
  };
}
