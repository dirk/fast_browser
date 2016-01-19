#!/bin/sh

# Call this like `TARGET=release ./build-debug.sh` to do a release build

# TODO: Make this set the right extension based on the current platform
NATIVE_EXT=dylib

if [ "$TARGET" == "release" ]; then
  CARGO_ARGS=--release
  TARGET=release
else
  CARGO_ARGS=
  TARGET=debug
fi

set -x
cargo build $CARGO_ARGS && cp target/$TARGET/libfast_browser.$NATIVE_EXT ../ext/fast_browser/libfast_browser.$NATIVE_EXT
