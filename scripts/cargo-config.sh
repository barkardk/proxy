#!/bin/bash
cat<<EOF > ~/.cargo/config
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
EOF