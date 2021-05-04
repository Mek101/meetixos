#! MeetiX OS Builder Script
#!
#! This Makefile is responsible to build all the source targets (kernel and
#! applications), pack a bootable image then run the QEMU emulator for the selected
#! architecture

include config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

SRC_DIRS ?= kernel libs userland
DOC_DIR  ?= $(BUILD_PREFIX)/doc
TARGET   ?= $(shell pwd)/userland/$(TARGET_PREFIX)/userland.json

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

run: image
	$(V) echo "- Running QEMU $(VIRT_ACCEL)"
	$(V) $(QEMU) $(VIRT_ACCEL) -m 64M -serial stdio -cdrom $(BUILD_DIR)/$(BUILD_MODE)/meetixos.iso

image: install
ifeq ($(ARCH),x86_64)
	$(V) echo "- Building GRUB bootable image..."
	$(V) $(MAKE_RESQUE) -d /usr/lib/grub/i386-pc/                     \
	                    -o $(BUILD_PREFIX)/$(BUILD_MODE)/meetixos.iso \
	                    $(BUILD_DIR)/sysroot/$(BUILD_MODE)
endif

install: build
	$(V) echo "- Copying sysroot to build dir..."
	$(V) $(MKDIR) -p $(DIST_SYSROOT_PREFIX)
	$(V) $(RSYNC) -a sysroot/* $(DIST_SYSROOT_PREFIX)
	$(V) $(RSYNC) -a boot/$(ARCH)/* $(DIST_SYSROOT_PREFIX)

	$(V) echo "- Installing Kernel..."
	$(V) cd kernel && $(MAKE) install

	$(V) echo "- Installing Userland..."
	$(V) cd userland && $(MAKE) install

build: build_kernel build_userland
	$(V) echo "- MeetiX OS Successfully Built..."

build_userland:
	$(V) echo "- Building $(ARCH)/$(BUILD_MODE) Userland..."
	$(V) cd userland && $(MAKE) build

build_kernel:
	$(V) echo "- Building $(ARCH)/$(BUILD_MODE) Kernel..."
	$(V) cd kernel && $(MAKE) build

doc: format_build_src
	$(V) echo "- Documenting Code..."
	$(V) cd $(DOC_DIR) &&                                 \
             RUSTFLAGS="$(RUSTC_FLAGS)"                   \
             CARGO_TARGET_DIR="$(DOC_DIR)/target/$(ARCH)" \
                 $(CARGO) doc --open --target $(TARGET) --all-features

format_build_src: $(DOC_DIR)/rustfmt.toml
	$(V) echo "- Formatting build-doc sources..."
	$(V) cd $(DOC_DIR) &&                                 \
             RUSTFLAGS="$(RUSTC_FLAGS)"                   \
             CARGO_TARGET_DIR="$(DOC_DIR)/target/$(ARCH)" \
                 $(CARGO) fmt

$(DOC_DIR)/rustfmt.toml: $(DOC_DIR)/Cargo.toml
	$(V) echo "normalize_comments = true" >$(DOC_DIR)/rustfmt.toml

$(DOC_DIR)/Cargo.toml: $(DOC_DIR)
	$(V) echo "- Syncing sources with build-doc sources..."
	$(V) for src_dir in $(SRC_DIRS); do $(RSYNC) -a $${src_dir} $(DOC_DIR); done
	$(V) $(RSYNC) -a Cargo.toml $(DOC_DIR)

$(DOC_DIR):
	$(V) $(MKDIR) -p $(DOC_DIR)

clean:
	$(V) echo "- Cleaning $(ARCH)/$(BUILD_MODE)..."
	$(V) cd kernel && $(MAKE) clean
	$(V) cd userland && $(MAKE) clean

clean_all:
	$(V) echo "- Cleaning All..."
	$(V) $(RM) -rf $(BUILD_PREFIX)
