#!/bin/bash

# This scripts installs the maestro system on a mounted partition
# The path to the partition is specified by the environment variable SYSROOT

set -e

echo "Installing system on "$SYSROOT"..."

# Creates the directories hierarchy
mkdir -pv $SYSROOT/{bin,boot,dev,home,mnt,opt,proc,run,sbin,srv,sys,tmp}
mkdir -pv $SYSROOT/etc/{opt,sysconfig}
mkdir -pv $SYSROOT/lib/firmware
mkdir -pv $SYSROOT/media/{floppy,cdrom}
mkdir -pv $SYSROOT/usr/{,local/}{bin,include,lib,sbin,src}
mkdir -pv $SYSROOT/usr/{,local/}share/{color,dict,doc,info,locale,man}
mkdir -pv $SYSROOT/usr/{,local/}share/{misc,terminfo,zoneinfo}
mkdir -pv $SYSROOT/usr/{,local/}share/man/man{1..8}
mkdir -pv $SYSROOT/var/{cache,local,log,mail,opt,spool}
mkdir -pv $SYSROOT/var/lib/{color,misc,locate}
ln -sfv ../run $SYSROOT/var/run
ln -sfv ../run/lock $SYSROOT/var/lock

# Installing blimp's files
mkdir -pv $SYSROOT/usr/lib/blimp/{cache,database}
blimp remote-add localhost:8080
#blimp remote-add luc.lenot.re:8080
blimp update

# Installing mandatory packages
blimp install $(cat base_packages.txt)
