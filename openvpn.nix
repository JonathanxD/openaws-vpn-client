{ openvpn, fetchpatch }:

openvpn.overrideAttrs (oldAttrs: rec {
  patches = [
    (fetchpatch {
      url =
        "https://raw.githubusercontent.com/samm-git/aws-vpn-client/master/openvpn-v2.5.1-aws.patch";
      hash = "sha256-9ijhANqqWXVPa00RBCRACtMIsjiBqYVa91V62L4mNas=";
    })
  ];
})
