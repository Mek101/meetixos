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
VIRT_ACCEL ?= #-enable-kvm
SMP_CORES  ?= 1 # TODO support SMP in the kernel
V          ?= @ # disable print of executed command, remove to print commands
OFFLINE    ?= false

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT    ?= $(shell pwd)
SRC_DIR      ?= $(REPO_ROOT)/src
BOOT_CFG_DIR ?= $(SRC_DIR)/boot/$(ARCH)
SYSROOT_DIR  ?= $(SRC_DIR)/sysroot
BUILD_DIR    ?= $(REPO_ROOT)/target/$(ARCH)

KERNEL        ?= $(SRC_DIR)/kernel
KERNEL_HHL    ?= $(KERNEL)/hh_loader
USERLAND      ?= $(SRC_DIR)/userland
USERLAND_APPS ?= $(USERLAND)/apps
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

ifeq ($(OFFLINE),true)
    CARGO_FLAGS += --offline
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
# -- -- -- -- -- -- -- -- -- -- -- QEMU Variables -- -- -- -- -- -- -- -- -- -- -- -- --
#

QEMU ?= qemu-system-$(ARCH)

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

run: image
	$(V) echo "- Running QEMU $(VIRT_ACCEL)"
	$(V) $(QEMU) $(VIRT_ACCEL) -m 64M -serial stdio -cdrom $(BUILD_DIR)/$(BUILD_MODE)/meetixos.iso

image: install
ifeq ($(ARCH),x86_64)
	$(V) echo "- Building GRUB bootable image..."
	$(V) grub-mkrescue -d /usr/lib/grub/i386-pc/        \
	         -o $(BUILD_DIR)/$(BUILD_MODE)/meetixos.iso \
	         $(BUILD_DIR)/sysroot/$(BUILD_MODE)
endif

install: build
	$(V) echo "- Copying sysroot to build dir..."
	$(V) mkdir -p $(BUILD_DIR)/sysroot/$(BUILD_MODE)
	$(V) cp -Rf $(SYSROOT_DIR)/* $(BUILD_DIR)/sysroot/$(BUILD_MODE)
	$(V) cp -Rf $(BOOT_CFG_DIR)/* $(BUILD_DIR)/sysroot/$(BUILD_MODE)

	$(V) echo "- Installing Kernel..."
	$(V) cp -f $(BUILD_DIR)/hh_loader/$(BUILD_MODE)/mx_kernel $(BUILD_DIR)/sysroot/$(BUILD_MODE)/MeetiX

	$(V) echo "- Installing Userland Apps..."
	$(V) for BIN in $(BUILD_DIR)/userland/$(BUILD_MODE)/*; do           \
             if [[ -f $${BIN} && -x $${BIN} ]]; then                    \
                 cp -f $${BIN} $(BUILD_DIR)/sysroot/$(BUILD_MODE)/Bins; \
             fi                                                         \
         done

	$(V) echo "- Installing Userland Binaries..."
	$(V) for BIN in $(BUILD_DIR)/userland/$(BUILD_MODE)/*; do           \
             if [[ -f $${BIN} && -x $${BIN} ]]; then                    \
                 cp -f $${BIN} $(BUILD_DIR)/sysroot/$(BUILD_MODE)/Bins; \
             fi                                                         \
         done

build:
	$(V) echo "- Building for $(ARCH) in $(BUILD_MODE) mode"

	$(V) echo "- Building Kernel Core..."
	$(V) CARGO_TARGET_DIR="$(BUILD_DIR)"              \
             $(CARGO) build $(CARGO_FLAGS)            \
                 --manifest-path $(KERNEL)/Cargo.toml \
                 --target $(KERNEL)/$(ARCH_CONF_PATH)/kernel.json

ifeq ($(BUILD_MODE), release)
	$(V) echo "- Stripping Kernel Core..."
	$(V) $(LLVM_STRIP) --strip-debug                  \
             $(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel
endif

	$(V) echo "- Writing Kernel Loader's Core module..."
	$(V) echo "const KERNEL_SIZE: usize = `stat --format %s $(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel`;" >$(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel_bin.rs
	$(V) echo "const KERNEL_BYTES: [u8; KERNEL_SIZE] = *include_bytes!(r\"$(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel\");" >>$(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel_bin.rs

	$(V) echo "- Building Kernel Loader..."
	$(V) KERNEL_BIN=$(BUILD_DIR)/kernel/$(BUILD_MODE)/kernel_bin.rs \
         CARGO_TARGET_DIR="$(BUILD_DIR)"                            \
             $(CARGO) build $(CARGO_FLAGS)                          \
                 --manifest-path $(KERNEL_HHL)/Cargo.toml           \
                 --target $(KERNEL_HHL)/$(ARCH_CONF_PATH)/hh_loader.json

	$(V) echo "- Building Userland Apps..."
	$(V) for APP_PRJ in $(USERLAND_APPS)/*/Cargo.toml; do                   \
              CARGO_TARGET_DIR="$(BUILD_DIR)"                               \
                  $(CARGO) build $(CARGO_FLAGS)                             \
                      --manifest-path $${APP_PRJ}                           \
                      --target $(USERLAND)/$(ARCH_CONF_PATH)/userland.json; \
         done

	$(V) echo "- Building Userland Binaries..."
	$(V) for BIN_PRJ in $(USERLAND_BINS)/*/Cargo.toml; do                   \
              CARGO_TARGET_DIR="$(BUILD_DIR)"                               \
                  $(CARGO) build $(CARGO_FLAGS)                             \
                      --manifest-path $${BIN_PRJ}                           \
                      --target $(USERLAND)/$(ARCH_CONF_PATH)/userland.json; \
         done

ifeq ($(ARCH), x86_64)
    # GRUB doesn't support ELF64 files, need pack the kernel into ELF32 file
	$(V) echo "- Copy ELF64 kernel into ELF32 executable..."
	$(V) $(LLVM_OBJCOPY) $(OBJCOPY_FLAGS) $(BUILD_DIR)/hh_loader/$(BUILD_MODE)/hh_loader
endif

	$(V) echo "- Creating mx_kernel"
	$(V) cp -f $(BUILD_DIR)/hh_loader/$(BUILD_MODE)/hh_loader $(BUILD_DIR)/hh_loader/$(BUILD_MODE)/mx_kernel


clean:
	$(V) echo "- Cleaning $(BUILD_MODE) build of $(ARCH)"
	$(V) rm -rf $(BUILD_DIR)/{kernel,hh_loader,userland}/debug

clean_all:
	$(V) echo "- Cleaning All"
	$(V) rm -rf $(BUILD_DIR)
