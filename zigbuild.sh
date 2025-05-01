#!/bin/sh
export RUSTFLAGS="-C default-linker-libraries \
-Z external-clangrt \
-Z macro-backtrace \
-Z remap-cwd-prefix=. \
-Z dep-info-omit-d-target \
-C llvm-args=-enable-ml-inliner=release \
-C llvm-args=-inliner-interactive-include-default \
-C llvm-args=-ml-inliner-model-selector=arm64-mixed \
-C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \
-C link-args=-fomit-frame-pointer \
-C link-args=-Wl,-z,relro,-x,-s,--strip-all,-z,now
" 

if [ "$1" = "release" ] || [ "$1" = "r" ]; then
    cargo-zigbuild +nightly zigbuild --target aarch64-unknown-linux-musl -Z trim-paths --verbose -r -Z build-std --
else
    cargo-zigbuild +nightly zigbuild --target aarch64-unknown-linux-musl -Z trim-paths --verbose --
fi
