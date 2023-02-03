#!/bin/bash

set -e

export CC=clang

TARGET=i686-unknown-linux-musl

INITRAMFS_ROOT=iso_build/mnt
MODULES_PATH="$INITRAMFS_ROOT/lib/modules/maestro-1.0/default"

GRUB_ROOT=iso_build/iso

# Setup grub config
mkdir -pv $GRUB_ROOT/boot/grub
cp -v grub.cfg $GRUB_ROOT/boot/grub/

# Create directories hierarchy
mkdir -pv $INITRAMFS_ROOT/{dev,etc,lang,proc,sbin,tmp}

# Compile and install installer
cargo build --release --target $TARGET -Zbuild-std
cp -v target/$TARGET/release/maestro_install $INITRAMFS_ROOT/sbin/install
cp -v lang/* $INITRAMFS_ROOT/lang/

# Install required packages
yes | SYSROOT="$INITRAMFS_ROOT" blimp install maestro solfege maestro-cmos maestro-ps2

# Move kernel to GRUB
mv -v $INITRAMFS_ROOT/boot/maestro $GRUB_ROOT/boot/

# Solfege setup
echo 'install' >$INITRAMFS_ROOT/etc/hostname
echo '/sbin/install' >$INITRAMFS_ROOT/etc/solfege/startup

# Create ISO file
cd $INITRAMFS_ROOT; find . | cpio -o >../../$GRUB_ROOT/boot/initramfs; cd ../..
grub-mkrescue -o maestro.iso iso_build/iso/
