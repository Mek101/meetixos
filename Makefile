#! MeetiX OS Builder Script
#!
#! This Makefile is responsible to build all the source targets (Kernel and
#! applications), pack a bootable image then run the QEMU emulator for the selected
#! architecture

include config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

SRC_DIRS   ?= Kernel UKLibs Userland
DOC_DIR    ?= $(BUILD_PREFIX)/Doc
DOC_TARGET ?= $(shell pwd)/Userland/$(TARGET_PREFIX)/userland.json

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

just_run:
	$(V) echo "- Running QEMU $(VIRT_ACCEL)"
	$(V) $(QEMU) $(VIRT_ACCEL) $(QEMU_ARGS) -cdrom $(BUILD_PREFIX)/$(BUILD_MODE)/meetixos.iso

run_dbg: image
	$(V) echo "- Debugging"
	$(V) echo "target remote localhost:1234" >$(BUILD_PREFIX)/$(BUILD_MODE)/gdb_commands.txt
	$(V) echo "layout src" >>$(BUILD_PREFIX)/$(BUILD_MODE)/gdb_commands.txt
	$(V) echo "set print pretty on" >>$(BUILD_PREFIX)/$(BUILD_MODE)/gdb_commands.txt
	$(V) $(QEMU) $(VIRT_ACCEL) $(QEMU_GDB_ARGS) -S -s -cdrom $(BUILD_PREFIX)/$(BUILD_MODE)/meetixos.iso &
	$(V) $(R_GDB) $(BUILD_PREFIX)/kernel/$(BUILD_MODE)/mx_kernel --command=$(BUILD_PREFIX)/$(BUILD_MODE)/gdb_commands.txt

run: image
	$(V) echo "- Running QEMU $(VIRT_ACCEL)"
	$(V) $(QEMU) $(VIRT_ACCEL) $(QEMU_ARGS) -cdrom $(BUILD_PREFIX)/$(BUILD_MODE)/meetixos.iso

image: install
ifeq ($(ARCH),x86_64)
	$(V) echo "- Building GRUB bootable image... ($(BUILD_PREFIX)/$(BUILD_MODE)/meetixos.iso)"
	$(V) $(MAKE_RESCUE) -d /usr/lib/grub/i386-pc/                     \
	                    -o $(BUILD_PREFIX)/$(BUILD_MODE)/meetixos.iso \
	                    $(DIST_SYSROOT_PREFIX)
endif

install: build
	$(V) echo "- Copying sysroot to build '$(DIST_SYSROOT_PREFIX)' dir..."
	$(V) $(MKDIR) -p $(DIST_SYSROOT_PREFIX)
	$(V) $(RSYNC) -a Root/* $(DIST_SYSROOT_PREFIX)
	$(V) $(RSYNC) -a Boot/$(ARCH)/* $(DIST_SYSROOT_PREFIX)
	$(V) $(MAKE) $(MAKE_ARGS) -C Kernel install
	$(V) $(MAKE) $(MAKE_ARGS) -C Userland install

build: build_kernel build_userland
	$(V) echo "- MeetiX OS Successfully Built..."

build_userland:
	$(V) $(MAKE) $(MAKE_ARGS) -C Userland build

build_kernel:
	$(V) $(MAKE) $(MAKE_ARGS) -C Kernel build

doc: format_build_src
	$(V) echo "- Documenting Code..."
	$(V) cd $(DOC_DIR) &&                                 \
             RUSTFLAGS="$(RUSTC_FLAGS)"                   \
             CARGO_TARGET_DIR="$(DOC_DIR)/Target/$(ARCH)" \
                 $(CARGO) doc --open --target $(DOC_TARGET) --all-features

format_build_src: $(DOC_DIR)/rustfmt.toml
	$(V) echo "- Formatting build-doc sources..."
	$(V) cd $(DOC_DIR) &&                                 \
             RUSTFLAGS="$(RUSTC_FLAGS)"                   \
             CARGO_TARGET_DIR="$(DOC_DIR)/Target/$(ARCH)" \
                 $(CARGO) fmt

$(DOC_DIR)/rustfmt.toml: $(DOC_DIR)/Cargo.toml
	$(V) echo "normalize_comments = true" >$(DOC_DIR)/rustfmt.toml

$(DOC_DIR)/Cargo.toml: $(DOC_DIR)
	$(V) echo "- Syncing sources with build-doc sources..."
	$(V) for src_dir in $(SRC_DIRS); do $(RSYNC) -a $$src_dir $(DOC_DIR); done
	$(V) $(RSYNC) -a Cargo.toml $(DOC_DIR)

$(DOC_DIR):
	$(V) $(MKDIR) -p $(DOC_DIR)

clean:
	$(V) echo "- Cleaning $(ARCH)/$(BUILD_MODE)..."
	$(V) $(MAKE) $(MAKE_ARGS) -C Kernel clean
	$(V) $(MAKE) $(MAKE_ARGS) -C Userland clean

clean_all:
	$(V) echo "- Cleaning All..."
	$(V) $(RM) -rf $(BUILD_PREFIX)
