ARCH ?= x86
TARGET ?= i686-unknown-linux-musl

INITRAMFS_ROOT = iso_build/mnt
MODULES_PATH = $(INITRAMFS_ROOT)/lib/modules/maestro-1.0/default

GRUB_ROOT = iso_build/iso

maestro.iso: iso_build/iso/boot/grub/grub.cfg iso_build/iso/boot/maestro iso_build/iso/boot/initramfs
	grub-mkrescue -o $@ iso_build/iso

$(GRUB_ROOT)/boot/grub/grub.cfg:
	mkdir -p iso_build/iso/boot/grub/
	cp grub.cfg $@

$(GRUB_ROOT)/boot/maestro:
	mkdir -p iso_build/iso/boot/
	yes | SYSROOT='$(INITRAMFS_ROOT)' blimp install maestro

$(GRUB_ROOT)/boot/initramfs: $(INITRAMFS_ROOT)/sbin/init $(INITRAMFS_ROOT)/sbin/install
	mkdir -p iso_build/iso/{boot,dev,proc,tmp}
	cd $(INITRAMFS_ROOT); find . | cpio -o >../../$@; cd ../..

$(INITRAMFS_ROOT)/sbin/init: $(MODULES_PATH)/cmos.kmod $(MODULES_PATH)/ps2.kmod
	yes | SYSROOT='$(INITRAMFS_ROOT)' blimp install solfege
	echo 'install' >$(INITRAMFS_ROOT)/etc/hostname
	echo '/sbin/install' >$(INITRAMFS_ROOT)/etc/solfege/startup
	mkdir $(INITRAMFS_ROOT)/{proc,tmp}

$(MODULES_PATH)/cmos.kmod:
	yes | SYSROOT='$(INITRAMFS_ROOT)' blimp install maestro-cmos

$(MODULES_PATH)/ps2.kmod:
	yes | SYSROOT='$(INITRAMFS_ROOT)' blimp install maestro-ps2

$(INITRAMFS_ROOT)/sbin/install:
	cargo build --release --target $(TARGET) -Zbuild-std
	mkdir -p $(INITRAMFS_ROOT)/sbin/
	cp -v target/$(TARGET)/release/maestro_install $@
	mkdir -v $(INITRAMFS_ROOT)/lang/
	cp -v lang/* $(INITRAMFS_ROOT)/lang/

clean:
	rm -rf iso_build/
	rm -rf maestro.iso

.PHONY: clean
