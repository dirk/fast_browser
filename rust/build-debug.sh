#!/bin/sh
NATIVE_EXT=dylib
cargo build && cp target/debug/libfast_browser.$NATIVE_EXT ../ext/fast_browser/libfast_browser.$NATIVE_EXT
