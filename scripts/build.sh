#!/bin/bash

# Backend lambda
(cd generator && cargo build --release --target=x86_64-unknown-linux-musl)
cp generator/target/x86_64-unknown-linux-musl/release/generator generator/bootstrap
zip generator/generator.zip generator/bootstrap
rm -f generator/bootstrap

# cdk
