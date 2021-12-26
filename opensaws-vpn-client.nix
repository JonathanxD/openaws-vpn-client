with import <nixpkgs> {
  overlays = [
    (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
};

(makeRustPlatform {
  rustc = rust-bin.nightly."2021-12-04".default;
  cargo = rust-bin.nightly."2021-12-04".default;
}).buildRustPackage rec {
  pname = "openaws-vpn-client";
  version = "0.1.4";

  buildInputs = [
    pkgconfig
    glib.dev
    gtk3.dev
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  src = fetchFromGitHub {
    owner = "JonathanxD";
    repo = pname;
    rev = "${version}";
    sha256 = "wByqNbgTFtoZpaGjwq66PKxGOQU9j22UcKgkxclQ1Ew=";
  };

  cargoSha256 = "nzBHdS4TK0oYM+GbliTvgIosQ6FL/36qyFUDS//oteE=";

  meta = with lib; {
    description = "Unofficial open-source AWS VPN client written in Rust";
    homepage = "https://github.com/JonathanxD/opensaws-vpn-client";
    license = licenses.mit;
    maintainers = with maintainers; [
      {
        name = "Jonathan H. R. Lopes";
        github = "JonathanxD";
        githubId = 5360060;
        matrix = "@jonathanhrl:matrix.org";
        email = "joniweb01@gmail.com";
        keys = [{
            longkeyid = "rsa3072/0x4DF5FC43FD4FE9CC";
            fingerprint = "B2C2 7303 C091 E72F C62A  AB69 4DF5 FC43 FD4F E9CC";
        }];
      }
    ];
  };
}
