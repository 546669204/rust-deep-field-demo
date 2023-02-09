#!/bin/bash

set -vex

choco install -y llvm 
choco install -y opencv --version "$OPENCV_VERSION"


export PATH="/C/tools/opencv/build/x64/vc15/bin:$PATH"
export OPENCV_LINK_PATHS="/C/tools/opencv/build/x64/vc15/lib"
export OPENCV_LINK_LIBS="opencv_world${OPENCV_VERSION//./}"
export OPENCV_INCLUDE_PATHS="/C/tools/opencv/build/include"


cargo install tauri-cli

cargo tauri build