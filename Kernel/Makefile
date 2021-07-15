#! OldKernel Builder Script
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
# -- -- -- -- -- -- -- -- -- -- -- -- OldKernel Paths -- -- -- -- -- -- -- -- -- -- -- --
#

KERNEL_BUILD_DIR ?= $(BUILD_DIR)/kernel/$(BUILD_MODE)

BUILT_KERNEL       ?= $(KERNEL_BUILD_DIR)/kernel
KERNEL_SYMBOLS     ?= $(KERNEL_BUILD_DIR)/kernel_symbols.syms

FINAL_KERNEL ?= $(KERNEL_BUILD_DIR)/mx_kernel

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Sources -- -- -- ----  -- -- -- -- -- -- -- --
#

KERNEL_SOURCES ?= $(shell find src -name *.rs -o -name *.S)
UKLIBS_SOURCES ?= $(shell find $(realpath ../UKLibs) -name *.rs)
SOURCES        ?= $(KERNEL_SOURCES) $(UKLIBS_SOURCES)

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- --
#

install: build
	$(V) $(RSYNC) -a $(FINAL_KERNEL) $(DIST_DIR)

build: $(FINAL_KERNEL)
	$(V) echo "- Kernel Build Completed..."

$(FINAL_KERNEL): $(KERNEL_SYMBOLS) $(BUILT_KERNEL)
	$(V) echo "- Updating Kernel Executable With Symbols..."
	$(V) $(OBJCOPY) --update-section .symbols="$(KERNEL_SYMBOLS)" $(BUILT_KERNEL)
	$(V) $(RM) -f $@ && $(MV) -f $(BUILT_KERNEL) $@

$(KERNEL_SYMBOLS): $(BUILT_KERNEL)
	$(V) echo "- Writing Kernel Symbols..."
	$(V) $(LLVM_NM) -n $< | awk '{ if ($$2 == "T") printf("%s %s\n", $$1, $$3); }' | uniq | $(RFILT) >$@

$(BUILT_KERNEL): $(KERNEL_TARGET) $(SOURCES)
	$(V) echo "- Building Kernel Without Symbols..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"    \
         CARGO_TARGET_DIR=$(BUILD_DIR) \
         $(CARGO) build $(CARGO_FLAGS) --target $(KERNEL_TARGET)

clean:
	$(V) echo "- Cleaning $(ARCH)/$(BUILD_MODE) Kernel..."
	$(V) $(RM) -rf $(KERNEL_BUILD_DIR)