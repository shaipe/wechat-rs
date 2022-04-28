#!/bin/bash
echo "start cross compiling app to linux ..."
cargo b --release --target x86_64-unknown-linux-musl
