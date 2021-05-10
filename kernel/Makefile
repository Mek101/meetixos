#! Kernel Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the kernel

include ../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT    ?= $(realpath ..)
BUILD_DIR    ?= $(REPO_ROOT)/$(BUILD_PREFIX)
BUILD_TARGET ?= $(realpath $(TARGET_PREFIX)/kernel.json)
DIST_DIR     ?= ../$(DIST_SYSROOT_PREFIX)/MeetiX

#
# -- -- -- -- -- -- -- -- -- -- -- -- Kernel Paths -- -- -- -- -- -- -- -- -- -- -- --
#

KRN_BUILT_DIR ?= $(BUILD_DIR)/kernel/$(BUILD_MODE)
HHL_BUILT_DIR ?= $(BUILD_DIR)/hh_loader/$(BUILD_MODE)

BUILT_KRN    ?= $(KRN_BUILT_DIR)/kernel
BUILT_HHL    ?= $(HHL_BUILT_DIR)/hh_loader
HHL_CORE_MOD ?= $(KRN_BUILT_DIR)/kernel_bin.rs

MX_KERNEL ?= $(HHL_BUILT_DIR)/mx_kernel

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- --
#

install: build
	$(V) $(RSYNC) -a $(MX_KERNEL) $(DIST_DIR)

build: $(MX_KERNEL)
	$(V) echo "- Kernel Build completed..."

$(MX_KERNEL): $(BUILT_HHL)
	$(V) echo "- Copy ELF64 kernel into ELF32 executable..."
	$(V) $(LLVM_OBJCOPY) $(OBJCOPY_FLAGS) $(BUILT_HHL)

	$(V) echo "- Linking '$(basename $(MX_KERNEL))'"
	$(V) $(CP) -f $(BUILT_HHL) $(MX_KERNEL)

$(BUILT_HHL): $(HHL_CORE_MOD)
	$(V) echo "- Building Kernel Loader..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"                        \
         KERNEL_BIN=$(HHL_CORE_MOD)                  \
         CARGO_TARGET_DIR="$(BUILD_DIR)"                   \
             $(CARGO) build $(CARGO_FLAGS)                 \
                      --manifest-path hh_loader/Cargo.toml \
                      --target hh_loader/$(TARGET_PREFIX)/hh_loader.json

$(HHL_CORE_MOD): #$(BUILT_KRN)
	$(V) echo "- Writing Kernel Core Module..."
	$(V) echo "const KERNEL_SIZE: usize = 16;" >$(HHL_CORE_MOD)
	$(V) echo "const KERNEL_BYTES: [u8; KERNEL_SIZE] = [0; KERNEL_SIZE];" >>$(HHL_CORE_MOD)
	$(V) #echo "const KERNEL_SIZE: usize = $$(stat --format %s $(BUILT_KERN_CORE));" >$(HHL_CORE_MOD)
	$(V) #echo "const KERNEL_BYTES: [u8; KERNEL_SIZE] = *include_bytes!(r\"$(BUILT_KRN)\");" >>$(HHL_CORE_MOD)

$(BUILT_KRN):
	$(V) echo "- Building Kernel Core..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"      \
         CARGO_TARGET_DIR="$(BUILD_DIR)" \
             $(CARGO) build $(CARGO_FLAGS) --target $(BUILD_TARGET)

clean:
	$(V) echo "- Cleaning $(ARCH)/$(BUILD_MODE) Kernel..."
	$(V) $(RM) -rf $(KRN_BUILT_DIR) $(HHL_BUILT_DIR)