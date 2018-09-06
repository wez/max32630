#!/usr/bin/env bash
set -x
set -e

cargo install --force --git https://github.com/wez/svd2rust --rev e0de96e90d6fd4c4d7207111fbe72bf2b531d593 svd2rust
cargo install --force --version 0.99.2 rustfmt-nightly
cargo install --force --version 0.3.0 form

cd max32630_svd
rm -rf src
mkdir src
svd2rust -i ../max32630.svd --nightly
form -i lib.rs -o src
rm lib.rs
cargo fmt
rustfmt build.rs
