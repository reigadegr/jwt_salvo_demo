rm -rf $(find ./target -name "*jwt_salvo_demo*")
# cargo +nightly ndk -p 35 -t arm64-v8a run --target aarch64-linux-android
cargo run
