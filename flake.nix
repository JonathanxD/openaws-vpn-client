{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ (import nixpkgs-mozilla) ];
        };

        toolchain = (pkgs.rustChannelOf {
          date = "2023-10-11";
          channel = "nightly";
          sha256 = "sha256-gq7H6KCWVbf5rp6ceZVomtz/DOxM40i4TeWCIKxNAr8=";
        }).rust;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in with pkgs; rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          buildInputs = [ pkg-config glib gtk3 ];
          nativeBuildInputs = [ pkg-config wrapGAppsHook makeWrapper ];
        };

        overlays.default = final: prev: {
          openaws-vpn-client = self.outputs.defaultPackage.${prev.system};
        };

        openvpn-patched =
          import ./openvpn.nix { inherit (pkgs) fetchpatch openvpn; };

        devShell = mkShell {
          buildInputs = [ pkg-config glib gtk3 openvpn-patched ];
          nativeBuildInputs =
            [ pkg-config wrapGAppsHook makeWrapper toolchain ];
        };
      });
}
