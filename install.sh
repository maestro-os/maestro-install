#!/bin/bash

# Creates the directories hierarchy
mkdir -pv {boot,home,mnt,opt,srv}
mkdir -pv etc/{opt,sysconfig}
mkdir -pv lib/firmware
mkdir -pv media/{floppy,cdrom}
mkdir -pv usr/{,local/}{bin,include,lib,sbin,src}
mkdir -pv usr/{,local/}share/{color,dict,doc,info,locale,man}
mkdir -pv usr/{,local/}share/{misc,terminfo,zoneinfo}
mkdir -pv usr/{,local/}share/man/man{1..8}
mkdir -pv var/{cache,local,log,mail,opt,spool}
mkdir -pv var/lib/{color,misc,locate}
ln -sfv ../run var/run
ln -sfv ../run/lock var/lock

# Installing mandatory packages
# TODO Call on blimp to install a list of packages

# Installing Solfège
# TODO

# Installing blimp
# TODO
