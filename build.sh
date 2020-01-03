$!/usr/bin/env bash
mkdir SOURCES
cargo build --release
cp target/release/demo SOURCES
cd SPECS
rpmbuild -ba rust-rpm-demo.spec
