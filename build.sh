#!/bin/bash
set -e
mkdir -p bin
cargo build --release
if [ -f target/release/wmnkextract.exe ]; then
    cp target/release/wmnkextract.exe bin/wmnkextract.exe
    cp target/release/wmpartinfo.exe bin/wmpartinfo.exe
else
    cp target/release/wmnkextract bin/wmnkextract
    cp target/release/wmpartinfo bin/wmpartinfo
fi
echo "Build successful! Binaries placed in bin/"
