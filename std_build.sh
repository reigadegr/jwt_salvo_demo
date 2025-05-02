cargo fmt
rm -rf $(find ./target/aarch64-linux-android/release -name "*jwt_salvo_demo*")

export RUSTFLAGS="-C default-linker-libraries \
-C llvm-args=-enable-ml-inliner=release \
-C llvm-args=-inliner-interactive-include-default \
-C llvm-args=-ml-inliner-model-selector=arm64-mixed \
-C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \
-C link-args=-fomit-frame-pointer \
-C link-args=-Wl,-z,relro,-x,-s,--strip-all,-z,now
" 

cargo build -r --verbose  --
