#! Userland Builder Script
#!
#! This Makefile is responsible to build/install/clean/doc the userland binaries

include ../config.mk

#
# -- -- -- -- -- -- -- -- -- -- -- Project Variables -- -- -- -- -- -- -- -- -- -- --
#

REPO_ROOT    ?= $(realpath ../)
BUILD_DIR    ?= $(REPO_ROOT)/$(BUILD_PREFIX)
BUILD_TARGET ?= $(realpath $(TARGET_PREFIX)/userland.json)

#
# -- -- -- -- -- -- -- -- -- -- -- -- Make Arguments -- -- -- -- -- -- -- -- -- -- --
#

MAKE_ARGS += REPO_ROOT="$(REPO_ROOT)" \
             BUILD_DIR="$(BUILD_DIR)" \
             BUILD_TARGET="$(BUILD_TARGET)"

#
# -- -- -- -- -- -- -- -- -- -- -- -- -- Make Targets -- -- -- -- -- -- -- -- -- -- --
#

install: build
	$(V) $(MAKE) $(MAKE_ARGS) -C apps install
	$(V) $(MAKE) $(MAKE_ARGS) -C bins install

build:
	$(V) $(MAKE) $(MAKE_ARGS) -C apps build
	$(V) $(MAKE) $(MAKE_ARGS) -C bins build

clean:
	$(V) $(RM) -rf $(BUILD_DIR)/userland/$(BUILD_MODE)