#!/bin/bash
#
# build.sh
# Sean Walker
# CPSC 490
# Build script for dynamically-generated NF service chains.
# Adapted from https://github.com/williamofockham/NetBricks/blob/master/build.sh
#

set -e
BASE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && cd .. && pwd)"
SRC_DIR=${BASE_DIR}/cpsc-490
CARGO_LOC=`which cargo || true`
export CARGO=${CARGO_PATH-"${CARGO_LOC}"}
NATIVE_LIB_PATH="${BASE_DIR}/native"
DPDK_VER=17.08
DPDK_HOME="/opt/dpdk/dpdk-stable-${DPDK_VER}"
DPDK_LD_PATH="${DPDK_HOME}/build/lib"

find_sctp () {
    set +o errexit
    gcc -lsctp 2>&1 | grep "cannot find" >/dev/null
    export SCTP_PRESENT=$?
    set -o errexit
    if [ ${SCTP_PRESENT} -eq 1 ]; then
        echo "SCTP library found"
    else
        echo "No SCTP library found, install libsctp ('sudo apt-get install libsctp-dev' on debian)"
    fi
}

native () {
    make -j $proc -C $BASE_DIR/native
    make -C $BASE_DIR/native install
}

# ensure proper usage
if [ $# -ne 1 ]; then
    echo "usage: build.sh <service chain to run>"
    echo "e.g. build.sh mme-nat for a service chain consiting of MME -> NAT"
    exit 1
fi

# ensure SCTP library is present, native code is built
find_sctp
native

# then build NetBricks framework
pushd $BASE_DIR/framework
${CARGO} build
popd

# build specified service chain
pushd ${SRC_DIR}/${1}
${CARGO} build

# run service chain
executable=${BASE_DIR}/target/debug/$1
export PATH="${BIN_DIR}:${PATH}"
export LD_LIBRARY_PATH="${NATIVE_LIB_PATH}:${DPDK_LD_PATH}:${LD_LIBRARY_PATH}"
env PATH="$PATH" LD_LIBRARY_PATH="$LD_LIBRARY_PATH" LD_PRELOAD="$LD_PRELOAD" \
$executable "$@"

done