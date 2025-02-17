#! Makefile Configuration Variables
#!
#! This Makefile is responsible to build/install/clean/doc the Userland applications

#
# -- -- -- -- -- -- -- -- -- -- Configuration Variables -- -- -- -- -- -- -- -- -- --
#

ARCH       ?= x86_64
BUILD_MODE ?= debug
VIRT_ACCEL ?= #-enable-kvm
SMP_CORES  ?= 1 # TODO support SMP in the Kernel
V          ?= @ # disable print of executed command, remove to print commands
OFFLINE    ?= false

#
# -- -- -- -- -- -- -- -- -- -- -- Build Prefixes -- -- -- -- -- -- -- -- -- -- -- --
#

BUILD_PREFIX        ?= Build/$(ARCH)
TARGET_PREFIX       ?= Targets/$(ARCH)
DIST_SYSROOT_PREFIX ?= $(BUILD_PREFIX)/Root/$(BUILD_MODE)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Rust Tools -- -- -- -- -- -- -- -- -- -- -- -- --
#

RUSTC ?= $(shell which rustc)
CARGO ?= $(shell which cargo)
R_GDB ?= $(shell which rust-gdb)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Rust Flags -- -- -- -- -- -- -- -- -- -- -- -- --
#

RUSTC_FLAGS ?= -Zmacro-backtrace -Cforce-frame-pointers=y
CARGO_FLAGS ?= --color=always

ifeq ($(BUILD_MODE),release)
    CARGO_FLAGS += --release
endif

ifeq ($(OFFLINE),true)
    CARGO_FLAGS += --offline
endif

ifeq ($(V),)
    CARGO_FLAGS += --verbose
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- LLVM Tools -- -- -- -- -- -- -- -- -- -- -- -- --
#

TOOLCHAIN_ROOT ?= $(shell $(RUSTC) --print sysroot)
LLVM_STRIP     ?= $(shell find $(TOOLCHAIN_ROOT) -name llvm-strip)
LLVM_NM        ?= $(shell find $(TOOLCHAIN_ROOT) -name llvm-nm)
LLVM_LD        ?= $(shell find $(TOOLCHAIN_ROOT) -name rust-lld)


#
# -- -- -- -- -- -- -- -- -- -- -- Command Line Tools -- -- -- -- -- -- -- -- -- -- -- --
#

RSYNC   ?= $(shell which rsync)
RM      ?= $(shell which rm)
CP      ?= $(shell which cp)
MV      ?= $(shell which mv)
MKDIR   ?= $(shell which mkdir)
RFILT   ?= $(shell which rustfilt)
OBJCOPY ?= $(shell which objcopy)

ifeq ($(ARCH), x86_64)
    MAKE_RESCUE ?= $(shell which grub-mkrescue)
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- QEMU Tools -- -- -- -- -- -- -- -- -- -- -- -- -- --
#

QEMU          ?= qemu-system-$(ARCH)
QEMU_ARGS     ?= -m 64M -serial stdio
QEMU_GDB_ARGS ?= -m 64M

ifeq ($(ARCH), x86_64)
    QEMU_ARGS     += -cpu IvyBridge
    QEMU_GDB_ARGS += -cpu IvyBridge
endif

#
# -- -- -- -- -- -- -- -- -- -- -- -- Make Arguments -- -- -- -- -- -- -- -- -- -- -- -- --
#

MAKE_ARGS = --no-print-directory