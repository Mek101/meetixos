#! Kernel Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the Kernel

include ../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT     ?= $(realpath ..)
BUILD_DIR     ?= $(REPO_ROOT)/$(BUILD_PREFIX)
KERNEL_TARGET ?= $(realpath $(TARGET_PREFIX)/kernel.json)
DIST_DIR      ?= $(realpath ../$(DIST_SYSROOT_PREFIX)/MeetiX)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Kernel Paths -- -- -- -- -- -- -- -- -- -- -- --
#

KERNEL_BUILD_DIR ?= $(BUILD_DIR)/kernel/$(BUILD_MODE)

BUILT_KERNEL       ?= $(KERNEL_BUILD_DIR)/kernel
BUILT_PRE_KERNEL   ?= $(KERNEL_BUILD_DIR)/pre_kernel
KERNEL_SYMBOLS     ?= $(KERNEL_BUILD_DIR)/kernel_symbols.rs
PRE_KERNEL_SYMBOLS ?= $(KERNEL_BUILD_DIR)/empty_kernel_symbols.rs

FINAL_KERNEL ?= $(KERNEL_BUILD_DIR)/mx_kernel

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Sources -- -- -- ----  -- -- -- -- -- -- -- --
#

KERNEL_SOURCES ?= $(shell find src -name *.rs -a -name *.S)
UKLIBS_SOURCES ?= $(shell find $(realpath ../UKLibs) -name *.rs)
SOURCES        ?= $(KERNEL_SOURCES) $(UKLIBS_SOURCES)

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- --
#

install: build
	$(V) $(RSYNC) -a $(FINAL_KERNEL) $(DIST_DIR)

build: $(FINAL_KERNEL)
	$(V) echo "- Kernel Build Completed..."

$(FINAL_KERNEL): $(BUILT_KERNEL)
	$(V) echo "- Linking '$(basename $(FINAL_KERNEL))'"
	$(V) $(MV) $(BUILT_KERNEL) $(FINAL_KERNEL)

$(BUILT_KERNEL): $(KERNEL_SYMBOLS)
	$(V) echo "- Updating Kernel Executable With Symbols..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"       \
         CARGO_TARGET_DIR=$(BUILD_DIR)    \
         KERNEL_SYMBOLS=$(KERNEL_SYMBOLS) \
         $(CARGO) build $(CARGO_FLAGS) --target $(KERNEL_TARGET)

$(KERNEL_SYMBOLS): $(BUILT_PRE_KERNEL)
	$(V) echo "- Writing Kernel Symbols..."
	$(V) $(LLVM_NM) -n $(BUILT_PRE_KERNEL) | awk '{ if ($$2 != "a") printf("%s %s\n", $$1, $$3); }' | uniq >$(KERNEL_SYMBOLS)

$(BUILT_PRE_KERNEL): $(SOURCES) $(PRE_KERNEL_SYMBOLS) $(KERNEL_TARGET)
	$(V) echo "- Building Kernel Without Symbols..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"           \
         CARGO_TARGET_DIR=$(BUILD_DIR)        \
         KERNEL_SYMBOLS=$(PRE_KERNEL_SYMBOLS) \
         $(CARGO) build $(CARGO_FLAGS) --target $(KERNEL_TARGET)
	$(V) $(MV) $(BUILT_KERNEL) $(BUILT_PRE_KERNEL)

$(PRE_KERNEL_SYMBOLS):
	$(V) echo "- Writing Empty Kernel Symbols..."
	$(V) $(MKDIR) -p $(dir $(PRE_KERNEL_SYMBOLS))
	$(V) echo " " >$@

clean:
	$(V) echo "- Cleaning $(ARCH)/$(BUILD_MODE) Kernel..."
	$$(V) $(RM) -rf $(KERNEL_BUILD_DIR)