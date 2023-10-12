{ fetchFromGitHub
, lib
, glib
, gtk3
, pkg-config
, makeRustPlatform
, rust-bin
}:
let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);
    rustc = rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);
  };
in rustPlatform.buildRustPackage rec {
  pname = "openaws-vpn-client";
  version = "0.1.7";

  buildInputs = [
    pkg-config
    glib.dev
    gtk3.dev
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  src = ./.;

  cargoHash = "sha256-yjhGDiO0pMVw9KFEUbCCF16uPfuusrxBKbFQcHlKYqY=";

  meta = with lib; {
    description = "Unofficial open-source AWS VPN client written in Rust";
    homepage = "https://github.com/JonathanxD/openaws-vpn-client";
    license = licenses.mit;
  };
}
