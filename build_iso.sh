#!/bin/sh

set -e

if [ -z "$ARCH" ]; then
	ARCH="x86"
fi

# TODO Set target according to ARCH
TARGET=i686-unknown-linux-musl

# Kernel compilation
git clone https://github.com/llenotre/maestro
cp maestro/default.config maestro/.config
sed -i "s/^GENERAL_ARCH=*$/GENERAL_ARCH=\"$ARCH\"/" maestro/.config
make -C maestro/ maestro



# Installer compilation
cargo build --release --target $TARGET -Zbuild-std



# Preparing grub
mkdir -pv iso/boot/grub/
cp -v grub.cfg iso/boot/grub/
cp -v maestro/maestro iso/boot/



# Initramfs preparation
mkdir -pv mnt/sbin
cp -v target/$TARGET/release/maestro_install mnt/sbin/init
# TODO Add sfdisk
# TODO Fill local packages repository, if building without network access

cd mnt
find . | cpio -o >../iso/boot/initramfs
cd ..



# Grub setup
grub-mkrescue -o maestro.iso iso
