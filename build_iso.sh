#!/bin/sh

cargo build --release

mkdir -pv mnt/sbin
cp target/release/maestro_install mnt/sbin/init

find mnt/ | cpio -o >initramfs
