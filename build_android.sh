cargo fmt
rm -rf $(find ./target/aarch64-linux-android/release -name "*jwt_salvo_demo*")

export RUSTFLAGS="-C default-linker-libraries \
-Z external-clangrt \
-Z macro-backtrace \
-Z remap-cwd-prefix=. \
-Z dep-info-omit-d-target \
-C target-feature=xeon \
-C llvm-args=-enable-ml-inliner=release \
-C llvm-args=-inliner-interactive-include-default \
-C llvm-args=-ml-inliner-model-selector=arm64-mixed \
-C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \
-C link-args=-fomit-frame-pointer \
-C link-args=-Wl,--icf=all,-z,relro,--pack-dyn-relocs=android+relr,-x,-s,--strip-all,-z,now
" 

if [ "$1" = "release" ] || [ "$1" = "r" ]; then
    cargo +nightly ndk -p 35 -t arm64-v8a build --target aarch64-linux-android -Z trim-paths --verbose -r -Z build-std --
else
    cargo +nightly ndk -p 35 -t arm64-v8a build --target aarch64-linux-android -Z trim-paths --verbose  --
fi
