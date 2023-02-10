#!/bin/bash

set -vex


if [[ "$OS_FAMILY" == "Linux" ]]; then
	sudo apt-get install -y libopencv-dev clang libclang-dev
    # sudo ln -fs libclang.so.1 /usr/lib/llvm-10/lib/libclang.so
    # sudo apt-get -y install "libopencv-dev=4.2.0*"
elif [[ "$OS_FAMILY" == "macOS" ]]; then
    brew -v update
    # fixes the install on 2023-01-14
    rm -f /usr/local/bin/2to3-3.11
    rm -f /usr/local/bin/idle3.11
    rm -f /usr/local/bin/pydoc3.11
    rm -f /usr/local/bin/python3.11
    rm -f /usr/local/bin/python3.11-config
    rm -f /usr/local/bin/2to3
    rm -f /usr/local/bin/idle3
    rm -f /usr/local/bin/pydoc3
    rm -f /usr/local/bin/python3
    rm -f /usr/local/bin/python3-config

    brew -v install opencv"4.2.0"
elif [[ "$OS_FAMILY" == "Windows" ]]; then
	export CHOCO_LLVM_VERSION=15.0.5
    choco install -y llvm --version "$CHOCO_LLVM_VERSION"
    choco install -y opencv --version "$OPENCV_VERSION"


    export PATH="/C/tools/opencv/build/x64/vc15/bin:$PATH"
    export OPENCV_LINK_PATHS="/C/tools/opencv/build/x64/vc15/lib"
    export OPENCV_LINK_LIBS="opencv_world${OPENCV_VERSION//./}"
    export OPENCV_INCLUDE_PATHS="/C/tools/opencv/build/include"
fi




if which cargo-tauri > /dev/null; then
    echo "Command exists"
else
    cargo install tauri-cli
fi

rm -rf target/release/bundle

# cargo tauri build --verbose
cargo build -r


zip -r "$OS_FAMILY.zip" target/release/* -x target/release/build -x target/release/bundle -x target/release/deps -x target/release/examples -x target/release/incremental 