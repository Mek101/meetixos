#! Userland Binaries Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the userland binaries

include ../../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT    ?= $(realpath ../..)
BUILD_DIR    ?= $(REPO_ROOT)/$(BUILD_PREFIX)
BUILD_TARGET ?= $(realpath ../$(TARGET_PREFIX)/userland.json)
DIST_DIR     ?= $(REPO_ROOT)/$(DIST_SYSROOT_PREFIX)/Bins

#
# -- -- -- -- -- -- -- -- -- -- -- Application Projects -- -- -- -- -- -- -- -- -- -- --
#

BIN_PROJECTS := $(wildcard */Cargo.toml)
BINS         := $(dir $(BIN_PROJECTS))
BINS         := $(BINS:%/=%) # removes the last slash from the name
BUILT_BINS   := $(addprefix $(BUILD_DIR)/userland/$(BUILD_MODE)/,$(BINS))

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

install: $(BUILT_BINS)
	$(V) echo "- Installing $(<)"
	$(V) $(RSYNC) -a $< $(DIST_DIR)

build: $(BUILT_BINS)
	$(V) echo "- Userland Bins Build completed..."

$(BUILT_BINS): $(BIN_PROJECTS)
	$(V) echo "- Compiling $(dir $<)..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"      \
         CARGO_TARGET_DIR="$(BUILD_DIR)" \
             $(CARGO) build $(CARGO_FLAGS) --manifest-path $< --target $(BUILD_TARGET)
