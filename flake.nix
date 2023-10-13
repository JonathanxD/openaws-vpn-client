{
  description = "openaws-vpn-client flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let system = "x86_64-linux"; in {

    packages.${system} = let
      pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
    in {
      default = self.packages.${system}.openaws-vpn-client;
      openvpn = import ./openvpn.nix { inherit (pkgs) fetchpatch openvpn; };
      openaws-vpn-client = import ./openaws-vpn-client.nix {
          inherit (self.packages.${system}) openvpn;
          inherit (pkgs) makeWrapper rust-bin makeRustPlatform fetchFromGitHub lib pkg-config glib gtk3 wrapGAppsHook;
         };
    };
  };
}
