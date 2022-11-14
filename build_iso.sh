#!/bin/sh

# Installer compilation
cargo build --release



# Preparing grub
mkdir -pv iso/boot/grub/
cp -v grub.cfg iso/boot/grub/
# TODO Install kernel



# Initramfs creation
mkdir -pv mnt/sbin
cp -v target/release/maestro_install mnt/sbin/init
find mnt/ | cpio -o >iso/boot/initramfs



# Grub setup
grub-mkrescue -o maestro.iso iso
