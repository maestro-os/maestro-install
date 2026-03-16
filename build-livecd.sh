#!/bin/bash

set -e

# If not specified, set build target to default
TARGET=${TARGET:-x86_64-unknown-linux-musl}

GRUB_ROOT=livecd-build/iso
INITRAMFS_ROOT=livecd-build/mnt

# Setup grub config
mkdir -pv $GRUB_ROOT/boot/grub
cp -v grub-livecd.cfg $GRUB_ROOT/boot/grub/

# Create directories hierarchy
mkdir -pv $INITRAMFS_ROOT/{bin,boot,dev,etc,home,lib,media,mnt,opt,proc,root,run,sbin,srv,sys,tmp,usr,var}
mkdir -pv $INITRAMFS_ROOT/etc/{opt,sysconfig}
mkdir -pv $INITRAMFS_ROOT/lib/firmware
mkdir -pv $INITRAMFS_ROOT/run/{lock,log}
mkdir -pv $INITRAMFS_ROOT/usr/{bin,include,lib,local,sbin,share,src}
mkdir -pv $INITRAMFS_ROOT/usr/share/{doc,info,locale,man,misc}
mkdir -pv $INITRAMFS_ROOT/usr/lib/blimp
mkdir -pv $INITRAMFS_ROOT/usr/local/{bin,include,lib,sbin,share,src}
mkdir -pv $INITRAMFS_ROOT/usr/local/share/{doc,info,locale,man,misc}
mkdir -pv $INITRAMFS_ROOT/var/{cache,lib,local,log,mail,opt,spool}
mkdir -pv $INITRAMFS_ROOT/var/lib/misc

# Install packages
yes | SYSROOT="$INITRAMFS_ROOT" blimp install bash coreutils maestro maestro-ps2 maestro-utils solfege

# Move kernel to GRUB
mv -v $INITRAMFS_ROOT/boot/maestro $GRUB_ROOT/boot/

# Solfege setup
echo 'Maestro' >$INITRAMFS_ROOT/etc/hostname
echo '/bin/bash' >$INITRAMFS_ROOT/etc/solfege/startup

# Create ISO file
cd $INITRAMFS_ROOT; find . | cpio -o >../../$GRUB_ROOT/boot/initramfs; cd ../..
grub-mkrescue -o maestro.iso $GRUB_ROOT
