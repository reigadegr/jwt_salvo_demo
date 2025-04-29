cargo fmt
rm -rf $(find ./target/aarch64-linux-android/release -name "*jwt_salvo_demo*")

export RUSTFLAGS="-C default-linker-libraries \
-Z external-clangrt \
-Z macro-backtrace \
-Z remap-cwd-prefix=. \
-Z dep-info-omit-d-target \
-C link-args=-fomit-frame-pointer \
-C link-args=-Wl,-z,relro,-x,-s,--strip-all,-z,now
" 

cargo +nightly run -r -Z build-std -Z trim-paths --verbose  --
