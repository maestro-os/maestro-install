#!/bin/sh

set -e

if [ -z "$ARCH" ]; then
	ARCH="x86"
fi

# Kernel compilation
git clone https://github.com/llenotre/maestro
cp maestro/default.config maestro/.config
sed -i "s/^GENERAL_ARCH=*$/GENERAL_ARCH=\"$ARCH\"/" maestro/.config
make -C maestro/ maestro



# Installer compilation
cargo build --release --target maestro/arch/$ARCH/target.json -Zbuild-std



# Preparing grub
mkdir -pv iso/boot/grub/
cp -v grub.cfg iso/boot/grub/
cp -v maestro/maestro iso/boot/



# Initramfs creation
mkdir -pv mnt/sbin
cp -v target/release/maestro_install mnt/sbin/init
find mnt/ | cpio -o >iso/boot/initramfs



# Grub setup
grub-mkrescue -o maestro.iso iso
