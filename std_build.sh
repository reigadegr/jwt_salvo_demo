#!/bin/sh
rm -rf $(find ./target/release -name "*jwt_salvo_demo*")

export RUSTFLAGS="
  -C default-linker-libraries \
  -C link-args=-Wl,-O3,--gc-sections,--as-needed \
  -C link-args=-Wl,-z,now,-z,norelro,--strip-all,-x,-s \
  -C link-args=-static-libgcc \
  -C link-args=-static-libstdc++ \
  
  -C llvm-args=-enable-ml-inliner=release \
  -C llvm-args=-inliner-interactive-include-default \
  -C llvm-args=-ml-inliner-model-selector=arm64-mixed \
  -C llvm-args=-ml-inliner-skip-policy=if-caller-not-cold \

  -C llvm-args=-mergefunc-use-aliases \
  -C llvm-args=-enable-shrink-wrap=1 \
  -C llvm-args=-enable-gvn-hoist \
  -C llvm-args=-enable-loop-versioning-licm \
"

cargo build -r --verbose  --
