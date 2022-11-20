ARCH ?= x86
TARGET ?= i686-unknown-linux-musl

maestro.iso: iso_build/iso/boot/grub/grub.cfg iso_build/iso/boot/maestro iso_build/iso/boot/initramfs
	grub-mkrescue -o $@ iso_build/iso

iso_build/iso/boot/grub/grub.cfg:
	mkdir -p iso_build/iso/boot/grub/
	cp grub.cfg $@

iso_build/iso/boot/maestro:
	mkdir -p iso_build/iso/boot/
	git clone https://github.com/llenotre/maestro iso_build/maestro/
	cp iso_build/maestro/default.config iso_build/maestro/.config
	sed -i 's/^GENERAL_ARCH=*$$/GENERAL_ARCH="$(ARCH)"/' iso_build/maestro/.config
	make -C iso_build/maestro/ maestro
	cp -v iso_build/maestro/maestro $@

iso_build/iso/boot/initramfs: iso_build/mnt/sbin/init iso_build/mnt/sbin/install
	mkdir -p iso_build/iso/boot/
	cd iso_build/mnt/; find . | cpio -o >../../$@; cd ../..

iso_build/mnt/sbin/init: iso_build/mnt/lib/modules/maestro-1.0/default/cmos.kmod iso_build/mnt/lib/modules/maestro-1.0/default/ps2.kmod
	SYSROOT='iso_build/sbin/' blimp update
	SYSROOT='iso_build/sbin/' blimp install solfege
	echo 'install' >iso_build/mnt/etc/hostname
	echo '/sbin/install' >iso_build/mnt/etc/solfege/startup

iso_build/mnt/lib/modules/maestro-1.0/default/cmos.kmod:
	git clone https://github.com/llenotre/maestro_cmos iso_build/maestro_cmos/
	KERN_SRC=../maestro make -C iso_build/maestro_cmos
	mkdir -p iso_build/mnt/lib/modules/maestro-1.0/default/
	cp iso_build/maestro_cmos/cmos.kmod $@

iso_build/mnt/lib/modules/maestro-1.0/default/ps2.kmod:
	git clone https://github.com/llenotre/maestro_ps2 iso_build/maestro_ps2/
	KERN_SRC=../maestro make -C iso_build/maestro_ps2
	mkdir -p iso_build/mnt/lib/modules/maestro-1.0/default/
	cp iso_build/maestro_ps2/ps2.kmod $@

iso_build/mnt/sbin/install:
	cargo build --release --target $(TARGET) -Zbuild-std
	mkdir -p iso_build/mnt/sbin/
	cp -v target/$(TARGET)/release/maestro_install $@

clean:
	rm -rf iso_build/
	rm -rf maestro.iso

.PHONY: clean
