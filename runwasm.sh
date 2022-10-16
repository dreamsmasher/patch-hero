#!/bin/bash

set -e

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/patch-hero.wasm site/wasm/patch-hero.wasm
killall -q basic-http-server
basic-http-server site/ &