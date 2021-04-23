#! MeetiX OS Builder Script
#!
#! This Makefile is responsible to build all the source targets (kernel and
#! applications), pack a bootable image then run the QEMU emulator for the selected
#! architecture

#
# -- -- -- -- -- -- -- -- -- -- Configuration Variables -- -- -- -- -- -- -- -- -- --
#

ARCH       ?= x86_64
BUILD_MODE ?= debug
VIRT_ACCEL ?= -enable-kvm
SMP_CORES  ?= 1 # TODO support SMP in the kernel
V          ?= @ # disable print of executed command, remove to print commands

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT  ?= $(shell pwd)
SRC_DIR    ?= $(REPO_ROOT)/src
BUILD_DIR  ?= $(REPO_ROOT)/target/$(ARCH)

KERNEL        ?= $(SRC_DIR)/kernel
KERNEL_HHL    ?= $(KERNEL)/hh_loader
USERLAND      ?= $(SRC_DIR)/userland
USERLAND_BINS ?= $(USERLAND)/bins

ARCH_CONF_PATH ?= targets/$(ARCH)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Rust Tools -- -- -- -- -- -- -- -- -- -- -- -- --
#

RUSTC := $(shell which rustc)
CARGO := $(shell which cargo)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Rust Flags -- -- -- -- -- -- -- -- -- -- -- -- --
#

RUSTC_FLAGS ?= -Zmacro-backtrace
CARGO_FLAGS ?= --color=always

ifeq ($(BUILD_MODE),release)
    CARGO_FLAGS += --release
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- LLVM Tools -- -- -- -- -- -- -- -- -- -- -- -- --
#

TOOLCHAIN_ROOT := $(shell $(RUSTC) --print sysroot)
LLVM_OBJCOPY   := $(shell find $(TOOLCHAIN_ROOT) -name llvm-objcopy)
LLVM_STRIP     := $(shell find $(TOOLCHAIN_ROOT) -name llvm-strip)
LLVM_SIZE      := $(shell find $(TOOLCHAIN_ROOT) -name llvm-size)
LLVM_LD        := $(shell find $(TOOLCHAIN_ROOT) -name rust-lld)

#
# -- -- -- -- -- -- -- -- -- -- -- LLVM Tools Flags -- -- -- -- -- -- -- -- -- -- -- --
#

OBJCOPY_FLAGS ?= -O elf32-i386

ifeq ($(BUILD_MODE),release)
    OBJCOPY_FLAGS += -S
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

run: build
	$(V) echo "Running QEMU $(VIRT_ACCEL)"

build:
	$(V) echo "Building for $(ARCH) in $(BUILD_MODE) mode"

	$(V) echo "Building Kernel Core..."
	$(V) CARGO_TARGET_DIR="$(BUILD_DIR)"              \
             $(CARGO) build $(CARGO_FLAGS)            \
                 --manifest-path $(KERNEL)/Cargo.toml \
                 --target $(KERNEL)/$(ARCH_CONF_PATH)/kernel.json

	$(V) echo "Building Kernel Loader..."
	$(V) CARGO_TARGET_DIR="$(BUILD_DIR)"                  \
             $(CARGO) build $(CARGO_FLAGS)                \
                 --manifest-path $(KERNEL_HHL)/Cargo.toml \
                 --target $(KERNEL_HHL)/$(ARCH_CONF_PATH)/hh_loader.json

	$(V) echo "Building Userland Binaries..."
	$(V) for BIN_PRJ in $(USERLAND_BINS)/*/Cargo.toml; do                   \
              CARGO_TARGET_DIR="$(BUILD_DIR)"                               \
                  $(CARGO) build $(CARGO_FLAGS)                             \
                      --manifest-path $${BIN_PRJ}                           \
                      --target $(USERLAND)/$(ARCH_CONF_PATH)/userland.json; \
         done

ifeq ($(BUILD_MODE), release)
	$(V) echo "Stripping kernel core and kernel loader..."
	$(V) $(LLVM_STRIP) --strip-debug                  \
             $(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel \
             $(BUILD_DIR)/hh_loader/$(BUILD_MODE)/hh_loader
endif

ifeq ($(ARCH), x86_64)
    # GRUB doesn't support ELF64 files, need pack the kernel into ELF32 file
	$(V) echo "Copy ELF64 kernel into ELF32 executable..."
	$(V) $(LLVM_OBJCOPY) $(OBJCOPY_FLAGS) $(BUILD_DIR)/hh_loader/$(BUILD_MODE)/hh_loader
endif

clean:
	$(V) echo "Cleaning $(BUILD_MODE) build of $(ARCH)"
	$(V) rm -rf $(BUILD_DIR)/{kernel,hh_loader,userland}/debug

clean_all:
	$(V) echo "Cleaning All"
	$(V) rm -rf $(BUILD_DIR)
