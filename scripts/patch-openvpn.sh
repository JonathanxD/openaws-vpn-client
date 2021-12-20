#!/usr/bin/env bash
# Files downloaded by this script are under the LICENSE of respective repositories and distributors.
# We are not responsible by any of these files

# All credits for the OpenVPN binaries goes for OpenVPN community:
# - https://openvpn.net/
# - https://github.com/OpenVPN/

# All credits for the patch files goes for 'samm-git'
# - https://smallhacks.wordpress.com/
# - https://github.com/samm-git/aws-vpn-client

ROOT_DIR="$(pwd)"
mkdir -p "$ROOT_DIR/../share/openvpn"

mkdir tmp
cd tmp

# Download OpenVPN
echo "Downloading OpenVPN..."
curl https://raw.githubusercontent.com/OpenVPN/openvpn/master/COPYING --output "$ROOT_DIR/../share/openvpn/COPYING"
curl https://raw.githubusercontent.com/OpenVPN/openvpn/master/COPYRIGHT.GPL --output "$ROOT_DIR/../share/openvpn/COPYRIGHT.GPL"
curl https://swupdate.openvpn.org/community/releases/openvpn-2.5.5.tar.gz --output openvpn-2.5.5.tar.gz
echo "a43ad7b1a92fc4fef7d0e64c9ecaad0a40aa76e55bf70385db4ebeee687b9f2ed1ef1e0294bdf73f63a57ab3ba72eb6c4e301a3aad203e2c167b408c5e6c432e  openvpn-2.5.5.tar.gz" | sha512sum -c -
echo "Decompressing OpenVPN..."
tar -xf openvpn-2.5.5.tar.gz
rm -rf openvpn-2.5.5.tar.gz
cd openvpn-2.5.5

# Apply OpenVPN patch by 'samm-git'
echo "Downloading OpenVPN Patch by 'samm-git'..."
curl https://raw.githubusercontent.com/samm-git/aws-vpn-client/master/LICENSE --output "$ROOT_DIR/../share/openvpn/PATCH-LICENSE"
curl https://raw.githubusercontent.com/samm-git/aws-vpn-client/master/openvpn-v2.5.1-aws.patch --output openvpn-v2.5.1-aws.patch
echo "61f9e670d5081b7628955c8eee90d6b04deb02b0e8f3494bc236f502b919a6bbb79ddd9775274fb795e99f90e8c134c7daece9b1be60ba52b4fa968c27369e8d  openvpn-v2.5.1-aws.patch" | sha512sum -c -
echo "Applying OpenVPN Patch by 'samm-git'..."
patch -p1 <openvpn-v2.5.1-aws.patch

# Configure and build OpenVPN
echo "Building OpenVPN..."
./configure
make

echo "Copying OpenVPN..."
mkdir -p "$ROOT_DIR/../share/openvpn/bin"
cp src/openvpn/openvpn "$ROOT_DIR/../share/openvpn/bin/openvpn"

echo "Custom OpenVPN binary created."
cd "$ROOT_DIR"
rm -rf tmp