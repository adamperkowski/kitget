{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}:

let
  p = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = p.name;
  inherit (p) version;

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  doCheck = false;

  meta = {
    inherit (p) description homepage;
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [ koi ];
    mainProgram = "kitget";
  };
}
