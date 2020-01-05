#!/usr/bin/env bash
mkdir SOURCES
cargo build --release
cp target/release/rust-rpm-demo SOURCES/
cp rust-rpm-demo.service SOURCES
rpmbuild --define "_topdir $PWD" -ba rust-rpm-demo.spec

