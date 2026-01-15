#!/bin/sh

export RUSTFLAGS="
    -Z mir-opt-level=2
    -Z dylib-lto=yes
    -Z inline-mir=yes
    -Z share-generics=yes
    -Z remap-cwd-prefix=.
    -Z function-sections=yes
    -Z dep-info-omit-d-target
    -Z flatten-format-args=yes
    -Z saturating-float-casts=yes
    -Z mir-enable-passes=+Inline
    -Z precise-enum-drop-elaboration=yes
    -C default-linker-libraries
    -C relro-level=full
    -C code-model=small
    -C relocation-model=static
    -C symbol-mangling-version=v0
    -C llvm-args=-fp-contract=off
    -C llvm-args=-enable-misched
    -C llvm-args=-enable-post-misched
    -C llvm-args=-enable-dfa-jump-thread
    -C link-arg=-Wl,--sort-section=alignment
    -C link-args=-Wl,--gc-sections,--as-needed
    -C link-args=-Wl,-x,-z,noexecstack,-s,--strip-all
" 

if [ "$1" = "release" ] || [ "$1" = "r" ]; then
    export CFLAGS="-Wno-error=date-time"
    cargo +nightly zigbuild --target aarch64-unknown-linux-musl -Z trim-paths --verbose -r -Z build-std=core,alloc,std,panic_abort --
else
    cargo +nightly zigbuild --target aarch64-unknown-linux-musl -Z trim-paths --verbose --
fi
