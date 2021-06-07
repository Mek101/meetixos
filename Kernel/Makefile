#! Kernel Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the Kernel

include ../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT  ?= $(realpath ..)
BUILD_DIR  ?= $(REPO_ROOT)/$(BUILD_PREFIX)
KRN_TARGET ?= $(realpath $(TARGET_PREFIX)/kernel.json)
HHL_TARGET ?= $(realpath HHLoader/$(TARGET_PREFIX)/hh_loader.json)
DIST_DIR   ?= ../$(DIST_SYSROOT_PREFIX)/MeetiX

#
# -- -- -- -- -- -- -- -- -- -- -- -- Kernel Paths -- -- -- -- -- -- -- -- -- -- -- --
#

KRN_BUILT_DIR ?= $(BUILD_DIR)/kernel/$(BUILD_MODE)
HHL_BUILT_DIR ?= $(BUILD_DIR)/hh_loader/$(BUILD_MODE)

BUILT_KRN    ?= $(KRN_BUILT_DIR)/kernel
BUILT_HHL    ?= $(HHL_BUILT_DIR)/hh_loader
KRN_CORE_MOD ?= $(KRN_BUILT_DIR)/kernel_bin.rs
KRN_CORE_MAP ?= $(KRN_BUILT_DIR)/kernel_symbols.map

MX_KERNEL ?= $(HHL_BUILT_DIR)/mx_kernel

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Sources -- -- -- ----  -- -- -- -- -- -- -- --
#

KRN_SOURCES ?= $(shell find src -name *.rs)
HHL_SOURCES ?= $(shell find HHLoader/src -name *.rs -o -name *.S)
SHF_SOURCES ?= $(shell find SharedCode/src -name *.rs)

HHL_LINK_SRC ?= $(realpath HHLoader/$(TARGET_PREFIX)/linker.ld)
KRN_LINK_SRC ?= $(realpath $(TARGET_PREFIX)/linker.ld)

UK_LIBS_SRC ?= $(shell find ../UKLibs -name *.rs)

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- --
#

install: build
	$(V) $(RSYNC) -a $(MX_KERNEL) $(DIST_DIR)

build: $(MX_KERNEL)
	$(V) echo "- Kernel Build completed..."

$(MX_KERNEL): $(BUILT_HHL)
	$(V) echo "- Linking '$(basename $(MX_KERNEL))'"
	$(V) $(CP) -f $(BUILT_HHL) $(MX_KERNEL)

$(BUILT_HHL): $(KRN_CORE_MOD) $(KRN_CORE_MAP) $(HHL_SOURCES) $(HHL_LINK_SRC)
	$(V) echo "- Building Kernel Loader..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"                        \
         KERNEL_BIN=$(KRN_CORE_MOD)                        \
         KERNEL_SYM=$(KRN_CORE_MAP)                        \
         CARGO_TARGET_DIR="$(BUILD_DIR)"                   \
             $(CARGO) build $(CARGO_FLAGS)                 \
                      --manifest-path HHLoader/Cargo.toml \
                      --target $(HHL_TARGET)

$(KRN_CORE_MOD): $(SHF_SOURCES) $(BUILT_KRN)
	$(V) echo "- Writing Kernel Core Module..."
	$(V) echo "const KERNEL_SIZE: usize = $$(stat --format %s $(BUILT_KRN));" >$(KRN_CORE_MOD)
	$(V) echo >>$(KRN_CORE_MOD)
	$(V) echo "#[link_section = \".kernel_core\"]" >>$(KRN_CORE_MOD)
	$(V) echo "static KERNEL_BYTES: [u8; KERNEL_SIZE] = *include_bytes!(r\"$(BUILT_KRN)\");" >>$(KRN_CORE_MOD)
	$(V) echo >>$(KRN_CORE_MOD)

$(KRN_CORE_MAP): $(SHF_SOURCES) $(BUILT_KRN)
	$(V) echo "- Writing Kernel Core Symbols Map..."
	$(V) $(LLVM_NM) -n $(BUILT_KRN) | awk '{ if ($$2 != "a") printf("%s %s\n", $$1, $$3); }' | uniq >$(KRN_CORE_MAP)

$(BUILT_KRN): $(SHF_SOURCES) $(KRN_SOURCES) $(KRN_LINK_SRC) $(UK_LIBS_SRC)
	$(V) echo "- Building Kernel Core..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"      \
         CARGO_TARGET_DIR="$(BUILD_DIR)" \
             $(CARGO) build $(CARGO_FLAGS) --target $(KRN_TARGET)

clean:
	$(V) echo "- Cleaning $(ARCH)/$(BUILD_MODE) Kernel..."
	$(V) $(RM) -rf $(KRN_BUILT_DIR) $(HHL_BUILT_DIR)