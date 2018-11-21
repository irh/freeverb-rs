#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen \
  ../target/wasm32-unknown-unknown/release/freeverb_wasm.wasm \
  --out-dir .

