#! Userland Applications Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the userland applications

include ../../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT    ?= $(realpath ../..)
BUILD_DIR    ?= $(REPO_ROOT)/$(BUILD_PREFIX)
BUILD_TARGET ?= $(realpath ../$(TARGET_PREFIX)/userland.json)
DIST_DIR     ?= $(REPO_ROOT)/$(DIST_SYSROOT_PREFIX)/Apps

#
# -- -- -- -- -- -- -- -- -- -- -- Application Projects -- -- -- -- -- -- -- -- -- -- --
#

APP_PROJECTS := $(wildcard */Cargo.toml)
APPS         := $(dir $(APP_PROJECTS))
APPS         := $(APPS:%/=%) # removes the last slash from the name
BUILT_APPS   := $(addprefix $(BUILD_DIR)/userland/$(BUILD_MODE)/,$(APPS))

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- -- --
#

install: $(BUILT_APPS)
	$(V) echo "- Installing $(<)"
	$(V) $(RSYNC) -a $< $(DIST_DIR)

build: $(BUILT_APPS)
	$(V) echo "- Userland Apps Build completed..."

$(BUILT_APPS): $(APP_PROJECTS)
	$(V) echo "- Compiling $(dir $<)..."
	$(V) RUSTFLAGS="$(RUSTC_FLAGS)"      \
         CARGO_TARGET_DIR="$(BUILD_DIR)" \
             $(CARGO) build $(CARGO_FLAGS) --manifest-path $< --target $(BUILD_TARGET)
