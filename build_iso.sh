#!/bin/sh

set -e

if [ -z "$ARCH" ]; then
	ARCH="x86"
fi

# TODO Set target according to ARCH
TARGET=i686-unknown-linux-musl

mkdir -pv iso_build
cd iso_build

# Kernel compilation
git clone https://github.com/llenotre/maestro
cp maestro/default.config maestro/.config
sed -i "s/^GENERAL_ARCH=*$/GENERAL_ARCH=\"$ARCH\"/" maestro/.config
make -C maestro/ maestro



# Kernel modules compilation
git clone https://github.com/llenotre/maestro_cmos
cd maestro_cmos
KERN_SRC=../maestro make
cd ..

git clone https://github.com/llenotre/maestro_ps2
cd maestro_ps2
KERN_SRC=../maestro make
cd ..



# Solfege compilation
git clone https://github.com/llenotre/solfege
cd solfege
cargo build --release --target $TARGET -Zbuild-std
cd ..



# Installer compilation
cargo build --release --target $TARGET -Zbuild-std



# Preparing grub
mkdir -pv iso/boot/grub/
cp -v ../grub.cfg iso/boot/grub/
cp -v maestro/maestro iso/boot/



# Preparing initramfs
mkdir -pv mnt/{etc/solfege,proc,sbin,tmp}

cat >mnt/etc/fstab <<EOF
tmpfs			/tmp	tmpfs	rw		0		0
procfs			/proc	procfs	rw		0		1
EOF
ln -sv /proc/self/mounts mnt/etc/mtab
echo 'install' >mnt/etc/hostname
echo '/sbin/install' >mnt/etc/solfege/startup
cp -v solfege/target/$TARGET/release/solfege mnt/sbin/init

mkdir -pv mnt/lib/modules/maestro-1.0/default
cp -v maestro_cmos/cmos.kmod mnt/lib/modules/maestro-1.0/default/
cp -v maestro_ps2/ps2.kmod mnt/lib/modules/maestro-1.0/default/

cp -v ../target/$TARGET/release/maestro_install mnt/sbin/install

# TODO Add sfdisk

# TODO Fill local packages repository, if building without network access

cd mnt
find . | cpio -o >../iso/boot/initramfs
cd ..



# Grub setup
grub-mkrescue -o ../maestro.iso iso
