{
  description = "openaws-vpn-client flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let system = "x86_64-linux"; in {

    packages.${system} = {
      default = self.packages.x86_64-linux.openaws-vpn-client;
      openaws-vpn-client = let
      pkgs = nixpkgs.legacyPackages.${system}.extend rust-overlay.overlays.default;
        in import ./openaws-vpn-client.nix {
          inherit (pkgs) rust-bin makeRustPlatform fetchFromGitHub lib pkg-config glib gtk3 wrapGAppsHook;
         };
    };
  };
}
