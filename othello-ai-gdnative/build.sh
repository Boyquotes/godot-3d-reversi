#!/bin/bash
cargo build --release --target=x86_64-unknown-linux-gnu
cargo build --release --target=x86_64-pc-windows-gnu
ANDROID_NDK_VERSION="21.3.6528147" ANDROID_SDK_ROOT="/home/lemtea/Android/Sdk/" cargo build --release --target=aarch64-linux-android
