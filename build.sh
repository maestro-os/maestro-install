#!/bin/bash

set -e

# If not specified, set build target to default
if [ -z $TARGET ]; then
  TARGET=i686-unknown-linux-musl
fi

GRUB_ROOT=iso_build/iso
INITRAMFS_ROOT=iso_build/mnt

# Setup grub config
mkdir -pv $GRUB_ROOT/boot/grub
cp -v grub.cfg $GRUB_ROOT/boot/grub/

# Create directories hierarchy
mkdir -pv $INITRAMFS_ROOT/{dev,etc,lang,proc,sbin,tmp,usr/lib/blimp}

# Compile and install installer
cargo build --release --target $TARGET -Zbuild-std
cp -v target/$TARGET/release/maestro_install $INITRAMFS_ROOT/sbin/install
cp -v lang/* $INITRAMFS_ROOT/lang/

# Copy packages required to be installed on the system
if [ ! -z "$LOCAL_REPO" ]; then
	mkdir -pv "$INITRAMFS_ROOT/local_repo"
	for name in $(cat base_packages.txt); do
		cp -rv "$LOCAL_REPO/$name" "$INITRAMFS_ROOT/local_repo"
	done
fi

# Install packages required by the installer
yes | SYSROOT="$INITRAMFS_ROOT" blimp install grub maestro maestro-ps2 maestro-utils solfege

# Move kernel to GRUB
mv -v $INITRAMFS_ROOT/boot/maestro $GRUB_ROOT/boot/

# Solfege setup
echo 'install' >$INITRAMFS_ROOT/etc/hostname
echo '/sbin/install' >$INITRAMFS_ROOT/etc/solfege/startup

# Create ISO file
cd $INITRAMFS_ROOT; find . | cpio -o >../../$GRUB_ROOT/boot/initramfs; cd ../..
grub-mkrescue -o maestro.iso iso_build/iso/
