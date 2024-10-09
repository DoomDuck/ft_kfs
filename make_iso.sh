#!/usr/bin/env bash

set -o noclobber  # Avoid overlay files (echo "hi" > foo)
set -o errexit    # Used to exit upon error, avoiding cascading errors
set -o pipefail   # Unveils hidden failures
set -o nounset    # Exposes unset variables

cargo build --target ./conf/i686-elf/i686-elf.json

export ISO_DIR=isofs/boot/
mkdir -p $ISO_DIR/grub

# Populate 
cp grub.cfg $ISO_DIR/grub/
ln ./target/i686-elf/debug/kfs $ISO_DIR

grub-mkrescue isofs -o kfs.iso

rm -r $ISO_DIR

