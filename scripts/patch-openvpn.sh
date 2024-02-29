#!/usr/bin/env bash
# Files downloaded by this script are under the LICENSE of respective repositories and distributors.
# We are not responsible by any of these files

# All credits for the OpenVPN binaries goes for OpenVPN community:
# - https://openvpn.net/
# - https://github.com/OpenVPN/

# All credits for the patch files goes for 'samm-git'
# - https://smallhacks.wordpress.com/
# - https://github.com/samm-git/aws-vpn-client

OPENVPN_VERSION="2.5.9"
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
echo "48d04e08ba62aa098d9e3bc246cf521c6e8b200bd817488a05989ae6c42d8fd144ddf03de43eb2f3c4778a643217db4220288c2d40f324076771a20b95d5028b  openvpn-$OPENVPN_VERSION.tar.gz" | sha512sum -c -
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
