#!/usr/bin/env bash
# Files downloaded by this script are under the LICENSE of respective repositories and distributors.
# We are not responsible by any of these files

# All credits for the OpenVPN binaries goes for OpenVPN community:
# - https://openvpn.net/
# - https://github.com/OpenVPN/

# All credits for the patch files goes for 'samm-git'
# - https://smallhacks.wordpress.com/
# - https://github.com/samm-git/aws-vpn-client

set -e

OPENVPN_VERSION="2.5.11"
CURRENT_DIRNAME=${PWD##*/}

if [ "$CURRENT_DIRNAME" != "scripts" ]; then
    if ! [[ -d "scripts" ]]
    then
        echo "Could not find 'scripts' directory. Please run this script from the root directory of the repository."
        exit 255
    fi

    cd "scripts"
fi

ROOT_DIR="$(pwd)"
mkdir -p "$ROOT_DIR/../share/openvpn"

mkdir tmp
cd tmp

# Download OpenVPN
echo "Downloading OpenVPN..."
curl https://raw.githubusercontent.com/OpenVPN/openvpn/master/COPYING --output "$ROOT_DIR/../share/openvpn/COPYING"
curl https://raw.githubusercontent.com/OpenVPN/openvpn/master/COPYRIGHT.GPL --output "$ROOT_DIR/../share/openvpn/COPYRIGHT.GPL"
curl https://swupdate.openvpn.org/community/releases/openvpn-$OPENVPN_VERSION.tar.gz --output openvpn-$OPENVPN_VERSION.tar.gz
echo "5ef80681e71aa84629d48b067b540c0e8169ee3ff4b1129fc0030a55f0f7e2bb9a9cd568aa627828d8adb1366f5b0cfdd37242fb5cb6cec4a50fea9ffe8805bc  openvpn-$OPENVPN_VERSION.tar.gz" | sha512sum -c -
echo "Decompressing OpenVPN..."
tar -xf openvpn-$OPENVPN_VERSION.tar.gz
rm -rf openvpn-$OPENVPN_VERSION.tar.gz
cd openvpn-$OPENVPN_VERSION || exit 1

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
