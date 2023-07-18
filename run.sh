#!/bin/sh
set -e
cd std-runner
RUSTFLAGS='-C panic=abort -C link-arg=-fuse-ld=mold -C target-cpu=native  -C embed-bitcode=yes -C lto=fat -C strip=symbols -C target-cpu=native -C target-feature=+crt-static -C relocation-model=pie' cargo b -r --target x86_64-unknown-linux-gnu
/usr/bin/time -f "Std threads completed in: [REAL]: %e, [USER]: %U, [KERNEL]: %S, [CPU]: %P" ./target/x86_64-unknown-linux-gnu/release/std-runner
cd ../tiny-std-runner
RUSTFLAGS='-C panic=abort -C link-arg=-nostartfiles -C target-cpu=native -C embed-bitcode=yes -C lto=fat -C strip=symbols -C link-arg=-fuse-ld=mold -C target-cpu=native -C target-feature=+crt-static -C relocation-model=pie' cargo b -r --target x86_64-unknown-linux-gnu
/usr/bin/time -f "Tiny std threads completed in: [REAL]: %e, [USER]: %U, [KERNEL]: %S, [CPU]: %P" ./target/x86_64-unknown-linux-gnu/release/tiny-std-runner
