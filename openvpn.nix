{ openvpn, fetchpatch }:

openvpn.overrideAttrs (oldAttrs: {
  patches = oldAttrs.patches or [] ++ [
    (fetchpatch {
      url =
        "https://raw.githubusercontent.com/scrive/aws-vpn-client/openvpn-v2.6.12/openvpn-v2.6.12-aws.patch";
      hash = "sha256-CxiWV7z7xvPrW8qcXd2YEZcvn1/+aA7ZuN8HLCcO8n0=";
    })
  ];
})
