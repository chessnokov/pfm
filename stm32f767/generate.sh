#!/bin/sh -aex
svd2rust -i STM32F767.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt