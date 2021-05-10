#! Makefile Configuration Variables
#!
#! This Makefile is responsible to build/install/clean/doc the userland applications

#
# -- -- -- -- -- -- -- -- -- -- Configuration Variables -- -- -- -- -- -- -- -- -- --
#

ARCH       ?= x86_64
BUILD_MODE ?= debug
VIRT_ACCEL ?= -enable-kvm
SMP_CORES  ?= 1 # TODO support SMP in the kernel
V          ?= @ # disable print of executed command, remove to print commands
OFFLINE    ?= false

#
# -- -- -- -- -- -- -- -- -- -- -- Build Prefixes -- -- -- -- -- -- -- -- -- -- -- --
#

BUILD_PREFIX        ?= target/$(ARCH)
TARGET_PREFIX       ?= targets/$(ARCH)
DIST_SYSROOT_PREFIX ?= $(BUILD_PREFIX)/sysroot/$(BUILD_MODE)

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

ifeq ($(OFFLINE),true)
    CARGO_FLAGS += --offline
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- LLVM Tools -- -- -- -- -- -- -- -- -- -- -- -- --
#

TOOLCHAIN_ROOT := $(shell $(RUSTC) --print sysroot)
LLVM_OBJCOPY   := $(shell find $(TOOLCHAIN_ROOT) -name llvm-objcopy)
LLVM_STRIP     := $(shell find $(TOOLCHAIN_ROOT) -name llvm-strip)
LLVM_LD        := $(shell find $(TOOLCHAIN_ROOT) -name rust-lld)

#
# -- -- -- -- -- -- -- -- -- -- -- LLVM Tools Flags -- -- -- -- -- -- -- -- -- -- -- --
#

OBJCOPY_FLAGS ?= -O elf32-i386

ifeq ($(BUILD_MODE),release)
    OBJCOPY_FLAGS += -S
endif

#
# -- -- -- -- -- -- -- -- -- -- -- Command Line Tools -- -- -- -- -- -- -- -- -- -- -- --
#

RSYNC ?= $(shell which rsync)
RM    ?= $(shell which rm)
CP    ?= $(shell which cp)
MKDIR ?= $(shell which mkdir)

ifeq ($(ARCH), x86_64)
    MAKE_RESQUE ?= $(shell which grub-mkrescue)
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- QEMU Tools -- -- -- -- -- -- -- -- -- -- -- -- -- --
#

QEMU ?= qemu-system-$(ARCH)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Make Arguments -- -- -- -- -- -- -- -- -- -- -- -- --
#

MAKE_ARGS = --no-print-directory